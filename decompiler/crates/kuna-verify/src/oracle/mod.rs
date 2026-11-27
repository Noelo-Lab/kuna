//! Export a shared seed and memory map for concrete function execution.

pub(crate) mod elf;

use std::collections::BTreeMap;

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::state::{ExplicitState, Region, State, Writes};
use crate::storage::Storage;
use crate::value::Halt;
use crate::{abi, calls, graph, machine, program, session, state::Env};

pub(crate) const PAGE: u64 = 4096;
pub(crate) const STACK: u64 = 0x7000_0000;
pub(crate) const CALLER: u64 = STACK + 3 * PAGE;
pub(crate) const SP: u64 = CALLER - 8;
pub(crate) const HEAP: u64 = 0x6000_0000;
const RETURN: u64 = 0x7100_0000;
const MAX_MAPPED: u64 = 16 * 1024 * 1024;
pub(crate) const GPR: [&str; 16] = [
    "RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RBP", "RSP", "R8", "R9", "R10", "R11", "R12", "R13",
    "R14", "R15",
];
const PRESERVED: [&str; 6] = ["RBX", "RBP", "R12", "R13", "R14", "R15"];
const FLAGS: [&str; 21] = [
    "CF", "F1", "PF", "F3", "AF", "F5", "ZF", "SF", "TF", "IF", "DF", "OF", "IOPL", "NT", "F15",
    "RF", "VM", "AC", "VIF", "VIP", "ID",
];
pub(crate) const SYMBOLIC_FLAGS: [(&str, u8); 7] = [
    ("CF", 0),
    ("PF", 2),
    ("AF", 4),
    ("ZF", 6),
    ("SF", 7),
    ("DF", 10),
    ("OF", 11),
];

pub struct RunConfig<'a> {
    pub seed: u64,
    pub budget: u64,
    pub input_regs: &'a [(String, String)],
    pub external_entries: &'a BTreeMap<u64, u8>,
    pub opcode_mutant: Option<usize>,
    pub memory: &'a BTreeMap<u64, u8>,
}

/// Run the Kuna interpreters and export the exact input for the external runner.
pub fn capture(
    path: &str,
    function: &str,
    options: &[(String, String)],
    spec_roots: &[String],
    config: RunConfig<'_>,
) -> Result<Value, String> {
    let inputs = parse_inputs(config.input_regs)?;
    if config
        .external_entries
        .iter()
        .any(|(&at, &arity)| at >= 1 << 47 || at == RETURN || arity > 6)
    {
        return Err(
            "external entries require a low canonical address and at most six integer arguments"
                .into(),
        );
    }
    let image = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    let mut regions = elf::regions(&image)?;
    let mut stack = vec![0; (SP - STACK) as usize];
    stack.extend_from_slice(&RETURN.to_le_bytes());
    regions.extend([
        Region {
            address: STACK,
            size: 3 * PAGE,
            bytes: stack,
            permissions: "rw",
            observe: false,
        },
        Region {
            address: CALLER,
            size: PAGE,
            bytes: Vec::new(),
            permissions: "rw",
            observe: true,
        },
        Region {
            address: HEAP,
            size: 2 * PAGE,
            bytes: Vec::new(),
            permissions: "rw",
            observe: true,
        },
    ]);
    regions.sort_unstable_by_key(|r| r.address);
    if regions.windows(2).any(|w| w[0].end() > w[1].address) {
        return Err("overlapping mapped ELF or synthetic pages".into());
    }
    if regions
        .iter()
        .any(|r| (r.address..r.end()).contains(&RETURN))
    {
        return Err("image maps the synthetic return address".into());
    }
    if regions.iter().map(|r| r.size).sum::<u64>() > MAX_MAPPED {
        return Err("mapped pages exceed the explicit case size limit".into());
    }
    for (&address, &value) in config.memory {
        if (SP..SP + 8).contains(&address) {
            return Err("initial memory override would replace the return sentinel".into());
        }
        let region = regions
            .iter_mut()
            .find(|r| r.address <= address && address < r.end() && r.permissions == "rw")
            .ok_or_else(|| {
                format!("initial byte override at {address:#x} is outside writable memory")
            })?;
        let offset = (address - region.address) as usize;
        if region.bytes.len() <= offset {
            region.bytes.resize(offset + 1, 0);
        }
        region.bytes[offset] = value;
    }

    let mut result = None;
    let mut regions = Some(regions);
    let commands = session::binary_script(path, &[function.to_owned()], options);
    session::drive(commands, None, spec_roots, |prog, fd| {
        if let Some(regions) = regions.take() {
            result = Some(observe(prog, fd, &config, regions, &inputs));
        }
    })?;
    let mut capture = result.ok_or_else(|| "no decompiled function is available".to_string())??;
    capture["decompile_options"] = json!(options);
    capture["initial_memory_overrides"] = json!(config
        .memory
        .iter()
        .map(|(at, byte)| (format!("{at:#x}"), format!("{byte:#04x}")))
        .collect::<BTreeMap<_, _>>());
    capture["provenance"]["elf_sha256"] = json!(bytes_hex(Sha256::digest(image).iter().copied()));
    Ok(capture)
}

fn parse_inputs(input_regs: &[(String, String)]) -> Result<BTreeMap<&str, u128>, String> {
    let mut inputs = BTreeMap::new();
    for (name, hex) in input_regs {
        let bits = if (GPR.contains(&name.as_str()) && name != "RSP") || name == "RFLAGS" {
            64
        } else if (0..16).any(|i| *name == format!("XMM{i}")) {
            128
        } else {
            return Err(format!("cannot override {name} in this case model"));
        };
        let digits = hex.strip_prefix("0x").unwrap_or_default();
        if digits.len() > bits / 4 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(format!(
                "{name} requires a {bits}-bit hexadecimal input with 0x prefix"
            ));
        }
        let value = u128::from_str_radix(digits, 16)
            .map_err(|_| format!("{name} requires a hexadecimal input with 0x prefix"))?;
        if name == "RFLAGS" && (value & !0xed7 != 0 || value & 2 == 0) {
            return Err("RFLAGS requires fixed bit 1 and only arithmetic flags, DF or IF".into());
        }
        if inputs.insert(name.as_str(), value).is_some() {
            return Err(format!("{name} is specified twice"));
        }
    }
    Ok(inputs)
}

pub(crate) fn register(env: &Env, name: &str) -> Result<Storage, String> {
    env.prog
        .arch()
        .manage()
        .register_lookup()
        .and_then(|lookup| lookup.probe_register(name))
        .and_then(|r| Some(Storage::new(r.space.as_ref()?, r.offset, r.size)))
        .ok_or_else(|| format!("x86-64 SLEIGH register {name} is unavailable"))
}

fn initial_state(
    env: &Env,
    entry: u64,
    regions: Vec<Region>,
    inputs: &BTreeMap<&str, u128>,
) -> Result<(ExplicitState, BTreeMap<String, String>), String> {
    let mut explicit = ExplicitState {
        regions,
        ..ExplicitState::default()
    };
    explicit.check(entry, 1, b'x').map_err(|e| e.render())?;
    let mut regs = BTreeMap::new();
    let mut sampled = State::new(env, crate::state::EDGE_RUNS);
    let mut set =
        |sleigh: &str, external: Option<&str>, value: Option<u128>| -> Result<(), String> {
            let st = register(env, sleigh)?;
            let value = match value {
                Some(value) => value,
                None => sampled
                    .read_storage(&st.space, st.off, st.size)
                    .map_err(|e| e.render())?,
            };
            for (i, byte) in value
                .to_le_bytes()
                .into_iter()
                .take(st.size as usize)
                .enumerate()
            {
                explicit.registers.insert(st.off + i as u64, byte);
            }
            if let Some(name) = external {
                regs.insert(name.to_owned(), format!("{value:#x}"));
            }
            Ok(())
        };
    for name in GPR {
        let value = if name == "RSP" {
            Some(u128::from(SP))
        } else {
            inputs.get(name).copied()
        };
        set(name, Some(name), value)?;
    }
    for i in 0..16 {
        let name = format!("XMM{i}");
        set(&name, Some(&name), inputs.get(name.as_str()).copied())?;
    }
    for i in 0..8 {
        set(&format!("ST{i}"), Some(&format!("FP{i}")), Some(0))?;
    }
    let packed_flags = inputs.get("RFLAGS").copied().unwrap_or(2);
    for (name, external, value) in [
        ("FS_OFFSET", Some("FS_BASE"), 0),
        ("GS_OFFSET", Some("GS_BASE"), 0),
        ("MXCSR", Some("MXCSR"), 0x1f80),
        ("FPUControlWord", Some("FPCW"), 0x037f),
        ("FPUStatusWord", Some("FPSW"), 0),
        ("FPUTagWord", Some("FPTAG"), 0xffff),
        ("rflags", Some("RFLAGS"), packed_flags),
        ("RIP", None, u128::from(entry)),
    ] {
        set(name, external, Some(value))?;
    }
    for name in FLAGS {
        let bit = SYMBOLIC_FLAGS
            .iter()
            .find_map(|&(flag, n)| (flag == name).then_some(n))
            .or(match name {
                "F1" => Some(1),
                "IF" => Some(9),
                _ => None,
            });
        let value = bit.map_or(0, |n| (packed_flags >> n) & 1);
        set(name, None, Some(value))?;
    }
    Ok((explicit, regs))
}

fn observe(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    config: &RunConfig<'_>,
    regions: Vec<Region>,
    inputs: &BTreeMap<&str, u128>,
) -> Result<Value, String> {
    let (seed, budget) = (config.seed, config.budget);
    let mut env = Env::new(prog, seed).map_err(|e| e.render())?;
    let entry = fd.get_address().get_offset();
    if config.external_entries.contains_key(&entry) {
        return Err("an external entry cannot be the function entry".into());
    }
    env.sp0 = SP;
    env.ret_addr = Some(RETURN);
    env.ret_slot = 8;
    let (mut explicit, registers) = initial_state(&env, entry, regions, inputs)?;
    explicit.external_entries = config.external_entries.clone();
    let external_calls: BTreeMap<_, _> = config
        .external_entries
        .iter()
        .map(|(&at, &arity)| (format!("{at:#x}"), arity))
        .collect();
    let memory: Vec<_> = explicit
        .regions
        .iter()
        .map(|region| {
            json!({
                "address": format!("{:#x}", region.address),
                "size": region.size,
                "bytes": bytes_hex(region.bytes.iter().copied()),
                "permissions": region.permissions,
                "observe": region.observe,
            })
        })
        .collect();
    env.explicit = Some(explicit);
    env.fd = Some(fd);
    let mut calls = calls::sites(fd, &env);
    env.arg_regs = calls.input_regs.clone();
    let mut program = program::compile(prog, fd, &calls);
    let mutation = apply_opcode_mutant(&mut program, &mut calls, &env, config.opcode_mutant)?;
    let run = crate::state::EDGE_RUNS;

    let raw = machine::run(&env, run, entry, budget, &calls, None);
    let replay = raw.as_ref().map(|r| r.calls.as_slice()).unwrap_or_default();
    let ir = graph::run(&env, &program, run, budget, &calls, replay, None);
    let explicit = env
        .explicit
        .as_ref()
        .expect("the case supplies explicit state");
    let ir = match ir {
        Err(h) => halted("final-ir", h),
        Ok(ir) => {
            let status = if ir.noreturn
                || ir.unmatched_switch
                || ir.overlapping_copy.is_some()
                || ir.read_uninitialized
                || !ir.calls.is_empty()
                || !ir.events.is_empty()
            {
                "incomplete"
            } else {
                "returned"
            };
            let claim = abi::claim(prog, fd);
            json!({
                "backend": "final-ir",
                "status": status,
                "return": ir.ret.zip(claim.storage).map(|(value, st)| json!({
                    "storage": st.name(prog), "size": st.size, "value": format!("{value:#x}"),
                })),
                "calls": call_records(&ir.calls),
                "events": event_records(&ir.events),
                "raw_callee_effects_available": replay.iter().any(|r| r.effect.is_some()),
                "unverified_arguments": ir.call_inputs.len() + ir.unkept_inputs,
                "memory": observed_memory(explicit, env.ram.get_index(), &ir.writes),
            })
        }
    };
    let raw = match raw {
        Err(h) => halted("raw-pcode", h),
        Ok(mut raw) => {
            let status = if raw.noreturn
                || raw.bad_return.is_some()
                || !raw.calls.is_empty()
                || !raw.events.is_empty()
                || !raw.followed.is_empty()
            {
                "incomplete"
            } else {
                "returned"
            };
            let mut final_regs = BTreeMap::new();
            for name in GPR.into_iter().chain(["XMM0", "XMM1"]) {
                let st = register(&env, name)?;
                let value = raw
                    .state
                    .read_storage(&st.space, st.off, st.size)
                    .map_err(|e| e.render())?;
                final_regs.insert(name, format!("{value:#x}"));
            }
            let mut violations: Vec<_> = PRESERVED
                .into_iter()
                .filter(|name| final_regs[*name] != registers[*name])
                .collect();
            if final_regs["RSP"] != format!("{:#x}", SP + 8) {
                violations.push("RSP");
            }
            json!({
                "backend": "raw-pcode",
                "status": status,
                "reason": raw.bad_return.as_ref().map(Halt::render),
                "registers": final_regs,
                "abi_violations": (status == "returned").then_some(violations),
                "calls": call_records(&raw.calls),
                "events": event_records(&raw.events),
                "memory": observed_memory(explicit, env.ram.get_index(), &raw.state.writes()),
            })
        }
    };

    Ok(json!({
        "version": 1, "function": fd.get_name(), "seed": format!("{seed:#x}"),
        "mutation": mutation,
        "provenance": {
            "language": prog.arch().archid,
            "sla_sha256": prog.arch().decode_recipe().map(|recipe| bytes_hex(Sha256::digest(recipe.sla.as_ref()).iter().copied())),
        },
        "pcode_budget": budget, "case": {
            "version": 1, "architecture": "x86-64", "abi": "sysv-amd64", "cpu_model": "qemu64",
            "entry": format!("{entry:#x}"), "return_address": format!("{RETURN:#x}"),
            "registers": registers, "memory": memory,
            "external_entries": external_calls.keys().collect::<Vec<_>>(), "external_calls": external_calls,
        },
        "observations": {"raw": raw, "ir": ir},
    }))
}

pub(crate) fn apply_opcode_mutant(
    program: &mut program::Program,
    calls: &mut calls::Calls,
    env: &Env,
    op: Option<usize>,
) -> Result<Option<Value>, String> {
    let Some(op) = op else { return Ok(None) };
    let mutation = crate::mutate::Mutation {
        block: 0,
        op,
        operator: crate::mutate::Operator::Opcode,
    };
    let description = crate::mutate::apply(program, calls, env, mutation)
        .ok_or_else(|| format!("no opcode mutant at operation {op} in the entry block"))?;
    Ok(Some(
        json!({ "operator": "opcode", "block": 0, "op": op, "description": description }),
    ))
}

fn halted(backend: &str, halt: Halt) -> Value {
    let status = if matches!(halt, Halt::Trap(_)) {
        "trap"
    } else {
        "incomplete"
    };
    json!({"backend": backend, "status": status, "reason": halt.render()})
}

fn call_records(calls: &[calls::Record]) -> Vec<Value> {
    calls.iter().map(|c| json!({
        "address": format!("{:#x}", c.at), "target": c.target.map(|at| format!("{at:#x}")),
        "indirect": c.indirect, "arguments": c.args.iter().map(|a| format!("{a:#x}")).collect::<Vec<_>>(),
        "concrete_effect": c.effect.is_some(),
    })).collect()
}

fn event_records(events: &[calls::UserEvent]) -> Vec<Value> {
    events
        .iter()
        .map(|e| {
            json!({
                "address": format!("{:#x}", e.at), "name": e.name,
                "inputs": e.ins.iter().map(|v| format!("{v:#x}")).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn observed_memory(explicit: &ExplicitState, ram: i32, writes: &Writes) -> Vec<Value> {
    explicit.regions.iter().filter(|r| r.observe).map(|region| {
        let mut bytes = region.bytes.clone();
        bytes.resize(region.size as usize, 0);
        for (&(_, at), &value) in writes.bytes.range((ram, region.address)..(ram, region.end())) {
            bytes[(at - region.address) as usize] = value;
        }
        json!({"address": format!("{:#x}", region.address), "bytes": bytes_hex(bytes.into_iter())})
    }).collect()
}

fn bytes_hex(bytes: impl Iterator<Item = u8>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.size_hint().0.saturating_mul(2));
    for byte in bytes {
        result.push(char::from(HEX[(byte >> 4) as usize]));
        result.push(char::from(HEX[(byte & 15) as usize]));
    }
    result
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    bytes_hex(Sha256::digest(bytes).iter().copied())
}
