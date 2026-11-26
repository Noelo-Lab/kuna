//! Plan Carmel block-repeat address, execute, and terminal phases.

use super::{FETCH, Fetch, Framing, Issue, SLOT, paint};
use kuna_analysis::pass::ContextPaint;

#[derive(Clone, Copy)]
enum LoopCount {
    Immediate,
    AddressStage,
    RegisterStage,
}

#[derive(Clone, Copy)]
struct LoopSetup {
    delay: u64,
    end_words: u64,
    count: LoopCount,
}

impl LoopSetup {
    fn parse(issue: Issue, fetch: &Fetch) -> Option<Self> {
        let compact = match issue {
            Issue::Parallel if fetch[3..] == [0x2b, 0xec, 0] => Some(&fetch[..3]),
            Issue::SeqLeft => Some(&fetch[..3]),
            Issue::SeqRight => Some(&fetch[3..]),
            _ => None,
        };
        if let Some(slot) = compact.filter(|slot| slot[0] & 0x7f == 0x2a) {
            if (1..0x80).contains(&slot[1]) {
                return Some(Self {
                    delay: u64::from(slot[2] & 3),
                    end_words: u64::from((slot[2] >> 2) & !1),
                    count: LoopCount::Immediate,
                });
            }
            if issue == Issue::Parallel && slot[1] & 0xfe == 0xb0 {
                return Some(Self {
                    delay: u64::from(slot[2] & 7),
                    end_words: u64::from((u16::from_be_bytes([slot[1], slot[2]]) >> 3) & 63),
                    count: LoopCount::RegisterStage,
                });
            }
        }
        if !matches!(issue, Issue::WideMarked | Issue::WideUnmarked) || fetch[3] != 0xce {
            return None;
        }
        let right = u16::from_be_bytes([fetch[4], fetch[5]]);
        let left_low = fetch[0] & 0x3f;
        let delay = u64::from((right >> 11) & 7);
        let end_words = u64::from(u32::from_be_bytes([0, left_low & 0xf, fetch[1], fetch[2]]));
        if right & 0xc400 == 0x0400 {
            return Some(Self { delay, end_words: end_words & !1, count: LoopCount::Immediate });
        }
        if issue == Issue::WideMarked
            && ((left_low == 0x10 && matches!(right & !0x3800, 0x8000 | 0x8402))
                || (left_low == 0 && right == 0x8426))
        {
            return Some(Self { delay, end_words, count: LoopCount::AddressStage });
        }
        None
    }
}

#[derive(Default)]
pub(super) struct Plans(Vec<(u64, Issue, LoopSetup)>);

impl Plans {
    pub(super) fn note_fetch(&mut self, addr: u64, left: Issue, fetch: &Fetch) {
        if let Some(setup) = LoopSetup::parse(left, fetch) {
            self.0.push((addr, left, setup));
        }
        if left == Issue::SeqLeft {
            if let Some(setup) = LoopSetup::parse(Issue::SeqRight, fetch) {
                self.0.push((addr + SLOT, Issue::SeqRight, setup));
            }
        }
    }

    pub(super) fn paint(mut self, result: &mut Framing) {
        // A tail with several possible restart addresses takes the actual bank's
        // target. Coalesce plans in place; address phases themselves do not alias.
        self.0.sort_unstable_by_key(|(setup, _, plan)| setup + plan.end_words * SLOT);
        let mut tail_plan: Option<(u64, Option<u32>)> = None;
        let finish_tail = |paints: &mut Vec<ContextPaint>, (tail, restart)| {
            if let Some(restart) = restart {
                paint(paints, "hw_loop_restart", tail, restart);
            } else {
                paint(paints, "hw_loop_indirect", tail, 1);
            }
        };
        for (setup, issue, LoopSetup { delay, end_words, count }) in self.0 {
            let dynamic = !matches!(count, LoopCount::Immediate);
            let fallthrough = setup + u64::from(issue.size());
            let fetch = setup / FETCH * FETCH;
            let first = (dynamic || delay < 4).then(|| {
                if delay == 0 {
                    fallthrough
                } else {
                    fetch + (delay + u64::from(issue != Issue::SeqLeft)) * FETCH
                }
            });
            let tail = setup + end_words * SLOT;
            let mut following = [0; 5];
            let mut next = fallthrough;
            for at in &mut following {
                *at = next;
                next += if result.is_issue_start(next + SLOT) { SLOT } else { FETCH };
            }
            let start_at = if dynamic {
                matches!(count, LoopCount::RegisterStage).then_some(following[0])
            } else {
                first.map(|_| following[delay as usize])
            };
            let end_at = following[if dynamic { 3 } else { 1 }];
            for (field, value) in [
                ("hw_loop_setup", 1),
                ("hw_loop_first", (first.unwrap_or(0) / SLOT) as u32),
                ("hw_loop_last", (tail / SLOT) as u32),
                ("hw_loop_start_at", start_at.unwrap_or(0) as u32),
                ("hw_loop_end_at", end_at as u32),
            ] {
                paint(&mut result.specializations, field, setup, value);
            }
            for (at, phase) in [
                (start_at, if dynamic { "hw_loop_capture_rn0" } else { "hw_loop_start_publish" }),
                (Some(end_at), if dynamic { "hw_loop_activate" } else { "hw_loop_end_publish" }),
            ] {
                if let Some(at) = at.filter(|&at| result.is_issue_start(at)) {
                    paint(&mut result.specializations, "hw_loop_actions", at, 1);
                    paint(&mut result.specializations, phase, at, 1);
                }
            }
            let earliest = if dynamic { 5 } else if first.is_some() { 3.max(delay + 2) as usize } else { 3 };
            if !result.is_issue_start(tail) || tail < following[earliest - 1] {
                continue;
            }
            let Some(prepare) = tail.checked_sub(SLOT).filter(|&at| result.is_issue_start(at))
                .or_else(|| tail.checked_sub(FETCH).filter(|&at| result.is_issue_start(at)))
            else {
                continue;
            };
            let restart = first.map(|at| (at / SLOT) as u32);
            if let Some((last_tail, previous)) = &mut tail_plan {
                if *last_tail == tail {
                    if *previous != restart {
                        *previous = None;
                    }
                    continue;
                }
            }
            if let Some(previous) = tail_plan.replace((tail, restart)) {
                finish_tail(&mut result.specializations, previous);
            }
            let word = (tail / SLOT) as u32;
            paint(&mut result.specializations, "hw_loop_actions", prepare, 1);
            paint(&mut result.specializations, "hw_loop_actions", tail, 1);
            paint(&mut result.specializations, "hw_loop_prepare", prepare, 1);
            paint(&mut result.specializations, "hw_loop_prepare_end", prepare, word);
            paint(&mut result.specializations, "hw_loop_tail", tail, 1);
            paint(&mut result.specializations, "hw_loop_tail_end", tail, word);
        }
        if let Some(plan) = tail_plan {
            finish_tail(&mut result.specializations, plan);
        }
    }
}
