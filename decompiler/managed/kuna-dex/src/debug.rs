//! `debug_info_item`: the line-number and local-variable state machine.

use crate::dex::NO_INDEX;
use crate::reader::Reader;
use crate::Result;

/// A named register over `[start, end)` (code units).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Local {
    pub reg: u32,
    pub start: u32,
    pub end: u32,
    pub name: u32,
    pub ty: u32,
    pub sig: u32,
}

#[derive(Clone, Debug, Default)]
pub struct DebugInfo {
    /// `(address, line)` in address order.
    pub lines: Vec<(u32, u32)>,
    pub locals: Vec<Local>,
    /// One name per declared parameter (not `this`), `NO_INDEX` when unnamed.
    pub params: Vec<u32>,
}

/// Runs the state machine; `params` are the method's registers for its
/// parameters as `(register, type)` (with `this` first when present, as
/// `(register, NO_INDEX)`), which start as locals at address 0.
pub fn parse(
    data: &[u8],
    off: u32,
    code_units: u32,
    params: &[(u32, u32)],
    this_name: u32,
) -> Result<DebugInfo> {
    let mut r = Reader::at(data, off as usize)?;
    let mut line = r.uleb()?;
    let count = r.uleb()?;
    if count as usize > r.remaining() {
        return Err(crate::err(format!("debug info at {off:#x} is out of bounds")));
    }
    let mut info = DebugInfo::default();
    for _ in 0..count {
        info.params.push(r.uleb_p1()?);
    }
    let mut open: Vec<Option<Local>> = Vec::new();
    let mut last: Vec<Option<Local>> = Vec::new();
    let slot = |v: &mut Vec<Option<Local>>, reg: u32| -> usize {
        let i = reg as usize;
        if v.len() <= i {
            v.resize(i + 1, None);
        }
        i
    };
    let has_this = params.first().is_some_and(|p| p.1 == NO_INDEX);
    for (k, &(reg, ty)) in params.iter().enumerate() {
        let name = if has_this && k == 0 {
            this_name
        } else {
            info.params
                .get(k - usize::from(has_this))
                .copied()
                .unwrap_or(NO_INDEX)
        };
        if name == NO_INDEX || reg >= 65536 {
            continue;
        }
        let i = slot(&mut open, reg);
        open[i] = Some(Local {
            reg,
            start: 0,
            end: code_units,
            name,
            ty,
            sig: NO_INDEX,
        });
    }
    let mut addr = 0u32;
    let close = |open: &mut Vec<Option<Local>>,
                 last: &mut Vec<Option<Local>>,
                 out: &mut Vec<Local>,
                 reg: u32,
                 at: u32| {
        if let Some(Some(mut l)) = open.get_mut(reg as usize).map(Option::take) {
            l.end = at;
            if l.end > l.start {
                out.push(l.clone());
            }
            let i = reg as usize;
            if last.len() <= i {
                last.resize(i + 1, None);
            }
            last[i] = Some(l);
        }
    };
    loop {
        let op = r.u8()?;
        match op {
            0x00 => break,
            0x01 => addr = addr.saturating_add(r.uleb()?),
            0x02 => line = line.wrapping_add_signed(r.sleb()?),
            0x03 | 0x04 => {
                let reg = r.uleb()?;
                let name = r.uleb_p1()?;
                let ty = r.uleb_p1()?;
                let sig = if op == 0x04 { r.uleb_p1()? } else { NO_INDEX };
                if reg >= 65536 {
                    continue;
                }
                close(&mut open, &mut last, &mut info.locals, reg, addr);
                let i = slot(&mut open, reg);
                open[i] = Some(Local {
                    reg,
                    start: addr,
                    end: code_units,
                    name,
                    ty,
                    sig,
                });
            }
            0x05 => {
                let reg = r.uleb()?;
                close(&mut open, &mut last, &mut info.locals, reg, addr);
            }
            0x06 => {
                let reg = r.uleb()?;
                if reg >= 65536 {
                    continue;
                }
                close(&mut open, &mut last, &mut info.locals, reg, addr);
                if let Some(Some(prev)) = last.get(reg as usize) {
                    let mut l = prev.clone();
                    l.start = addr;
                    l.end = code_units;
                    let i = slot(&mut open, reg);
                    open[i] = Some(l);
                }
            }
            0x07 | 0x08 => {}
            0x09 => {
                r.uleb_p1()?;
            }
            _ => {
                let adjusted = u32::from(op - 0x0a);
                line = line.wrapping_add_signed(adjusted as i32 % 15 - 4);
                addr = addr.saturating_add(adjusted / 15);
                if info.lines.last().is_none_or(|&(a, _)| a < addr) {
                    info.lines.push((addr, line));
                } else if let Some(last) = info.lines.last_mut() {
                    last.1 = line;
                }
            }
        }
    }
    for reg in 0..open.len() as u32 {
        close(&mut open, &mut last, &mut info.locals, reg, code_units);
    }
    info.locals.retain(|l| l.start < code_units);
    for l in &mut info.locals {
        l.end = l.end.min(code_units);
    }
    info.locals.sort_by_key(|l| (l.start, l.reg));
    Ok(info)
}
