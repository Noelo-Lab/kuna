use kuna_slacomp::slgh_compile::{SectionVector, SleighCompile};
use kuna_slacomp::slghparse::{ParserActions, SleighParser};
use kuna_slacomp::slghscan::SleighScanner;
use kuna_sleigh::slghsymbol::{Constructor, ConstructorRef};

fn compiler() -> SleighCompile {
    let mut compiler = SleighCompile::new();
    compiler.parse_from_new_file(b"builder.slaspec");
    let mut scanner = SleighScanner::new();
    scanner.open(
        b"define endian = little;\n\
          define space ram type=ram_space size=4 default;\n\
          define space register type=register_space size=4;\n\
          define register offset=0 size=4 contextreg;\n\
          define context contextreg mode=(0,1);\n"
            .to_vec(),
    );
    assert_eq!(SleighParser::new(scanner).parse(&mut compiler).unwrap(), 0);
    compiler.calc_context_layout();
    assert_eq!(compiler.num_errors(), 0);
    compiler
}

fn constructor(compiler: &SleighCompile) -> &Constructor {
    compiler
        .base
        .symtab()
        .get_constructor(ConstructorRef {
            table_id: compiler.base.get_root().unwrap(),
            ct_id: 0,
        })
        .unwrap()
}

fn assert_scope_closed(compiler: &SleighCompile) {
    let table = compiler.base.symtab();
    assert_eq!(
        table.get_current_scope(),
        table.get_global_scope().map(|scope| scope.get_id())
    );
}

#[test]
fn both_builders_attach_main_and_sparse_named_sections() {
    for arena in [false, true] {
        let mut compiler = compiler();
        compiler.new_section_symbol(b"unused");
        let named_symbol = compiler.new_section_symbol(b"named");
        let ctor = compiler.create_constructor(None);
        let main = compiler.enter_section();
        let named = compiler.enter_section();
        if arena {
            let sections = compiler.first_named_section(main, named_symbol);
            let sections = compiler.final_named_section(sections, named);
            compiler.build_constructor_ws4c(ctor, None, None, Some(sections));
        } else {
            let scope = compiler.base.symtab().get_current_scope();
            let mut sections = SectionVector::new(main, scope);
            sections.set_next_index(1);
            sections.append(named, scope);
            compiler.build_constructor(ctor, None, None, Some(sections));
        }
        assert_eq!(compiler.num_errors(), 0);
        let ctor = constructor(&compiler);
        assert!(ctor.get_templ().is_some());
        assert_eq!(ctor.get_num_sections(), 2);
        assert_eq!(ctor.get_named_templ(0), None);
        assert!(ctor.get_named_templ(1).is_some());
        assert_ne!(ctor.get_templ(), ctor.get_named_templ(1));
        assert_eq!(compiler.base.templates().len(), 2);
        assert_scope_closed(&compiler);
    }
}

#[test]
fn both_builders_preserve_inherited_and_local_context_without_sections() {
    for arena in [false, true] {
        let mut compiler = compiler();
        let mode = compiler
            .base
            .symtab()
            .find_symbol(b"mode")
            .unwrap()
            .get_id();
        let mut inherited = Vec::new();
        let value = compiler.pexp_constant(1);
        assert!(compiler.context_mod(&mut inherited, mode, value));
        compiler.push_with(None, None, Some(inherited));
        let ctor = compiler.create_constructor(None);
        let mut local = Vec::new();
        let value = compiler.pexp_constant(2);
        assert!(compiler.context_mod(&mut local, mode, value));
        if arena {
            compiler.build_constructor_ws4c(ctor, None, Some(local), None);
        } else {
            compiler.build_constructor(ctor, None, Some(local), None);
        }
        compiler.pop_with();
        assert_eq!(compiler.num_errors(), 0);
        assert_eq!(constructor(&compiler).get_context_changes().len(), 2);
        assert_eq!(constructor(&compiler).get_templ(), None);
        assert_scope_closed(&compiler);
    }
}

#[test]
fn both_builders_validate_sections_and_close_scope_after_errors() {
    for arena in [false, true] {
        let mut compiler = compiler();
        let ctor = compiler.create_constructor(None);
        let main = compiler.enter_section();
        let value = compiler.intvn_integer_colon(42, 4);
        compiler.set_result_varnode(main, value);
        if arena {
            let sections = compiler.standalone_section(main);
            compiler.build_constructor_ws4c(ctor, None, None, Some(sections));
        } else {
            let scope = compiler.base.symtab().get_current_scope();
            compiler.build_constructor(ctor, None, None, Some(SectionVector::new(main, scope)));
        }
        assert_eq!(compiler.num_errors(), 2);
        assert_eq!(constructor(&compiler).get_templ(), None);
        assert!(compiler.base.templates().is_empty());
        assert_scope_closed(&compiler);
    }
}
