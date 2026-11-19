//! [`implied`] against every value of a one-byte operand: whenever it
//! answers, each value that satisfies the facts gives that answer.

use super::*;

const KINDS: [(u8, Order); 6] = [
    (EQ, Order::Either),
    (LT | GT, Order::Either),
    (LT, Order::Unsigned),
    (LT | EQ, Order::Unsigned),
    (LT, Order::Signed),
    (LT | EQ, Order::Signed),
];

struct Lcg(u64);

impl Lcg {
    fn next(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) % bound
    }
}

/// Whether the ordering of `a` relative to `b` under `order` is in `mask`.
fn holds(a: u64, b: u64, mask: u8, order: Order) -> bool {
    let ordering = if order == Order::Signed { (a as u8 as i8).cmp(&(b as u8 as i8)) } else { a.cmp(&b) };
    let bit = match ordering {
        std::cmp::Ordering::Less => LT,
        std::cmp::Ordering::Equal => EQ,
        std::cmp::Ordering::Greater => GT,
    };
    mask & bit != 0
}

fn random_fact(rng: &mut Lcg, bound: Option<u64>) -> Fact {
    let (truth, order) = KINDS[rng.next(6) as usize];
    let known = if rng.next(2) == 0 { truth } else { ALL & !truth };
    Fact { known, order, bound }
}

#[test]
fn a_bound_answer_holds_for_every_byte_the_facts_admit() {
    let mut rng = Lcg(1);
    let mut answered = 0;
    for _ in 0..20_000 {
        let edges = [0, 1, 0x7f, 0x80, 0xfe, 0xff];
        let pick = |rng: &mut Lcg| if rng.next(3) == 0 { edges[rng.next(6) as usize] } else { rng.next(256) };
        let facts: Vec<Fact> = (0..1 + rng.next(3)).map(|_| { let c = pick(&mut rng); random_fact(&mut rng, Some(c)) }).collect();
        let (truth, order) = KINDS[rng.next(6) as usize];
        let bound = pick(&mut rng);
        let Some(result) = implied(1, truth, order, Some(bound), &facts) else { continue };
        answered += 1;
        for x in 0..256u64 {
            if facts.iter().all(|f| holds(x, f.bound.unwrap(), f.known, f.order)) {
                assert_eq!(holds(x, bound, truth, order), result, "x={x:#x} bound={bound:#x} truth={truth}");
            }
        }
    }
    assert!(answered > 1000, "{answered}");
}

#[test]
fn a_pair_answer_holds_for_every_two_bytes_the_facts_admit() {
    let mut rng = Lcg(2);
    let mut answered = 0;
    for _ in 0..2_000 {
        let facts: Vec<Fact> = (0..1 + rng.next(2)).map(|_| random_fact(&mut rng, None)).collect();
        let (truth, order) = KINDS[rng.next(6) as usize];
        let Some(result) = implied(1, truth, order, None, &facts) else { continue };
        answered += 1;
        for a in 0..256u64 {
            for b in 0..256u64 {
                if facts.iter().all(|f| holds(a, b, f.known, f.order)) {
                    assert_eq!(holds(a, b, truth, order), result, "a={a:#x} b={b:#x} truth={truth}");
                }
            }
        }
    }
    assert!(answered > 100, "{answered}");
}
