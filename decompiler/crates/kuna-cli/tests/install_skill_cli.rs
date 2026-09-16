//! `kuna install-skill` — the agent skill shipped inside the binary.
//!
//! Every run isolates agent configuration in a scratch directory, so nothing
//! here touches the real agent configuration.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SKILL_DIR: &str = "kuna-decompiler";

fn repo_skill() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../skills/kuna/SKILL.md");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn scratch_home() -> PathBuf {
    let home = common::scratch_file("install-skill", "home");
    std::fs::create_dir_all(&home).unwrap();
    home
}

fn command_in(home: &Path, argv: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.arg("install-skill")
        .args(argv)
        .current_dir(home)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env_remove("CLAUDE_CONFIG_DIR")
        .env_remove("CODEX_HOME")
        .env_remove("OPENCODE_CONFIG_DIR")
        .env_remove("XDG_CONFIG_HOME");
    cmd
}

fn run_in(home: &Path, argv: &[&str]) -> Output {
    command_in(home, argv).output().expect("failed to spawn the kuna binary")
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn installed(skills: &Path) -> PathBuf {
    skills.join(SKILL_DIR).join("SKILL.md")
}

#[test]
fn print_is_the_repo_skill_verbatim() {
    let out = run_in(&scratch_home(), &["--print"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stdout(&out) == repo_skill(), "the binary embeds a stale skills/kuna/SKILL.md; rebuild kuna-cli");
}

#[test]
fn detects_only_the_agents_that_are_present() {
    let home = scratch_home();
    std::fs::create_dir_all(home.join(".claude")).unwrap();
    let out = run_in(&home, &[]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&home.join(".claude/skills"))).unwrap(), repo_skill());
    assert!(!home.join(".codex").exists(), "installed for an agent that is not present");
    assert!(!home.join(".config/opencode").exists(), "installed for an agent that is not present");
    assert!(stdout(&out).starts_with("installed  claude"), "{}", stdout(&out));
}

#[test]
fn no_agent_found_is_a_usage_error() {
    let home = scratch_home();
    std::fs::create_dir_all(home.join(".config")).unwrap();
    let out = run_in(&home, &[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("--agent"), "{}", stderr(&out));
}

#[test]
fn agent_homes_follow_their_environment_overrides() {
    let home = scratch_home();
    let claude = home.join("claude-cfg");
    let codex = home.join("codex-home");
    let opencode = home.join("opencode-cfg");
    let out = command_in(&home, &["--agent", "all"])
        .env("CLAUDE_CONFIG_DIR", &claude)
        .env("CODEX_HOME", &codex)
        .env("OPENCODE_CONFIG_DIR", &opencode)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(installed(&claude.join("skills")).is_file());
    assert!(installed(&codex.join("skills")).is_file());
    assert_eq!(std::fs::read_to_string(installed(&opencode.join("skills"))).unwrap(), repo_skill());
}

#[test]
fn opencode_installs_under_the_default_config_directory() {
    let home = scratch_home();
    let out = run_in(&home, &["--agent", "opencode"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&home.join(".config/opencode/skills"))).unwrap(), repo_skill());
    assert!(!home.join(".claude").exists());
    assert!(!home.join(".codex").exists());
}

#[test]
fn detects_an_opencode_only_installation() {
    let home = scratch_home();
    std::fs::create_dir_all(home.join(".config/opencode")).unwrap();
    let out = run_in(&home, &[]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&home.join(".config/opencode/skills"))).unwrap(), repo_skill());
    assert!(stdout(&out).starts_with("installed  opencode"), "{}", stdout(&out));
    assert!(!home.join(".claude").exists());
    assert!(!home.join(".codex").exists());
}

#[test]
fn opencode_config_override_takes_precedence_over_xdg() {
    let home = scratch_home();
    let xdg = home.join("xdg");
    let custom = home.join("custom-opencode");
    std::fs::create_dir_all(&custom).unwrap();
    let out = command_in(&home, &[])
        .env("XDG_CONFIG_HOME", &xdg)
        .env("OPENCODE_CONFIG_DIR", &custom)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&custom.join("skills"))).unwrap(), repo_skill());
    assert!(!xdg.exists());
    assert!(!home.join(".config").exists());
}

#[test]
fn opencode_uses_xdg_and_falls_back_for_empty_overrides() {
    let home = scratch_home();
    let xdg = home.join("xdg");
    let out = command_in(&home, &["--agent", "opencode"])
        .env("XDG_CONFIG_HOME", &xdg)
        .env("OPENCODE_CONFIG_DIR", "")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&xdg.join("opencode/skills"))).unwrap(), repo_skill());
    assert!(!home.join(".config").exists());

    let out = command_in(&home, &["--agent", "opencode"])
        .env("XDG_CONFIG_HOME", "")
        .env("OPENCODE_CONFIG_DIR", "")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&home.join(".config/opencode/skills"))).unwrap(), repo_skill());
}

#[test]
fn project_installs_under_the_working_directory() {
    let home = scratch_home();
    let out = run_in(&home, &["--project", "--agent", "all"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(installed(&home.join(".claude/skills")).is_file());
    assert!(installed(&home.join(".agents/skills")).is_file());
    assert_eq!(std::fs::read_to_string(installed(&home.join(".opencode/skills"))).unwrap(), repo_skill());
    assert!(!home.join(".config").exists());
}

#[test]
fn opencode_project_install_ignores_global_overrides() {
    let home = scratch_home();
    let out = command_in(&home, &["--project", "--agent", "opencode"])
        .env("OPENCODE_CONFIG_DIR", home.join("custom"))
        .env("XDG_CONFIG_HOME", home.join("xdg"))
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(installed(&home.join(".opencode/skills"))).unwrap(), repo_skill());
    assert!(!home.join("custom").exists());
    assert!(!home.join("xdg").exists());
    assert!(!home.join(".claude").exists());
    assert!(!home.join(".agents").exists());
}

#[test]
fn a_differing_copy_is_kept_unless_forced() {
    let home = scratch_home();
    let skills = home.join("elsewhere");
    let dir = skills.to_str().unwrap();

    assert_eq!(run_in(&home, &["--dir", dir]).status.code(), Some(0));
    let again = run_in(&home, &["--dir", dir]);
    assert_eq!(again.status.code(), Some(0));
    assert!(stdout(&again).starts_with("current"), "{}", stdout(&again));

    std::fs::write(installed(&skills), "local edits\n").unwrap();
    let kept = run_in(&home, &["--dir", dir]);
    assert_eq!(kept.status.code(), Some(1));
    assert!(stderr(&kept).contains("--force"), "{}", stderr(&kept));
    assert_eq!(std::fs::read_to_string(installed(&skills)).unwrap(), "local edits\n");

    let forced = run_in(&home, &["--dir", dir, "--force"]);
    assert_eq!(forced.status.code(), Some(0), "{}", stderr(&forced));
    assert!(stdout(&forced).starts_with("updated"), "{}", stdout(&forced));
    assert_eq!(std::fs::read_to_string(installed(&skills)).unwrap(), repo_skill());
}

#[test]
fn conflicting_flags_are_usage_errors() {
    let home = scratch_home();
    for argv in [
        &["--dir", "x", "--agent", "claude"][..],
        &["--dir", "x", "--project"],
        &["--print", "--force"],
        &["--agent", "cursor"],
        &["--bogus"],
    ] {
        assert_eq!(run_in(&home, argv).status.code(), Some(2), "{argv:?}");
    }
}

#[test]
fn top_level_help_points_agents_at_the_skill() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna")).arg("--help").output().unwrap();
    assert!(stderr(&out).contains("LLM agents: run `kuna install-skill`"), "{}", stderr(&out));
}
