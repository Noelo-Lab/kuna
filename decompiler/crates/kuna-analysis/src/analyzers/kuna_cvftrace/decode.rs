//! Bounded decoder. An incomplete/unknown record contributes no facts.

#[derive(Debug, Default)]
pub struct Trace {
    pub routines: Vec<Routine>,
    pub lines: Vec<Line>,
}
#[derive(Debug)]
pub struct Routine {
    pub addr: u64,
    pub name: String,
}
#[derive(Debug)]
pub struct Line {
    pub func_addr: u64,
    pub start: u64,
    pub end: u64,
    pub file: std::sync::Arc<str>,
    pub line: u32,
}

struct Cursor<'a> {
    data: &'a [u8],
    at: usize,
}
impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(n)?;
        let value = self.data.get(self.at..end)?;
        self.at = end;
        Some(value)
    }
    fn uint(&mut self, n: usize) -> Option<u32> {
        Some(
            self.take(n)?
                .iter()
                .enumerate()
                .fold(0, |v, (i, b)| v | (u32::from(*b) << (i * 8))),
        )
    }
    fn text(&mut self, n: usize) -> Option<String> {
        if n > 4096 {
            return None;
        }
        let bytes = self.take(n)?;
        Some(
            bytes
                .iter()
                .map(|&b| char::from(b))
                .collect::<String>()
                .trim_end_matches('\0')
                .to_owned(),
        )
    }
}

/// Decode ordinary type-1 records. Incremental-link type-11 records are declined.
/// Limits bound allocation even for adversarial sections; arithmetic never wraps.
pub fn decode(data: &[u8]) -> Trace {
    let mut out = Trace::default();
    if data.len() > 16 * 1024 * 1024 {
        return out;
    }
    let mut c = Cursor { data, at: 0 };
    while c.at < data.len() {
        let start = c.at;
        let Some(kind) = c.uint(1) else {
            break;
        };
        match kind {
            0 => continue,
            10 => {
                if c.take(7).is_none() {
                    break;
                }
                continue;
            }
            1 | 11 => {}
            _ => break,
        }
        let Some(header) = data.get(start..start.saturating_add(16)) else {
            break;
        };
        let size = u32::from_le_bytes(header[12..16].try_into().unwrap()) as usize;
        if size < 16 {
            break;
        }
        let Some(end) = start.checked_add(size).filter(|&e| e <= data.len()) else {
            break;
        };
        if kind == 1 {
            if let Some(record) = record(&data[start..end]) {
                if out.lines.len() + record.lines.len() + out.routines.len() + record.routines.len()
                    > 262144
                {
                    break;
                }
                out.routines.extend(record.routines);
                out.lines.extend(record.lines);
            }
        }
        c.at = end;
    }
    out
}

fn record(data: &[u8]) -> Option<Trace> {
    let mut c = Cursor { data, at: 1 };
    let count = c.uint(3)? as usize;
    let low = c.uint(4)?;
    let high = low.checked_add(c.uint(4)?)?;
    c.take(4)?;
    let mut files = Vec::new();
    for _ in 0..count {
        let n = c.uint(2)? as usize;
        files.push(std::sync::Arc::<str>::from(c.text(n)?));
    }
    let mut out = Trace::default();
    let (mut pc, mut func, mut line, mut file_cursor) = (0u32, 0u32, 0i64, 0usize);
    let mut file: Option<&std::sync::Arc<str>> = None;
    let mut covered = low;
    while c.at < data.len() {
        if out.lines.len() + out.routines.len() >= 262144 {
            return None;
        }
        let op = c.uint(1)?;
        let previous = pc;
        let advance = match op {
            0 => continue,
            2 | 12 => {
                c.take(if op == 2 { 1 } else { 5 })?;
                let n = c.uint(2)? as usize;
                pc = c.uint(4)?;
                let name = c.text(n)?;
                if pc < low || pc >= high || name.is_empty() || name.chars().any(char::is_control) {
                    return None;
                }
                func = pc;
                out.routines.push(Routine {
                    addr: u64::from(pc),
                    name,
                });
                continue;
            }
            3 => {
                let index = file_cursor.checked_add(c.uint(4)? as usize)?;
                file = Some(files.get(index)?);
                file_cursor = index.checked_add(1)?;
                continue;
            }
            4..=6 => {
                let n = 1usize << (op - 4);
                let value = c.uint(n)?;
                let delta = match n {
                    1 => i64::from(value as i8),
                    2 => i64::from(value as i16),
                    _ => i64::from(value as i32),
                };
                line = line.checked_add(delta)?;
                continue;
            }
            7..=9 | 13 => {
                let n = match op {
                    7 => 1,
                    8 => 2,
                    _ => 4,
                };
                c.uint(n)?.checked_add(1)?
            }
            10 => {
                c.take(7)?;
                continue;
            }
            0x80..=0xbf => {
                line = line.checked_add(1)?;
                (op & 0x3f) + 1
            }
            0xc0..=0xff => {
                line = line.checked_add(i64::from(c.uint(1)? as i8))?;
                (op & 0x3f) + 1
            }
            _ => return None,
        };
        pc = pc.checked_add(advance)?;
        let start = previous.max(covered);
        covered = covered.max(pc.checked_add(1)?);
        if func != 0 && start >= low && start < high && line > 0 && line <= i64::from(u32::MAX) {
            if let Some(file) = file {
                out.lines.push(Line {
                    func_addr: u64::from(func),
                    start: u64::from(start),
                    end: u64::from(pc.min(high - 1)),
                    file: file.clone(),
                    line: line as u32,
                });
            }
        }
    }
    Some(out)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
