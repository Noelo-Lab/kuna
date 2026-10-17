//! (kuna `armframes`) The decisions one whole-image frame recovery reached, so a
//! `--jobs` worker rebuilds the parent's final Listing from them instead of
//! searching again. The record is a pure function of the image and options,
//! which both processes share.

use std::collections::BTreeSet;

use super::kuna_entrychecks::CheckedAif;

const HEADER: &str = "kuna-armframes 1";

/// The final root set, code-pointer and AIF entries, and retained prior
/// entries of one frame recovery; `recovered` is false when no frame validated.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArmFrameReplay {
    pub recovered: bool,
    pub frames: Vec<u64>,
    pub pointers: Vec<u64>,
    pub aif: Option<AifReplay>,
    pub retained: Vec<u64>,
}

/// The post-frame AIF entries the inventory keeps.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AifReplay {
    pub entries: Vec<u64>,
    pub pool_entries: Vec<u64>,
    pub pointer_entries: Vec<u64>,
}

impl AifReplay {
    pub(crate) fn of(checked: &CheckedAif) -> Self {
        AifReplay {
            entries: checked.entries.clone(),
            pool_entries: checked.pool_entries.clone(),
            pointer_entries: checked.pointer_entries.clone(),
        }
    }

    pub(crate) fn checked(&self) -> CheckedAif {
        CheckedAif {
            entries: self.entries.clone(),
            pool_entries: self.pool_entries.clone(),
            pointer_entries: self.pointer_entries.clone(),
            straight_line: BTreeSet::new(),
        }
    }
}

impl ArmFrameReplay {
    /// One `name addr...` line per list, in hex, after a versioned header.
    pub fn encode(&self) -> String {
        let mut out = format!("{HEADER}\nrecovered {}\n", u8::from(self.recovered));
        let mut line = |name: &str, list: &[u64]| {
            out.push_str(name);
            for at in list {
                out.push_str(&format!(" {at:x}"));
            }
            out.push('\n');
        };
        line("frames", &self.frames);
        line("pointers", &self.pointers);
        if let Some(aif) = &self.aif {
            line("aif", &aif.entries);
            line("pool", &aif.pool_entries);
            line("pointer_entries", &aif.pointer_entries);
        }
        line("retained", &self.retained);
        out
    }

    pub fn decode(text: &str) -> Result<Self, String> {
        let mut lines = text.lines();
        if lines.next() != Some(HEADER) {
            return Err("not a frame-recovery record".into());
        }
        let mut replay = ArmFrameReplay::default();
        let mut aif = AifReplay::default();
        let mut has_aif = false;
        for line in lines {
            let mut words = line.split(' ');
            let name = words.next().unwrap_or_default();
            if name == "recovered" {
                replay.recovered = words.next() == Some("1");
                continue;
            }
            let list = words
                .filter(|w| !w.is_empty())
                .map(|w| u64::from_str_radix(w, 16).map_err(|e| format!("bad address {w:?}: {e}")))
                .collect::<Result<Vec<u64>, String>>()?;
            match name {
                "frames" => replay.frames = list,
                "pointers" => replay.pointers = list,
                "aif" => (aif.entries, has_aif) = (list, true),
                "pool" => aif.pool_entries = list,
                "pointer_entries" => aif.pointer_entries = list,
                "retained" => replay.retained = list,
                other => return Err(format!("unknown frame-recovery line {other:?}")),
            }
        }
        replay.aif = has_aif.then_some(aif);
        Ok(replay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_round_trips() {
        let replay = ArmFrameReplay {
            recovered: true,
            frames: vec![0x8000, 0x8040],
            pointers: vec![],
            aif: Some(AifReplay { entries: vec![0x9000], pool_entries: vec![], pointer_entries: vec![0x9100] }),
            retained: vec![0x8100],
        };
        assert_eq!(ArmFrameReplay::decode(&replay.encode()), Ok(replay));
        let none = ArmFrameReplay::default();
        assert_eq!(ArmFrameReplay::decode(&none.encode()), Ok(none));
    }

    #[test]
    fn a_foreign_file_is_refused() {
        assert!(ArmFrameReplay::decode("frames 10\n").is_err());
    }
}
