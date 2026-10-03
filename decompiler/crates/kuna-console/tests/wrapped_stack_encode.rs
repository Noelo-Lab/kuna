//! Wrapped locals keep physical data flow while their wire storage stays distinct.
use kuna_base::marshal::XmlEncode;
use kuna_console::assertions::{self, Body, Directive};
use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::print_c;
use std::path::PathBuf;

#[test]
fn recovered_wrapped_local_does_not_create_a_native_join_varnode_or_change_c_on_encode() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    for target in ["thumb", "arm_be"] {
        for opt in ["O0", "O2"] {
            let fixture = format!("wrappedstackaggregate_clang_{opt}_{target}.o");
            let path = root
                .join("decompiler/crates/kuna-analysis/tests/fixtures")
                .join(&fixture);
            let specs = vec![root.join("specs").to_str().unwrap().to_string()];
            let mut program = bootstrap_from_object(path.to_str().unwrap(), "", &specs).unwrap();
            program.commit_pending_analysis().unwrap();
            program.set_assertions(vec![
                Directive {
                    raw: "dev type".into(),
                    body: Body::Typedef {
                        decl:
                            "struct dev {void (*temp)(struct dev *,short *);int pad[16];short t;};"
                                .into(),
                    },
                },
                Directive {
                    raw: "readtemp0 prototype".into(),
                    body: Body::Prototype {
                        func: "readtemp0".into(),
                        decl: "short readtemp0(struct dev d)".into(),
                    },
                },
            ]);
            assertions::apply_program_scoped(&mut program);
            let commands = vec![
                "load function readtemp0".into(),
                "decompile".into(),
                "print C".into(),
                "decompile".into(),
            ];
            let mut status = ConsoleCommands::into_status(commands);
            register_decomp_commands(&mut status);
            let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
            data.as_any_mut()
                .downcast_mut::<IfaceDecompData>()
                .unwrap()
                .conf = Some(program);
            execute(&mut status);
            execute(&mut status);
            let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
            let data = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
            let mut fd = data.fd.take().unwrap();
            let program = data.conf.as_mut().unwrap();
            let before = print_c(program.arch_mut(), &fd);
            assert!(before.contains("return v1.t;"), "{fixture}: {before}");
            let maps = fd.mapped_symbol_specs();
            let wrapped: Vec<_> = maps
                .iter()
                .filter_map(|(_, _, addr, _)| {
                    if !addr.is_join() {
                        return None;
                    }
                    let join = fd.get_arch().manage().find_join(addr.get_offset()).unwrap();
                    (join.num_pieces() == 2
                        && (0..2).all(|i| {
                            join.get_piece(i)
                                .space
                                .as_ref()
                                .is_some_and(|space| space.is_formal_stack_space())
                        }))
                    .then_some(addr.clone())
                })
                .collect();
            let map_locations: Vec<_> = maps
                .iter()
                .map(|(name, ty, addr, _)| {
                    (
                        name.as_str(),
                        ty.get_size(),
                        addr.get_space().unwrap().get_name(),
                        addr.get_offset(),
                    )
                })
                .collect();
            assert_eq!(wrapped.len(), 1, "{fixture}: {map_locations:?}");
            for id in fd.vbank().iter_loc() {
                let varnode = fd.vbank().get(id).unwrap();
                assert!(
                    !wrapped.contains(varnode.get_addr()),
                    "{fixture}: native wrapped JOIN"
                );
            }
            let mut xml = Vec::new();
            fd.encode(&mut XmlEncode::new(&mut xml), 1, true).unwrap();
            let text = String::from_utf8(xml).unwrap();
            assert!(!text.contains("stack:0xfffffff0:16"), "{fixture}: {text}");
            assert_eq!(before, print_c(program.arch_mut(), &fd), "{fixture}");
            let mut again = Vec::new();
            fd.encode(&mut XmlEncode::new(&mut again), 1, true).unwrap();
            assert_eq!(text.as_bytes(), again.as_slice(), "{fixture}");
            if let Some(directory) = std::env::var_os("KUNA_WRAPPED_FUNCTION_DUMP_DIR") {
                std::fs::create_dir_all(&directory).unwrap();
                std::fs::write(
                    PathBuf::from(directory).join(format!("{fixture}.xml")),
                    text,
                )
                .unwrap();
            }
            data.fd = Some(fd);
            execute(&mut status);
            execute(&mut status);
            let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
            let data = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
            let fd = data.fd.take().unwrap();
            let program = data.conf.as_mut().unwrap();
            assert_eq!(
                before,
                print_c(program.arch_mut(), &fd),
                "{fixture}: second decompile"
            );
            let maps = fd.mapped_symbol_specs();
            assert_eq!(
                maps.iter()
                    .filter(|(_, _, addr, _)| wrapped.contains(addr))
                    .count(),
                1,
                "{fixture}: repeated whole mapping"
            );
        }
    }
}
