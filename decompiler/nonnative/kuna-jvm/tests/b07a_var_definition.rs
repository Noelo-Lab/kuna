//! `VarDefinitionHelper.remapClashingNames` on hand-built statement trees,
//! against the real Vineflower 1.12.0 (`data/b07a/VarDefOracle.java` ->
//! `data/b07a/b07a_golden.txt`).

use kuna_jvm::assignment_exprent::AssignmentExprent;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::reset_class_scope;
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_definition_helper::VarDefinitionHelper;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_processor::VarProcessor;
use kuna_jvm::var_type::VarType;
use kuna_jvm::var_version_pair::VarVersionPair;

use crate::common;
use crate::k2_dom_helper::fixture_method;

const GOLDEN: &str = include_str!("data/b07a/b07a_golden.txt");

fn golden(tag: &str) -> String {
    GOLDEN
        .lines()
        .find(|l| l.split(' ').next() == Some(tag))
        .unwrap_or_else(|| panic!("no golden line {tag}"))
        .to_string()
}

struct Scenario {
    mt: StructMethod,
    varproc: VarProcessor,
}

impl Scenario {
    fn new(mt: StructMethod) -> Scenario {
        reset_class_scope();
        let varproc = VarProcessor::new(mt, &mt.method_descriptor());
        kuna_jvm::decompiler_context::DecompilerContext::start_method(varproc);
        Scenario { mt, varproc }
    }

    fn var(&self, index: i32, name: Option<&str>, def: bool) -> VarExprent {
        let var = VarExprent::new(index, Some(VarType::VARTYPE_INT()), Some(self.varproc));
        var.set_definition(def);
        if let Some(name) = name {
            self.varproc
                .set_var_name(VarVersionPair::new(index, 0), name);
        }
        var
    }

    fn assign(&self, index: i32, name: &str) -> Statement {
        let bb = BasicBlockStatement::create();
        let left = self.var(index, Some(name), true);
        let right = ConstExprent::new_int(index, true, None);
        let assign = AssignmentExprent::new(left.0, right.0, None);
        bb.get_exprents().unwrap().add(assign.0);
        bb.0
    }

    fn block(&self, exprents: Vec<Exprent>) -> Statement {
        let bb = BasicBlockStatement::create();
        for e in exprents {
            bb.get_exprents().unwrap().add(e);
        }
        bb.0
    }

    fn run(&self, tag: &str, head: Statement) -> String {
        let root = RootStatement::new(head, DummyExitStatement::new(), self.mt);
        let helper = VarDefinitionHelper::new_run(root, self.mt, self.varproc, false);
        helper.remap_clashing_names(root.0, self.mt);
        let mut line = tag.to_string();
        for (k, v) in helper.get_clashing_names().borrow().entry_vec() {
            line.push_str(&format!(" {},{}={}", k.var, k.version, v));
        }
        line
    }
}

fn seq(stats: &[Statement]) -> Statement {
    let seq = SequenceStatement::new(stats.to_vec());
    seq.set_all_parent();
    seq.0
}

#[test]
fn remap_clashing_names_matches_vineflower() {
    let _context = common::install_default_context();
    let mt = fixture_method();

    let s = Scenario::new(mt);
    let head = seq(&[s.assign(5, "a"), s.assign(6, "a")]);
    assert_eq!(s.run("seq_clash", head), golden("seq_clash"));

    let s = Scenario::new(mt);
    let head = seq(&[seq(&[s.assign(5, "a")]), s.assign(6, "a")]);
    assert_eq!(s.run("nested_scope", head), golden("nested_scope"));

    let s = Scenario::new(mt);
    let head = seq(&[s.assign(5, "a"), s.assign(6, "a"), s.assign(7, "a")]);
    assert_eq!(s.run("triple", head), golden("triple"));

    let s = Scenario::new(mt);
    s.varproc.set_var_name(VarVersionPair::new(1, 0), "x");
    let head = seq(&[s.assign(5, "x"), s.assign(6, "x")]);
    assert_eq!(s.run("param_clash", head), golden("param_clash"));

    let s = Scenario::new(mt);
    let with_defs = seq(&[s.assign(8, "b")]);
    with_defs
        .get_var_definitions()
        .add(s.var(7, Some("b"), true).0);
    assert_eq!(
        s.run("var_definitions", with_defs),
        golden("var_definitions")
    );

    let s = Scenario::new(mt);
    let bare = s.block(vec![
        s.var(9, Some("c"), true).0,
        s.var(9, Some("c"), false).0,
    ]);
    let left = s.var(10, Some("c"), true);
    let right = s.var(9, None, false);
    let reuse = s.block(vec![AssignmentExprent::new(left.0, right.0, None).0]);
    let head = seq(&[bare, reuse, s.assign(11, "cx"), s.assign(12, "unnamed_ok")]);
    assert_eq!(s.run("bare_def", head), golden("bare_def"));

    let s = Scenario::new(mt);
    let many: Vec<Statement> = (0..20).map(|i| s.assign(20 + i * 7, "v")).collect();
    assert_eq!(s.run("many", seq(&many)), golden("many"));
}
