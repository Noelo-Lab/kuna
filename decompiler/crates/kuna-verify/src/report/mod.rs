//! Verdicts and how they are printed: as text for a person, as JSON for a
//! script or an agent.

pub mod baseline;

use std::time::Duration;

use serde_json::{json, Value};

use crate::value::Halt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Agrees(u32),
    Differs { run: u32, of: u32 },
    Diverges(&'static str),
    Unknown(String),
    Skipped(String),
}

impl Verdict {
    pub fn label(&self) -> String {
        match self {
            Verdict::Agrees(n) => format!("AGREES({n})"),
            Verdict::Differs { .. } => "DIFFERS".to_string(),
            Verdict::Diverges(id) => format!("DIVERGES({id})"),
            Verdict::Unknown(s) | Verdict::Skipped(s) => s.clone(),
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Verdict::Agrees(_) => "AGREES",
            Verdict::Differs { .. } => "DIFFERS",
            Verdict::Diverges(_) => "DIVERGES",
            Verdict::Unknown(_) => "UNKNOWN",
            Verdict::Skipped(_) => "SKIPPED",
        }
    }

    pub fn from_halt(h: &Halt) -> Verdict {
        Verdict::Unknown(h.render())
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub function: String,
    pub verdict: Verdict,
    pub lines: Vec<String>,
    pub suspects: Vec<String>,
    pub elapsed: Duration,
    /// With `--concolic`: what the exploration did, z3 failing included.
    pub concolic: Option<crate::concolic::Summary>,
}

impl Report {
    pub fn render(&self) -> String {
        Row::from(self).render()
    }
}

/// A report reduced to what printing and aggregation need, so it can cross a
/// process boundary as one JSON line.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub function: String,
    pub class: String,
    pub label: String,
    /// `DIFFERS` only: the failing run and how many were planned.
    pub run: Option<(u32, u32)>,
    pub lines: Vec<String>,
    pub suspects: Vec<String>,
    pub ms: f64,
    /// `(queries, inputs found, solver state)` with `--concolic`.
    pub concolic: Option<(u32, u32, String)>,
}

impl From<&Report> for Row {
    fn from(r: &Report) -> Self {
        Row {
            function: r.function.clone(),
            class: r.verdict.class().to_string(),
            label: r.verdict.label(),
            run: match r.verdict {
                Verdict::Differs { run, of } => Some((run, of)),
                _ => None,
            },
            lines: r.lines.clone(),
            suspects: r.suspects.clone(),
            ms: r.elapsed.as_secs_f64() * 1e3,
            concolic: r.concolic.map(|s| (s.queries, s.models, s.solver.name().to_string())),
        }
    }
}

impl Row {
    pub fn render(&self) -> String {
        let mut s = match self.run {
            Some((run, of)) => format!("{}  DIFFERS  (run {} of {})\n", self.function, run + 1, of),
            None => format!("{}  {}\n", self.function, self.label),
        };
        for l in &self.lines {
            s.push_str("  ");
            s.push_str(l);
            s.push('\n');
        }
        for p in &self.suspects {
            s.push_str(&format!("  PROTOTYPE_SUSPECT({p})\n"));
        }
        s
    }

    /// The reason a verdict is `UNKNOWN` or `SKIPPED`, without addresses: what
    /// the summary counts.
    pub fn reason(&self) -> Option<String> {
        matches!(self.class.as_str(), "UNKNOWN" | "SKIPPED").then(|| {
            self.label.split(':').next().unwrap_or(&self.label).trim_end_matches(')').to_string()
        })
    }

    pub fn to_json(&self) -> Value {
        let mut v = json!({
            "function": self.function,
            "verdict": self.class,
            "label": self.label,
            "run": self.run.map(|(r, of)| json!({"run": r + 1, "of": of})),
            "lines": self.lines,
            "suspects": self.suspects,
            "ms": self.ms,
        });
        if let Some((q, m, s)) = &self.concolic {
            v["z3"] = json!({"queries": q, "models": m, "solver": s});
        }
        v
    }

    pub fn from_json(v: &Value) -> Option<Row> {
        let strings = |k: &str| -> Option<Vec<String>> {
            v.get(k)?.as_array()?.iter().map(|x| x.as_str().map(str::to_string)).collect()
        };
        let run = match v.get("run") {
            Some(Value::Object(o)) => Some((o.get("run")?.as_u64()? as u32 - 1, o.get("of")?.as_u64()? as u32)),
            _ => None,
        };
        Some(Row {
            function: v.get("function")?.as_str()?.to_string(),
            class: v.get("verdict")?.as_str()?.to_string(),
            label: v.get("label")?.as_str()?.to_string(),
            run,
            lines: strings("lines")?,
            suspects: strings("suspects")?,
            ms: v.get("ms")?.as_f64()?,
            concolic: v.get("z3").and_then(|c| {
                Some((c.get("queries")?.as_u64()? as u32, c.get("models")?.as_u64()? as u32, c.get("solver")?.as_str()?.to_string()))
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_round_trip_through_json() {
        let r = Report {
            function: "ins16".into(),
            verdict: Verdict::Differs { run: 2, of: 256 },
            lines: vec!["machine  r1:r0 = 0x1".into()],
            suspects: vec!["reads r2".into()],
            elapsed: Duration::from_micros(1500),
            concolic: Some(crate::concolic::Summary { queries: 3, models: 1, solver: crate::concolic::SolverState::Ran }),
        };
        let row = Row::from(&r);
        assert_eq!(Row::from_json(&row.to_json()), Some(row.clone()));
        assert!(row.render().starts_with("ins16  DIFFERS  (run 3 of 256)\n"));
        let skipped = Row { class: "SKIPPED".into(), label: "SKIPPED(memory: store at 0x10)".into(), ..row };
        assert_eq!(skipped.reason().as_deref(), Some("SKIPPED(memory"));
    }
}
