use super::*;

#[test]
fn failed_concrete_callee_restores_state_and_metadata() {
    let image = br#"<binaryimage arch="x86:LE:64:default:gcc">
        <bytechunk space="ram" offset="0x1000" readonly="true">b800300000c60001e8f30f00000f0b</bytechunk>
        <bytechunk space="ram" offset="0x1100" readonly="true">e8fbfeffffc3</bytechunk>
        <symbol space="ram" offset="0x1100" name="caller"/>
    </binaryimage>"#;
    let mut store = kuna_base::xml::DocumentStorage::new();
    let root = store.parse_document(image).unwrap().get_root().clone();
    let specs = vec![format!("{}/../../../specs", env!("CARGO_MANIFEST_DIR"))];
    let prog = kuna_console::engine::bootstrap_program(root, "x86:LE:64:default:gcc", &specs).unwrap();
    let commands = vec!["load function caller".into(), "decompile".into()];
    crate::session::drive(commands, Some(prog), &specs, |prog, fd| {
        let env = Env::new(prog, 0).unwrap();
        env.record_path.set(true);
        let mut calls = crate::calls::sites(fd, &env);
        calls.default.killed.clear();
        let run = || {
            let mut m = Machine::new(&env, &calls, 0, 1000, None, None);
            let key = (env.ram.get_index(), 0x3000);
            m.state.write(&env.ram, key.1, 1, 1).unwrap();
            m.shadow.as_mut().unwrap().bytes.insert(key, (Rc::new(crate::concolic::Expr::Const(1, 8)), 0, 1));
            (m, key)
        };
        let (mut probe, key) = run();
        assert!(exec(&mut probe, 0x1000, Mode::Opaque, 1, None).is_err());
        assert_eq!(probe.stubs, 1);
        assert!(probe.narrowed);
        assert!(!probe.shadow.as_ref().unwrap().bytes.contains_key(&key));

        let (mut m, key) = run();
        assert!(m.real(0x1100, 0x1000).unwrap().is_none());
        assert_eq!(m.stubs, 0);
        assert!(!m.narrowed);
        assert_eq!(m.state.epoch, 0);
        assert!(m.shadow.as_ref().unwrap().bytes.contains_key(&key));
        assert!(m.callee_pool < 1000);
    }).unwrap();
}
