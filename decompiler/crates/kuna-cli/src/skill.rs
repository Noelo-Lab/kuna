//! `kuna install-skill` — the agent skill (`skills/kuna/SKILL.md`), compiled into
//! the binary and written to where coding agents discover skills.
//!
//! Like `kuna docs`, the file is `include_str!`-ed so a downloaded release binary
//! is the whole distribution: no checkout, no network.  Installing from the binary
//! also keeps the skill's advice matched to the CLI version that runs it.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::output;

pub const BODY: &str = include_str!("../../../../skills/kuna/SKILL.md");

const USAGE: &str = "usage: kuna install-skill [--agent claude|codex|all] [--project | --dir DIR] [--force]\n\
                     \x20      kuna install-skill --print\n\
                     \n\
                     Install the kuna agent skill -- a guide to driving this CLI, embedded in the\n\
                     binary -- where coding agents load skills.  Nothing is downloaded.\n\
                     \n\
                     --agent A     claude: $CLAUDE_CONFIG_DIR/skills (default ~/.claude/skills)\n\
                     \x20             codex:  $CODEX_HOME/skills (default ~/.codex/skills)\n\
                     \x20             all:    both.  Without --agent, every agent whose config\n\
                     \x20             directory exists is chosen.\n\
                     --project     install into the current directory instead (.claude/skills,\n\
                     \x20             .agents/skills), to commit alongside a repo.\n\
                     --dir DIR     install into DIR/<skill-name>/ for any other agent.\n\
                     --force       replace an installed copy that differs (another kuna version,\n\
                     \x20             or local edits).  An identical copy is left alone.\n\
                     --print       write the skill to stdout and install nothing.\n\
                     \n\
                     Exit codes: 0 installed or already current, 1 a write failed or a differing\n\
                     copy was kept, 2 usage error or no agent found.";

#[derive(Clone, Copy, PartialEq)]
enum Agent {
    Claude,
    Codex,
}

impl Agent {
    const ALL: [Agent; 2] = [Agent::Claude, Agent::Codex];

    fn label(self) -> &'static str {
        match self {
            Agent::Claude => "claude",
            Agent::Codex => "codex",
        }
    }

    fn config_dir(self) -> Option<PathBuf> {
        let (var, default) = match self {
            Agent::Claude => ("CLAUDE_CONFIG_DIR", ".claude"),
            Agent::Codex => ("CODEX_HOME", ".codex"),
        };
        match std::env::var_os(var) {
            Some(v) if !v.is_empty() => Some(PathBuf::from(v)),
            _ => home_dir().map(|h| h.join(default)),
        }
    }

    fn project_dir(self) -> &'static str {
        match self {
            Agent::Claude => ".claude",
            Agent::Codex => ".agents",
        }
    }
}

pub fn run(argv: &[String]) -> i32 {
    let mut agents: Option<Vec<Agent>> = None;
    let mut project = false;
    let mut dir: Option<PathBuf> = None;
    let mut force = false;
    let mut print = false;

    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--agent" => {
                let Some(v) = crate::take_value(argv, &mut i, "--agent") else { return 2 };
                agents = Some(match v.as_str() {
                    "claude" => vec![Agent::Claude],
                    "codex" => vec![Agent::Codex],
                    "all" => Agent::ALL.to_vec(),
                    other => {
                        eprintln!("error: unknown agent {other:?} (expected claude, codex or all)");
                        return 2;
                    }
                });
            }
            "--dir" => {
                let Some(v) = crate::take_value(argv, &mut i, "--dir") else { return 2 };
                dir = Some(PathBuf::from(v));
            }
            "--project" => project = true,
            "--force" => force = true,
            "--print" => print = true,
            "-h" | "--help" => {
                eprintln!("{USAGE}");
                return 0;
            }
            s if s.starts_with('-') => {
                eprintln!("error: unknown option {s}");
                eprintln!("{USAGE}");
                return 2;
            }
            other => {
                eprintln!("error: unexpected argument {other:?}");
                return 2;
            }
        }
        i += 1;
    }

    if print {
        if agents.is_some() || project || dir.is_some() || force {
            eprintln!("error: --print installs nothing and takes no other flag");
            return 2;
        }
        return output::emit_with_status(BODY, 0);
    }
    if dir.is_some() && (agents.is_some() || project) {
        eprintln!("error: --dir names the skills directory itself; drop --agent/--project");
        return 2;
    }

    let name = skill_name();
    let targets: Vec<(&str, PathBuf)> = if let Some(d) = dir {
        vec![("dir", d)]
    } else {
        let chosen = agents.unwrap_or_else(|| {
            Agent::ALL.into_iter().filter(|a| a.config_dir().is_some_and(|d| d.is_dir())).collect()
        });
        if chosen.is_empty() {
            eprintln!(
                "error: found no agent config directory (~/.claude, ~/.codex); pass --agent \
                 claude|codex, or --dir DIR for another agent (--print shows the skill)"
            );
            return 2;
        }
        let mut targets = Vec::new();
        for a in chosen {
            let root = if project { Some(PathBuf::from(a.project_dir())) } else { a.config_dir() };
            match root {
                Some(r) => targets.push((a.label(), r.join("skills"))),
                None => {
                    eprintln!("error: {}: cannot locate the home directory", a.label());
                    return 2;
                }
            }
        }
        targets
    };

    let mut status = 0;
    let mut wrote = false;
    let mut report = String::new();
    for (label, skills) in targets {
        let path = skills.join(name).join("SKILL.md");
        match install(&path, force) {
            Ok(verdict) => {
                wrote |= verdict != "current";
                let _ = writeln!(report, "{verdict:<10} {label:<7} {}", path.display());
            }
            Err(msg) => {
                eprintln!("error: {label}: {}: {msg}", path.display());
                status = 1;
            }
        }
    }
    let status = output::emit_with_status(&report, status);
    if !report.is_empty() {
        if wrote {
            eprintln!("agents load skills at session start; begin a new session to use `{name}`");
        }
        if !on_path("kuna") {
            let exe = std::env::current_exe().map(|p| p.display().to_string());
            eprintln!(
                "note: `kuna` is not on PATH, and the skill invokes it by that name; put this \
                 binary ({}) on PATH",
                exe.as_deref().unwrap_or("kuna")
            );
        }
    }
    status
}

/// Write `BODY` to `path`, returning the verdict word for the report.
fn install(path: &Path, force: bool) -> Result<&'static str, String> {
    let verdict = match std::fs::read(path) {
        Ok(existing) if existing == BODY.as_bytes() => return Ok("current"),
        Ok(_) if !force => {
            return Err("an installed copy differs (another kuna version or local edits); \
                        rerun with --force to replace it"
                .into())
        }
        Ok(_) => "updated",
        Err(_) => "installed",
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, BODY).map_err(|e| e.to_string())?;
    Ok(verdict)
}

/// The frontmatter `name:`, which the Agent Skills layout requires the skill's
/// directory to match.
pub fn skill_name() -> &'static str {
    let mut lines = BODY.lines().map(str::trim);
    if lines.next() != Some("---") {
        return "kuna";
    }
    lines
        .take_while(|l| *l != "---")
        .find_map(|l| l.strip_prefix("name:"))
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .unwrap_or("kuna")
}

fn home_dir() -> Option<PathBuf> {
    ["HOME", "USERPROFILE"]
        .iter()
        .filter_map(std::env::var_os)
        .find(|v| !v.is_empty())
        .map(PathBuf::from)
}

fn on_path(program: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else { return false };
    std::env::split_paths(&path).any(|d| {
        d.join(program).is_file() || (cfg!(windows) && d.join(format!("{program}.exe")).is_file())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_skill_names_itself() {
        assert_eq!(BODY.lines().next().map(str::trim), Some("---"), "SKILL.md lost its frontmatter");
        assert_eq!(skill_name(), "kuna-decompiler");
    }
}
