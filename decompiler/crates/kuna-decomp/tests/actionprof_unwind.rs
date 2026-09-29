//! Profiling must remain usable after a caught action panic.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_decomp::action::{
    Action, ActionBase, ActionContext, ActionGroup, ActionGroupList, ApplyResult,
};
use kuna_decomp::actionprof;
use kuna_decomp::context::ArchContext;
use kuna_decomp::funcdata::Funcdata;

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("kuna_actionprof_{}_{stamp}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct ProbeAction {
    base: ActionBase,
    panic: bool,
}

impl ProbeAction {
    fn new(name: &str, panic: bool) -> Self {
        Self {
            base: ActionBase::new(0, name, "test"),
            panic,
        }
    }
}

impl Action for ProbeAction {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn apply(&mut self, _: &mut Funcdata, _: &mut ActionContext) -> ApplyResult {
        assert!(!self.panic, "intentional test action panic");
        0
    }
    fn clone_filtered(&self, _: &ActionGroupList) -> Option<Box<dyn Action>> {
        None
    }
}

fn data() -> Funcdata {
    let mut spaces = AddrSpaceManager::new();
    spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    spaces
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    spaces.insert_space(ram.clone()).unwrap();
    Funcdata::new(
        "probe",
        "probe",
        Rc::new(ArchContext::new(spaces)),
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap()
}

fn calls(table: &str, key: &str) -> u64 {
    let rows: Vec<_> = table.lines().filter(|line| line.ends_with(key)).collect();
    assert_eq!(rows.len(), 1, "missing or duplicate row {key}: {table}");
    rows[0].split_whitespace().nth(3).unwrap().parse().unwrap()
}

/// This binary has one test: the environment flag is cached once per process.
#[test]
fn nested_panic_closes_frames_and_later_actions_publish() {
    let scratch = Scratch::new();
    let output = scratch.0.join("profile.txt");
    std::env::set_var(actionprof::ENV_VAR, &output);
    let mut data = data();
    let mut context = ActionContext::default();
    actionprof::set_root("probe");
    let mut group = ActionGroup::new(0, "outer");
    group.add_action(Box::new(ProbeAction::new("panics", true)));

    assert!(catch_unwind(AssertUnwindSafe(|| group.perform(&mut data, &mut context))).is_err());
    let after_panic = actionprof::render();
    assert_eq!(calls(&after_panic, "probe/panics"), 1);
    assert_eq!(calls(&after_panic, "probe/outer"), 1);
    assert_eq!(std::fs::read_to_string(&output).unwrap(), after_panic);

    for count in 1..=2 {
        assert_eq!(
            ProbeAction::new("healthy", false).perform(&mut data, &mut context),
            0
        );
        let table = actionprof::render();
        assert_eq!(calls(&table, "probe/healthy"), count);
        assert_eq!(calls(&table, "probe/panics"), 1);
        assert_eq!(std::fs::read_to_string(&output).unwrap(), table);
    }

    std::env::set_var(actionprof::ENV_VAR, &scratch.0);
    assert_eq!(
        ProbeAction::new("healthy", false).perform(&mut data, &mut context),
        0
    );
    assert_eq!(calls(&actionprof::render(), "probe/healthy"), 3);
}
