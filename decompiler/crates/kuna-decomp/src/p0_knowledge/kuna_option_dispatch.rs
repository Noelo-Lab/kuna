//! Named kuna option handlers and the allowlist used by every frontend.
//!
//! Each declaration defines both the dispatch arm and its registered name.
//! Catalog metadata remains in phases.toml and is checked against this list.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::uint4;

use crate::architecture::Architecture;
use crate::options::on_or_off;

macro_rules! kuna_options {
    ($this:ident, $value:ident; $($name:literal => $apply:expr),* $(,)?) => {
        pub const KUNA_OPTION_NAMES: &[&str] = &[$($name),*];

        impl Architecture {
            /// Validate and apply a named kuna option to its live configuration.
            pub fn set_kuna_option(&mut $this, name: &str, $value: &str) -> KunaResult<String> {
                macro_rules! on_off {
                    ($field:ident, $label:literal) => {{
                        let val = on_or_off($value)?;
                        $this.$field = val;
                        Ok(format!(concat!($label, " turned {}"), if val { "on" } else { "off" }))
                    }};
                }
                match name {
                    $($name => $apply,)*
                    other => Err(KunaError::parse(format!("Unknown kuna option: {other}"))),
                }
            }
        }
    };
}

kuna_options! { self, p1;
    "outlang" => {
        self.set_print_language_checked(p1)?;
        Ok(format!("Output language set to {}", self.print().get_name()))
    },
    "compareform" => {
        let (form, msg) = crate::kuna_compareform::parse_compare_form(p1)?;
        self.present_lessequal = form.present_lessequal();
        Ok(msg)
    },
    "armconstshift" => on_off!(arm_constant_shift, "ARM constant-shift C11 conversion"),
    "rustarrayzero" => on_off!(rust_array_zero, "Rust byte-array zero values"),
    "arraynotation" => {
        let (val, msg) = crate::kuna_arraynotation::OptionArrayNotation.apply(p1)?;
        self.print_mut().options.set_array_notation(val);
        Ok(msg)
    },
    "truthycond" => {
        let (val, msg) = crate::kuna_truthycond::OptionTruthyCond.apply(p1)?;
        self.print_mut().options.set_truthy_cond(val);
        Ok(msg)
    },
    "braceelide" => {
        let (val, msg) = crate::kuna_braceelide::OptionBraceElide.apply(p1)?;
        self.print_mut().options.set_brace_elide(val);
        Ok(msg)
    },
    "infloopstyle" => {
        let (top, msg) = crate::kuna_infloopstyle::parse_inf_loop_style(p1)?;
        self.print_mut().options.inf_loop_top = top;
        Ok(msg)
    },
    "warnstyle" => {
        let (val, msg) = crate::kuna_warnstyle::OptionWarnStyle.apply(p1)?;
        self.print_mut().options.set_warn_inline(val);
        Ok(msg)
    },
    "partialaccess" => {
        let value = on_or_off(p1)?;
        self.print_mut().options.partial_access = value;
        Ok(format!("Typed pointer partial accesses turned {}", if value { "on" } else { "off" }))
    },
    "bytearrayc" => {
        let value = on_or_off(p1)?;
        self.print_mut().options.bytearray_c = value;
        Ok(format!("Standard C byte-array transfers turned {}", if value { "on" } else { "off" }))
    },
    "structdefs" => {
        let (val, msg) = crate::kuna_structdefs::OptionStructDefs.apply(p1)?;
        self.print_mut().options.set_struct_defs(val);
        Ok(msg)
    },
    "globalref" => {
        let (val, msg) = crate::kuna_globalref::OptionGlobalRef.apply(p1)?;
        self.print_mut().options.set_global_ref(val);
        Ok(msg)
    },
    "returnpair" => {
        let (form, msg) = crate::kuna_returnpair::parse_return_pair_form(p1)?;
        self.return_single = form.return_single();
        Ok(msg)
    },
    "stubtailcall" => on_off!(stub_tail_call, "Loader-proven stub tail calls"),
    "globalarrayanchor" => on_off!(global_array_anchor, "Global array anchor preservation"),
    "signedness" => {
        let (val, msg) = crate::kuna_typeround::OptionSignedness.apply(p1)?;
        self.signedness = val;
        Ok(msg)
    },
    "retsplitglobal" => {
        let (val, msg) =
            crate::p8_structure::kuna_retsplitglobal::OptionRetSplitGlobal.apply(p1)?;
        self.ret_split_global = val;
        Ok(msg)
    },
    "retsplitlimit" => {
        let (val, msg) =
            crate::p8_structure::kuna_retsplitlimit::OptionRetSplitLimit.apply(p1)?;
        self.ret_split_limit = val;
        Ok(msg)
    },
    "v850indirectbranch" => on_off!(v850_indirect_branch, "V850 indirect-branch reclassification"),
    "fastfailnoreturn" => on_off!(fastfail_noreturn, "Windows int 0x29 (__fastfail) no-return"),
    "ptrauthtrap" => on_off!(ptrauth_trap, "arm64e pointer-auth failure-check removal"),
    "int3pad" => {
        let (mode, msg) = crate::kuna_int3pad::OptionInt3Pad.apply(p1)?;
        self.int3_pad = mode;
        Ok(msg)
    },
    "x64syscall" => {
        let (mode, msg) = crate::kuna_x64syscall::OptionX64Syscall.apply(p1)?;
        self.x64_syscall = mode;
        Ok(msg)
    },
    "fpuemu" => {
        let (mode, msg) = crate::kuna_fpuemu::parse_fpuemu_mode(p1).map_err(KunaError::parse)?;
        self.fpu_emu = mode.as_u8();
        crate::kuna_fpuemu::apply(self);
        Ok(msg)
    },
    "syscallregs" => {
        let (mode, msg) = crate::kuna_syscallregs::OptionSyscallRegs.apply(p1)?;
        self.syscall_regs = mode;
        Ok(msg)
    },
    "pebnames" => {
        let (mode, msg) = crate::kuna_pebnames::OptionPebNames.apply(p1)?;
        self.peb_names = mode;
        Ok(msg)
    },
    "structsynth" => {
        let (mode, msg) = crate::kuna_structsynth::OptionStructSynth.apply(p1)?;
        self.struct_synth = mode;
        Ok(msg)
    },
    "structmerge" => {
        let (mode, msg) = crate::kuna_structmerge::OptionStructMerge.apply(p1)?;
        self.struct_merge = mode;
        Ok(msg)
    },
    "structheadless" => {
        let (mode, msg) = crate::kuna_structheadless::OptionStructHeadless.apply(p1)?;
        self.struct_headless = mode;
        Ok(msg)
    },
    "fieldtype" => {
        let (val, msg) = crate::kuna_fieldtype::OptionFieldType.apply(p1)?;
        self.field_type = val;
        Ok(msg)
    },
    "decodehalt" => on_off!(decode_halt, "Decode-failure halt reporting"),
    "msvcftol" => on_off!(msvc_ftol, "MSVC __ftol-family call-fixup"),
    "tailcalljump" => {
        let (jumps, tables, msg) = crate::kuna_tailcalljump::tail_call_mode(p1)?;
        self.tail_call_jumps = jumps;
        self.tail_call_tables = tables;
        Ok(msg.to_string())
    },
    "gottailcall" => {
        let (val, msg) = crate::kuna_gottailcall::OptionGotTailCall.apply(p1)?;
        self.got_tail_call = val;
        Ok(msg)
    },
    "tailcallframe" => on_off!(tail_call_frame, "Frame-teardown tail-call recovery"),
    "tailcallsaved" => {
        on_off!(tail_call_saved, "Saved-register restore test for a frame teardown")
    },
    "calltrampoline" => on_off!(call_trampoline, "Return-address-discarding call trampoline flow-through"),
    "callpopret" => on_off!(call_pop_ret, "Return-address-popping call transfer flow-through"),
    "entryretdispatch" => on_off!(entry_ret_dispatch, "Entry-point RET-dispatch call-chain recovery"),
    "pushimmediateret" => on_off!(push_immediate_ret, "Push-immediate RET tail-transfer recovery"),
    "funcboundflow" => on_off!(funcbound_flow, "Fall-through bound at function entries"),
    "mappedflowboundary" => on_off!(mapped_flow_boundary, "Mapped ELF x86 flow boundaries"),
    "overlapbranch" => on_off!(overlap_branch, "Overlapping-branch fall-through truncation"),
    "cleanupcode" => on_off!(remove_cleanup_code, "Rust drop/deallocate call removal"),
    "linuxsyscall" => on_off!(linux_syscall, "Linux int 0x80 syscall naming"),
    "msvcstrappend" => on_off!(msvc_str_append, "MSVC std::string inlined append collapsing"),
    "switchselector" => on_off!(switch_selector_guard, "Lowered-switch in-function-selector restriction"),
    "noreturn_extern" => on_off!(noreturn_extern_calls, "Name-based extern no-return"),
    "stackaddrargtrial" => on_off!(stack_addr_arg_trial, "Stack-address input trials"),
    "mipsunaligned" => on_off!(mips_unaligned, "MIPS unaligned load/store pairs as one access"),
    "callwide" => {
        let enabled = on_or_off(p1)?;
        self.kuna_callwide.clear();
        self.call_wide = enabled;
        Ok(format!("Caller-proved register-pair returns turned {}", if enabled { "on" } else { "off" }))
    },
    "bejoin" => {
        let (mode, msg) = crate::kuna_bejoin::BeJoinMode::parse(p1)?;
        self.kuna_callwide.clear();
        self.be_join = mode;
        Ok(msg)
    },
    "bejoinpass" => {
        let (mode, msg) = crate::kuna_bejoinpass::BeJoinPassMode::parse(p1)?;
        self.be_join_pass = mode;
        Ok(msg)
    },
    "rustabi" => {
        let (mode, msg) = crate::kuna_rustabi::parse_rust_abi_mode(p1)?;
        self.rust_abi = mode.as_u8();
        Ok(msg)
    },
    "fatstr" => {
        let (mode, msg) = crate::kuna_fatstr::parse_fatstr_mode(p1).map_err(KunaError::parse)?;
        self.fat_str = mode.as_u8();
        Ok(msg)
    },
    "gobool" => {
        let (mode, msg) = crate::kuna_gobool::parse_gobool_mode(p1).map_err(KunaError::parse)?;
        self.go_bool = mode.as_u8();
        Ok(msg)
    },
    "rustprobestack" => {
        let (mode, msg) = crate::kuna_rustprobestack::parse_mode(p1).map_err(KunaError::parse)?;
        self.rust_probestack = mode;
        Ok(msg)
    },
    "gomorestack" => {
        let (mode, msg) = crate::kuna_gomorestack::parse_gomorestack_mode(p1).map_err(KunaError::parse)?;
        self.go_morestack = mode.as_u8();
        Ok(msg)
    },
    "goreg" => {
        let val = on_or_off(p1)?;
        crate::kuna_goreg::set_goreg_env(val);
        self.go_reg = val;
        Ok(format!("Goroutine register naming turned {}", if val { "on" } else { "off" }))
    },
    "goabi" => {
        let on = crate::kuna_goabi::parse_goabi(p1).map_err(KunaError::parse)?;
        crate::kuna_goabi::set_goabi_env(on);
        self.analysis_goabi = on;
        Ok(format!("Golang compiler-spec selection turned {}", if on { "auto" } else { "off" }))
    },
    "gowritebarrier" => {
        let (mode, msg) = crate::kuna_gowritebarrier::parse_gowritebarrier_mode(p1).map_err(KunaError::parse)?;
        self.go_write_barrier = mode.as_u8();
        Ok(msg)
    },
    "sourcelang" => {
        let forced = crate::kuna_sourcelang::parse_option_value(p1).map_err(KunaError::parse)?;
        crate::kuna_sourcelang::set_sourcelang_env(forced);
        self.source_lang_forced = forced;
        Ok(match forced {
            Some(lang) => format!("Source language forced to {}", lang.as_str()),
            None => "Source language follows detection (auto)".to_string(),
        })
    },
    "sparcstructret" => on_off!(sparc_struct_return, "SPARC struct-return tail recovery"),
    "arraystride" => on_off!(recover_array_stride, "Strided-induction array recovery"),
    "stackalias" => on_off!(stack_alias_deadstore, "Stack-pointer-alias dead-store hold"),
    "readonlycalls" => {
        let field = p1.trim();
        let (val, msg) = match field {
            "off" => (None, "Unguarded read-only memory at calls turned off".to_string()),
            "on" => (Some(1), "Read-only memory is unguarded at every call".to_string()),
            _ => {
                let n = field
                    .parse::<u64>()
                    .ok()
                    .filter(|_| !field.is_empty() && field.bytes().all(|b| b.is_ascii_digit()))
                    .ok_or_else(|| KunaError::parse("Must specify off, on or a decimal guard count"))?;
                (Some(n), format!("Read-only memory is unguarded at calls from {n} guards"))
            }
        };
        self.readonly_calls = val;
        Ok(msg)
    },
    "persistcalls" => {
        let field = p1.trim();
        let (val, msg) = match field {
            "off" => (None, "Persistent globals are guarded at every call".to_string()),
            "on" => (Some(1), "Persistent globals are guarded only next to their accesses".to_string()),
            _ => {
                let n = field
                    .parse::<u64>()
                    .ok()
                    .filter(|_| !field.is_empty() && field.bytes().all(|b| b.is_ascii_digit()))
                    .ok_or_else(|| KunaError::parse("Must specify off, on or a decimal guard count"))?;
                (Some(n), format!("Persistent globals are guarded only next to their accesses from {n} guards"))
            }
        };
        self.persist_calls = val;
        Ok(msg)
    },
    "stackviews" => {
        let (value, message) = crate::kuna_stackviews::OptionStackViews.apply(p1)?;
        self.stack_views = value;
        Ok(message)
    },
    "stackprobeloop" => {
        let (form, msg) = crate::kuna_stackprobeloop::parse_stack_probe_loop_form(p1)?;
        self.model_stack_probe_loop = form.model_stack_probe_loop();
        Ok(msg)
    },
    "memsetrecover" => {
        let (form, msg) = crate::kuna_memsetsequence::parse_memset_recover_form(p1)?;
        self.memset_recover = form.memset_recover();
        Ok(msg)
    },
    "rodatastring" => {
        let (form, msg) = crate::kuna_rodatastring::parse_rodata_string_form(p1)?;
        self.rodata_string = form.rodata_string();
        Ok(msg)
    },
    "switchmodbound" => on_off!(switch_modulo_bound, "Switch modulo/and-mask index bound"),
    "switchrunon" => on_off!(switch_run_on, "Jump-table run-on cut"),
    "switchrunonguard" => on_off!(switch_run_on_guard, "Jump-table run-on cut past a non-bounding guard"),
    "constselectjump" => on_off!(const_select_jump, "Constant-select indirect branch recovery"),
    "switchguardbound" => on_off!(switch_guard_bound, "Switch CBRANCH-guard index bound"),
    "highsliceguard" => on_off!(highslice_guard, "High-byte slice jump-table guard"),
    "switchsharedcase" => on_off!(switch_shared_case, "Switch loop-carried-guard table"),
    "switchmultipred" => on_off!(switch_multi_pred, "Switch multi-predecessor unrolled-guard table"),
    "unrolledguard" => on_off!(unrolled_guard, "Interleaved unrolled-guard jump-table partial-flow recovery"),
    "jtsharepartial" => on_off!(jumptable_share_partial, "Shared jump-table partial sub-decompilation"),
    "deadcascade" => {
        let field = p1.trim();
        let (val, msg) = match field {
            "off" => (None, "Dead-op removal takes one link of a dead chain per pass".to_string()),
            "on" => (Some(0), "Dead-op removal takes a dead chain of joins and guards out at once".to_string()),
            _ => {
                let n = field
                    .parse::<u32>()
                    .ok()
                    .filter(|_| !field.is_empty() && field.bytes().all(|b| b.is_ascii_digit()))
                    .ok_or_else(|| KunaError::parse("Must specify off, on or a decimal pass count"))?;
                (Some(n), format!("Dead-op removal takes a dead chain of joins and guards out at once once a pool run has made {n} passes"))
            }
        };
        self.dead_cascade = val;
        Ok(msg)
    },
    "noreturn_externmatch" => on_off!(noreturn_extern_match, "Name-matched extern no-return"),
    "loweredswitch" => {
        let (val, msg) = crate::kuna_loweredswitch::OptionLowerSwitch.apply(p1)?;
        self.recover_lowered_switch = val;
        Ok(msg)
    },
    "loweredswitchlabels" => {
        let (val, msg) = crate::kuna_loweredswitchlabels::OptionLowerSwitchLabels.apply(p1)?;
        self.lowered_switch_labels = val;
        Ok(msg)
    },
    "loweredswitchvalue" => on_off!(lowered_switch_value_check, "Lowered-switch dispatch value check"),
    "loweredswitchexact" => on_off!(lowered_switch_exact, "Exact lowered-switch recovery"),
    "loweredswitchheads" => on_off!(lowered_switch_every_head, "Lowered-switch detection from every cascade head"),
    "nulterminator" => {
        let (val, msg) =
            crate::p6_variables::kuna_nulterminator::OptionNulTerminator.apply(p1)?;
        self.nul_terminator = val;
        Ok(msg)
    },
    "endptrbound" => {
        let (val, msg) =
            crate::p6_variables::kuna_endptrbound::OptionEndPtrBound.apply(p1)?;
        self.end_ptr_bound = val;
        Ok(msg)
    },
    "castobject" => {
        let (val, msg) =
            crate::p6_variables::kuna_castobject::OptionCastObject.apply(p1)?;
        self.cast_object = val;
        Ok(msg)
    },
    "argclobber" => {
        let (val, msg) = crate::p4_calls::kuna_argclobber::OptionArgClobber.apply(p1)?;
        self.arg_clobber = val;
        Ok(msg)
    },
    "armfloatargs" => {
        self.arm_float_args = on_or_off(p1)?;
        Ok(format!("ARM floating argument recovery turned {p1}"))
    },
    "narrowargs" => {
        let (body, callers) = match p1 {
            "callers" => (true, true),
            _ => (on_or_off(p1)?, false),
        };
        self.narrow_args = body;
        self.narrow_args_callers = callers;
        Ok(format!("Narrow register parameter recovery set to {p1}"))
    },
    "regnarrow" => {
        self.narrow_locals = on_or_off(p1)?;
        Ok(format!("Narrow register local recovery turned {p1}"))
    },
    "narrowret" => {
        self.narrow_ret = on_or_off(p1)?;
        Ok(format!("Narrow call result recovery turned {p1}"))
    },
    "regsigned" => {
        self.narrow_signed = on_or_off(p1)?;
        Ok(format!("Signed narrow register local recovery turned {p1}"))
    },
    "narrowext" => {
        let (mode, msg) = crate::kuna_narrowext::OptionNarrowExt.apply(p1)?;
        if self.narrow_ext != mode {
            self.kuna_callwide.clear();
        }
        self.narrow_ext = mode;
        Ok(msg)
    },
    "armfloatreturn" => {
        self.arm_float_return = on_or_off(p1)?;
        Ok(format!("ARM floating return recovery turned {p1}"))
    },
    "passthrough" => {
        let (val, msg) = crate::p4_calls::kuna_passthrough::OptionPassThrough.apply(p1)?;
        self.pass_through = val;
        Ok(msg)
    },
    "mixedtailret" => {
        let (val, msg) = crate::p4_calls::kuna_mixedtailret::OptionMixedTailRet.apply(p1)?;
        self.mixed_tail_ret = val;
        Ok(msg)
    },
    "varargsharedfloat" => on_off!(vararg_shared_float, "variadic doubles that also feed an earlier argument"),
    "calloverlap" => {
        let (val, msg) =
            crate::p3_dataflow::kuna_calloverlap::OptionCallOverlap.apply(p1)?;
        self.call_overlap = val;
        Ok(msg)
    },
    "spillargtrial" => {
        let (val, msg) =
            crate::p4_calls::kuna_spillargtrial::OptionSpillArgTrial.apply(p1)?;
        self.spill_arg_trial = val;
        Ok(msg)
    },
    "indexaliasguard" => {
        let (val, msg) =
            crate::p3_dataflow::kuna_indexaliasguard::OptionIndexAliasGuard.apply(p1)?;
        self.index_alias_guard = val;
        Ok(msg)
    },
    "callarrayextent" => on_off!(call_array_extent, "Call-escaped frame array extent"),
    "stackstruct" => on_off!(stack_struct, "Address-taken frame record layout"),
    "arrayextent" => {
        let (val, msg) =
            crate::p6_variables::kuna_arrayextent::OptionArrayExtent.apply(p1)?;
        self.array_extent = val;
        Ok(msg)
    },
    "arraysize" => {
        let (val, msg) = crate::p6_variables::kuna_arraysize::OptionArraySize.apply(p1)?;
        self.array_size = val;
        Ok(msg)
    },
    "devectorize" => on_off!(devectorize_reduction, "Vectorized reduction re-roll"),
    "strlenscan" => on_off!(strlen_scan, "inline repnz scasb string length as strlen"),
    "regionstructure" => {
        let (val, msg) =
            crate::p8_structure::region_structurer::OptionRegionStructure.apply(p1)?;
        self.region_structure = val;
        Ok(msg)
    },
    "guardarm" => {
        let (val, msg) =
            crate::p8_structure::kuna_ifnoexit::OptionGuardArm.apply(p1)?;
        self.guard_arm = val;
        Ok(msg)
    },
    "loopcondhoist" => {
        let (val, msg) =
            crate::p8_structure::kuna_ifnoexit::OptionLoopCondHoist.apply(p1)?;
        self.loop_cond_hoist = val;
        Ok(msg)
    },
    "loopcontinue" => on_off!(loop_continue, "Secondary loop-latch continue recovery"),
    "regionlooprefine" => on_off!(
        region_loop_refine,
        "Region structurer multi-exit/irreducible loop-successor refinement"
    ),
    "regionedgeorder" => on_off!(
        region_edge_order,
        "Region structurer H2 post-dominator + dominance-tiered edge-virtualization ordering"
    ),
    "outline" => {
        let (val, msg) =
            crate::p8_structure::kuna_outline::OptionOutline.apply(p1)?;
        self.outline_spec = val;
        Ok(msg)
    },
    "condfold" => {
        let (val, msg) =
            crate::p8_structure::kuna_condfold::OptionCondFold.apply(p1)?;
        self.cond_fold = val;
        Ok(msg)
    },
    "condstmts" => {
        let (val, msg) =
            crate::p8_structure::kuna_condstmts::OptionCondStmts.apply(p1)?;
        self.cond_stmts = val;
        Ok(msg)
    },
    "gotoreduce" => {
        let (val, msg) =
            crate::p8_structure::kuna_gotoreduce::OptionGotoReduce.apply(p1)?;
        self.reduce_return_gotos = val;
        Ok(msg)
    },
    "ifelseflatten" => {
        let (val, msg) =
            crate::p8_structure::kuna_ifelseflatten::OptionIfElseFlatten.apply(p1)?;
        self.flatten_ifelse = val;
        Ok(msg)
    },
    "crossjumprevert" => {
        let (val, msg) =
            crate::p8_structure::kuna_crossjumpreverter::OptionCrossJumpReverter.apply(p1)?;
        self.revert_cross_jumps = val;
        Ok(msg)
    },
    "taildup" => {
        let (val, msg) = crate::p8_structure::kuna_taildup::OptionTailDup.apply(p1)?;
        self.dup_return_call_tails = val;
        Ok(msg)
    },
    "dedupitetail" => {
        let (val, msg) =
            crate::p8_structure::kuna_dedupitetail::OptionDedupIteTail.apply(p1)?;
        self.dedup_ite_tail = val;
        Ok(msg)
    },
    "iteregion" => {
        let (val, msg) = crate::p8_structure::kuna_iteregion::OptionIteRegion.apply(p1)?;
        self.iteregion = val;
        Ok(msg)
    },
    "iteexpr" => on_off!(iteexpr, "Computed-expression arm ?: recovery (iteregion extension)"),
    "iteboolean" => {
        let (val, msg) =
            crate::p8_structure::kuna_iteboolean::OptionIteBoolean.apply(p1)?;
        self.iteboolean = val;
        Ok(msg)
    },
    "paramcopyhoist" => {
        let (val, msg) =
            crate::p6_variables::kuna_paramcopyhoist::OptionParamCopyHoist.apply(p1)?;
        self.param_copy_hoist = val;
        Ok(msg)
    },
    "returndup" => {
        let (val, msg) =
            crate::p8_structure::kuna_returndup::OptionReturnDup.apply(p1)?;
        self.duplicate_shared_returns = val;
        Ok(msg)
    },
    "rettriangle" => {
        let (val, msg) = crate::p8_structure::kuna_rettriangle::OptionRetTriangle.apply(p1)?;
        self.returndup_triangle = val;
        Ok(msg)
    },
    "earlyreturn" => {
        let (val, msg) =
            crate::p8_structure::kuna_earlyreturn::OptionEarlyReturn.apply(p1)?;
        self.early_return = val;
        Ok(msg)
    },
    "switchreturn" => {
        let (val, msg) =
            crate::p8_structure::kuna_switchreturn::OptionSwitchReturn.apply(p1)?;
        self.switch_return = val;
        Ok(msg)
    },
    "foldcallret" => {
        let (val, msg) = crate::kuna_callretfold::OptionFoldCallRet.apply(p1)?;
        self.fold_call_returns = val;
        Ok(msg)
    },
    "indirectonly" => {
        let (val, msg) = crate::kuna_indirectonly::OptionIndirectOnly.apply(p1)?;
        self.mark_indirect_only = val;
        Ok(msg)
    },
    "impliedrefs" => {
        let (val, msg) = crate::kuna_impliedrefs::OptionImpliedRefs.apply(p1)?;
        self.max_implied_ref = val;
        Ok(msg)
    },
    "termdup" => {
        let (val, msg) = crate::kuna_impliedrefs::OptionTermDup.apply(p1)?;
        self.max_term_duplication = val;
        Ok(msg)
    },
    "jumptablemax" => {
        let field = p1.trim();
        let val = field
            .parse::<uint4>()
            .ok()
            .filter(|&v| v > 0 && field.bytes().all(|b| b.is_ascii_digit()))
            .ok_or_else(|| KunaError::parse("Must specify integer maximum"))?;
        self.max_jumptable_size = val;
        Ok(format!("Maximum jumptable size set to {val}"))
    },
    "stackguard" => on_off!(strip_stack_guard, "Stack-guard canary stripping"),
    "msvcstackguard" => {
        on_off!(strip_msvc_stack_guard, "MSVC /GS frame-cookie stripping")
    },
    "gobounds" => on_off!(go_bounds, "Go compiler-check folding"),
    "gojumpchain" => on_off!(go_jump_chain, "Early Go jump-chain normalization"),
    "rustfmtliteral" => on_off!(rust_fmt_literal, "Rust literal print macros"),
    "rustfmtargs" => on_off!(rust_fmt_args, "Rust primitive print arguments"),
    "rustfmtflags" => on_off!(rust_fmt_flags, "Rust static integer format specifications"),
    "rustfmtresult" => on_off!(rust_fmt_result, "Rust format String results"),
    "rustfmtpanic" => on_off!(rust_fmt_panic, "Rust panic formatting"),
    "rustfmtpieces" => on_off!(rust_fmt_pieces, "Rust piece-array formatting"),
    "rustfmtstr" => on_off!(rust_fmt_str, "Rust verified string Display arguments"),
    "rustfmtwrite" => on_off!(rust_fmt_write, "Rust verified String writer formatting"),
    "rustfmtio" => on_off!(rust_fmt_io, "Rust verified Stdout writer formatting"),
    "securitycheck" => {
        on_off!(strip_security_check, "Rust security-check branch stripping")
    },
    "branchflip" => on_off!(branch_flip, "Negated-guard branch flipping for linearity"),
    "litpoolconst" => {
        on_off!(litpoolconst, "In-code literal-pool constant folding")
    },
    "loopbreak_recovery" => {
        let (val, msg) =
            crate::kuna_loopbreak_recovery::OptionLoopBreakRecovery.apply(p1)?;
        self.recover_loop_break = val;
        Ok(msg)
    },
    "namestyle" => {
        let (val, msg) = crate::kuna_naming::OptionNameStyle.apply(p1)?;
        self.name_style_angr = val;
        Ok(msg)
    },
    "realtypes" => on_off!(realtypes, "Real-C-type rendering for unknowns"),
    "ctypes" => on_off!(ctypes, "valid-C core type spelling"),
    "callmechanisms" => {
        let (mode, msg) = crate::kuna_callmechanisms::OptionCallMechanisms.apply(p1)?;
        self.call_mechanism_mode = mode;
        Ok(msg)
    },
    "voidtailreturn" => on_off!(voidtailreturn, "void tail-return elision"),
    "importdata" => on_off!(import_data, "import-slot data-read rendering"),
    "castimplied" => on_off!(cast_implied, "C-implied cast elision"),
    "castsign" => on_off!(cast_sign, "signed declarations for signed-only locals"),
    "castternary" => on_off!(cast_ternary, "conditional-arm cast elision"),
    "callrettype" => on_off!(call_ret_type, "callee-stated call return types"),
    "castnarrow" => on_off!(cast_narrow, "implied narrowing-cast elision"),
    "libcnames" => on_off!(libc_names, "library-call local names"),
    "declorder" => {
        let (mode, msg) = crate::kuna_declorder::OptionDeclOrder.apply(p1)?;
        self.decl_order = mode;
        Ok(msg)
    },
    "castwiden" => {
        let (mode, msg) = crate::kuna_castwiden::OptionCastWiden.apply(p1)?;
        self.cast_widen = mode;
        Ok(msg)
    },
    "ptrdepthcap" => on_off!(ptrdepthcap, "inferred pointer-nesting cap"),
    "calltargettype" => on_off!(call_target_type, "indirect-call target types"),
    "floatglobals" => on_off!(float_globals, "float typing of globals moved only through float registers"),
    "boolbyte" => on_off!(bool_byte, "truth-valued byte typing"),
    "charbyte" => on_off!(char_byte, "char-pointer byte typing"),
    "castarith" => on_off!(cast_arith, "pointer arithmetic in pointer terms"),
    "castindex" => on_off!(cast_index, "variable indexes and pointer differences in pointer terms"),
    "ptrfromuse" => {
        let (val, msg) =
            crate::p5_types::kuna_ptrfromuse::OptionPtrFromUse.apply(p1)?;
        self.ptr_from_use = val;
        Ok(msg)
    },
    "charptr" => on_off!(char_ptr, "character-pointer evidence"),
    "elemptr" => {
        let (val, msg) = crate::p5_types::kuna_elemptr::OptionElemPtr.apply(p1)?;
        self.elem_ptr = val;
        Ok(msg)
    },
    "protoorder" => {
        let (mode, msg) = crate::kuna_protoorder::OptionProtoOrder.apply(p1)?;
        self.protoorder = mode;
        Ok(msg)
    },
    "calleevote" => {
        let (mode, msg) = crate::kuna_calleevote::OptionCalleeVote.apply(p1)?;
        self.calleevote = mode;
        Ok(msg)
    },
    "callerptr" => on_off!(caller_ptr, "Caller pointer-class parameter typing"),
    "callbacktype" => {
        let (mode, msg) = crate::kuna_callbacktype::OptionCallbackType.apply(p1)?;
        self.callbacktype = mode;
        Ok(msg)
    },
    "cortexmpriv" => on_off!(cortexmpriv, "Cortex-M privileged-mode guard folding"),
    "dedupvardecls" => {
        let (val, msg) = crate::kuna_dedupvardecls::OptionDedupVarDecls.apply(p1)?;
        self.dedup_var_decls = val;
        Ok(msg)
    },
    "noreturn_known" => on_off!(analysis_noreturn_known, "No-return-known analysis pass"),
    "peimportcall" => on_off!(analysis_peimportcall, "PE/Mach-O import-slot call binding"),
    "authptrcall" => on_off!(analysis_authptrcall, "arm64e __auth_ptr call binding"),
    "libproto" => on_off!(analysis_libproto, "Library-prototype analysis pass"),
    "libcsigs" => on_off!(analysis_libcsigs, "Measured libc signature extension"),
    "libgccsigs" => on_off!(analysis_libgccsigs, "libgcc __divdi3 DI-mode contract"),
    "libctypes" => {
        use crate::kuna_libctypes::LibcTypesLayout;
        let layout = match p1.trim().to_ascii_lowercase().as_str() {
            "opaque" | "on" | "1" | "true" => LibcTypesLayout::Opaque,
            "glibc" => LibcTypesLayout::Glibc,
            "off" | "0" | "false" => LibcTypesLayout::Off,
            other => {
                return Err(KunaError::lowlevel(format!(
                    "libctypes: expected `off`, `opaque` or `glibc`, got `{other}`"
                )))
            }
        };
        self.analysis_libctypes = layout != LibcTypesLayout::Off;
        self.analysis_libctypes_glibc = layout == LibcTypesLayout::Glibc;
        crate::kuna_libctypes::set_libctypes_env_layout(layout);
        Ok(format!(
            "Named libc aggregate types turned {}",
            match layout {
                LibcTypesLayout::Off => "off",
                LibcTypesLayout::Opaque => "on (opaque)",
                LibcTypesLayout::Glibc => "on (glibc layouts)",
            }
        ))
    },
    "win32sigs" => on_off!(analysis_win32sigs, "Built-in Win32 API signature table"),
    "declaredlibcproto" => {
        on_off!(analysis_declaredlibcproto, "Declared-name libc prototype lookup")
    },
    "strings" => on_off!(analysis_strings, "String-literal analysis pass"),
    "widestrings" => {
        on_off!(analysis_widestrings, "UTF-16LE width of the string-literal pass")
    },
    "widestrings32" => {
        on_off!(analysis_widestrings32, "UTF-32 width of the string-literal pass")
    },
    "entry_disc" => on_off!(analysis_entry_disc, "Entry-discovery analysis pass"),
    "unmappedentry" => {
        on_off!(analysis_unmappedentry, "Unmapped-CALL-target entry suppression")
    },
    "thumbswitch" => {
        on_off!(analysis_thumbswitch, "Thumb tbb/tbh switch-table read in the Listing walk (GH-299)")
    },
    "x86listingswitch" => {
        on_off!(analysis_x86listingswitch, "Bounded i386 switch successors in the discovery Listing")
    },
    "switchreach" => {
        on_off!(analysis_switchreach, "Register-chain jump-table reach in the reference walk")
    },
    "thunkentry" => on_off!(analysis_thunkentry, "Jump-thunk target function entries"),
    "addrtakenentry" => {
        on_off!(analysis_addrtakenentry, "Address-taken code function entries")
    },
    "ppclocalentry" => {
        on_off!(analysis_ppclocalentry, "PPC64 ELFv2 local-entry entry suppression")
    },
    "gowalkprune" => {
        on_off!(analysis_gowalkprune, "Go pclntab functions out of the fast inventory walk")
    },
    "flowmode" => {
        let (on, after_call) = match p1.trim().to_ascii_lowercase().as_str() {
            "on" | "1" | "true" => (true, false),
            "aftercall" => (true, true),
            "off" | "0" | "false" => (false, false),
            other => {
                return Err(KunaError::lowlevel(format!(
                    "flowmode: expected `on`, `aftercall` or `off`, got `{other}`"
                )))
            }
        };
        self.analysis_flowmode = on;
        self.analysis_flowmode_aftercall = after_call;
        Ok(format!(
            "ARM flow-proven decode-mode paints turned {}",
            match (on, after_call) {
                (false, _) => "off",
                (true, false) => "on",
                (true, true) => "on, past calls proven to return",
            }
        ))
    },
    "picbase" => {
        on_off!(analysis_picbase, "PIC base-register folding in the xref index")
    },
    "entrymainproto" => {
        on_off!(analysis_entrymainproto, "PE CRT entry-function prototype recovery")
    },
    "machomain" => {
        on_off!(analysis_machomain, "Mach-O LC_MAIN entry naming + prototype")
    },
    "pemain" => {
        on_off!(analysis_pemain, "PE CRT user-entry naming")
    },
    "armlibcmain" => {
        on_off!(analysis_armlibcmain, "non-PIE ARM crt1 _start->main recovery")
    },
    "elfmain" => {
        on_off!(analysis_elfmain, "ELF libc-start main naming + prototype")
    },
    "eh_frame_full" => {
        on_off!(analysis_eh_frame_full, ".eh_frame LSDA landing-pad discovery")
    },
    "unwindstarts" => {
        on_off!(analysis_unwindstarts, "Mach-O __unwind_info function-start discovery")
    },
    "coldentry" => {
        on_off!(analysis_coldentry, "Multi-entry .cold fragment entry discovery")
    },
    "fdeinterior" => {
        on_off!(analysis_fdeinterior, ".eh_frame FDE-interior entry suppression")
    },
    "pdatainterior" => {
        on_off!(analysis_pdatainterior, ".pdata RUNTIME_FUNCTION-interior entry suppression")
    },
    "pdbinterior" => {
        on_off!(analysis_pdbinterior, "PDB procedure-interior entry suppression")
    },
    "funcstart_patterns" => {
        on_off!(analysis_funcstart_patterns, "Full byte-pattern function-start pass")
    },
    "armframes" => {
        let (inventory, full) = if p1 == "inventory" {
            (true, false)
        } else {
            let on = on_or_off(p1)?;
            (on, on)
        };
        self.analysis_armframes = inventory;
        self.analysis_armframes_full = full;
        Ok(format!("Validated ARM frame recovery set to {p1}"))
    },
    "cortexmvectors" => {
        on_off!(analysis_cortexmvectors, "Widened ARM Cortex-M vector-table signature")
    },
    "ptrentry" => {
        on_off!(analysis_ptrentry, "Pointer-referenced ARM function entries")
    },
    "poolentry" => {
        on_off!(
            analysis_poolentry,
            "ARM literal-pool inference (entry recall + AIF phantom suppression)"
        )
    },
    "arm_markers" => on_off!(analysis_arm_markers, "ARM/Thumb decode-mode marker pass"),
    "armfuncmode" => on_off!(analysis_armfuncmode, "A32 mode at even ARM function symbols"),
    "armwalkmode" => on_off!(analysis_armwalkmode, "ARM decode mode carried along the Listing walk"),
    "armpltslot" => on_off!(analysis_armpltslot, "ARM PLT stub slot reference"),
    "armcondskip" => on_off!(analysis_armcondskip, "ARM condition skips filed as no jump"),
    "armderef" => on_off!(analysis_armderef, "ARM accesses through a register holding a datum's address"),
    "entrythumbflow" => {
        on_off!(analysis_entrythumbflow, "Entry-reachable Thumb context walk")
    },
    "mips_gp" => on_off!(analysis_mips_gp, "MIPS $gp-recovery (t9 tracking) pass"),
    "ppc32picimports" => {
        let mode = crate::kuna_ppc32picimports::Mode::parse(p1)
            .ok_or_else(|| KunaError::parse("Expected ppc32picimports off|on|abi"))?;
        self.analysis_ppc32picimports = mode;
        mode.set_env();
        Ok(format!("PPC32 PIC import recovery {}", mode.as_str()))
    },
    "i386_pie_plt" => {
        let val = on_or_off(p1)?;
        self.analysis_i386_pie_plt = val;
        crate::kuna_i386_pie_plt::set_i386_pie_plt_env(val);
        Ok(format!(
            "i386-PIE PLT-stub decode turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "ppcapu" => {
        let val = on_or_off(p1)?;
        self.analysis_ppcapu = val;
        crate::kuna_ppcapu::set_ppcapu_env(val);
        Ok(format!(
            "PowerPC APU-note language selection turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "ifuncfpret" => {
        let val = on_or_off(p1)?;
        self.analysis_ifuncfpret = val;
        crate::kuna_ifuncfpret::set_ifuncfpret_env(val);
        Ok(format!(
            "IFUNC PLT-stub naming turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "relocrebase" => {
        let val = on_or_off(p1)?;
        self.analysis_relocrebase = val;
        crate::kuna_relocrebase::set_relocrebase_env(val);
        Ok(format!(
            "relocatable-object analysis rebase turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "kernelcache" => {
        let (choice, msg) = crate::kuna_kernelcache::parse_kernelcache(p1)?;
        crate::kuna_kernelcache::set_kernelcache_env(&choice);
        Ok(msg)
    },
    "dynrelocs" => {
        let val = on_or_off(p1)?;
        self.analysis_dynrelocs = val;
        crate::kuna_dynrelocs::set_dynrelocs_env(val);
        Ok(format!(
            "linked-image dynamic-relocation application turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "pdatachained" => {
        let val = on_or_off(p1)?;
        self.analysis_pdatachained = val;
        crate::kuna_pdatachained::set_pdatachained_env(val);
        Ok(format!(
            "PE chained-UNWIND_INFO .pdata entry suppression turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "rexthunk" => {
        let val = on_or_off(p1)?;
        self.analysis_rexthunk = val;
        crate::kuna_rexthunk::set_rexthunk_env(val);
        Ok(format!(
            "x86-64 PE REX-prefixed import-thunk rejection turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "peordinal" => {
        let val = on_or_off(p1)?;
        self.analysis_peordinal = val;
        crate::kuna_peordinal::set_peordinal_env(val);
        Ok(format!(
            "PE import-by-ordinal naming turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "symbolnamerepair" => {
        let val = on_or_off(p1)?;
        self.analysis_symbolnamerepair = val;
        crate::kuna_symbolnamerepair::set_symbolnamerepair_env(val);
        Ok(format!(
            "degenerate-symbol-name repair turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "dyldimage" => Ok(crate::kuna_dyldcache::set_dyldimage(p1)),
    "dyldcrossrefs" => crate::kuna_dyldcache::set_dyldcrossrefs(p1),
    "swiftdemangle" => {
        let val = on_or_off(p1)?;
        self.analysis_swiftdemangle = val;
        crate::kuna_swiftdemangle::set_swiftdemangle_env(val);
        Ok(format!("Swift symbol demangling turned {}", if val { "on" } else { "off" }))
    },
    "symbolnamechars" => {
        let mode = crate::kuna_symbolnamechars::NameChars::parse(p1).ok_or_else(|| {
            KunaError::lowlevel(format!(
                "symbolnamechars must be off|safe|ident, got `{p1}`"
            ))
        })?;
        self.analysis_symbolnamechars = mode;
        crate::kuna_symbolnamechars::set_symbolnamechars_env(mode);
        Ok(format!("symbol-name character sanitizing set to {}", mode.as_str()))
    },
    "symbolnamebound" => {
        let (bound, msg) = crate::kuna_symbolnamebound::parse_symbolnamebound(p1)?;
        self.analysis_symbolnamebound = bound;
        crate::kuna_symbolnamebound::set_symbolnamebound_env(bound);
        Ok(msg)
    },
    "msvcfpconst" => {
        let val = on_or_off(p1)?;
        self.analysis_msvcfpconst = val;
        crate::kuna_msvcfpconst::set_msvcfpconst_env(val);
        Ok(format!(
            "MSVC __real@ FP-constant recovery turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "mips_isa" => on_off!(analysis_mips_isa, "MIPS16 ISA_MODE decode-mode marker pass"),
    "dwarf" => on_off!(analysis_dwarf, "DWARF recovery analysis pass"),
    "datasyms" => {
        on_off!(analysis_datasyms, "ELF data-symbol (STT_OBJECT) naming")
    },
    "dwarf_lines" => {
        on_off!(analysis_dwarf_lines, "DWARF .debug_line source-line comment pass")
    },
    "cppproto" => {
        on_off!(analysis_cppproto, "DWARF C++ prototype recovery arm")
    },
    "godwarfsig" => on_off!(analysis_godwarfsig, "Go DWARF scalar and pointer signatures"),
    "gopairreturns" => on_off!(analysis_gopairreturns, "Go DWARF paired integer results"),
    "cppsig" => {
        let (mode, msg) = crate::kuna_cppsig::parse_cppsig_mode(p1)?;
        self.analysis_cppsig = mode;
        Ok(msg)
    },
    "msvcsig" => on_off!(analysis_msvcsig, "MSVC declaration arm of the demangled signatures"),
    "typedepth" => {
        let val = on_or_off(p1)?;
        self.analysis_typedepth = val;
        crate::kuna_typedepth::set_typedepth_env(val);
        Ok(format!(
            "Full-depth DWARF type resolution turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "dwarfstructs" => {
        let val = on_or_off(p1)?;
        self.analysis_dwarfstructs = val;
        crate::kuna_dwarfstructs::set_dwarfstructs_env(val);
        Ok(format!(
            "DWARF aggregate-layout import turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "variantnames" => {
        let val = on_or_off(p1)?;
        self.analysis_variantnames = val;
        crate::kuna_variantnames::set_variantnames_env(val);
        Ok(format!(
            "Variant discriminant-value naming turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "dwarfvariants" => {
        let val = on_or_off(p1)?;
        self.analysis_dwarfvariants = val;
        crate::kuna_dwarfvariants::set_dwarfvariants_env(val);
        Ok(format!(
            "DWARF variant-part import turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "callfixup" => on_off!(analysis_callfixup, "Call-fixup analysis pass"),
    "addrtable" => on_off!(analysis_addrtable, "Address-table analysis pass"),
    "operand_refs" => on_off!(analysis_operand_refs, "Scalar/operand reference-markup pass"),
    "formatstring" => {
        let (mode, msg) = crate::kuna_formatstring::parse_formatstring_mode(p1)?;
        self.analysis_formatstring = mode;
        Ok(msg)
    },
    "listing" => on_off!(analysis_listing, "Listing/xref disassembly tier"),
    "fast_funcdisc" => {
        on_off!(analysis_fast_funcdisc, "Fast whole-project function discovery")
    },
    "lazyfuncs" => on_off!(
        analysis_lazyfuncs,
        "Materialize Function objects on demand instead of one per entry"
    ),
    "rawdiscover" => {
        on_off!(analysis_rawdiscover, "Raw-image recursive function discovery")
    },
    "mzsweep" => {
        on_off!(analysis_mzsweep, "MS-DOS executable call-sweep evidence")
    },
    "noreturn_disc" => {
        on_off!(analysis_noreturn_disc, "Discovered-no-return Listing consumer")
    },
    "noreturn_discstrict" => {
        on_off!(
            analysis_noreturn_discstrict,
            "Discovered-no-return positive-evidence-only tally"
        )
    },
    "noreturn_propagate" => {
        on_off!(analysis_noreturn_propagate, "No-return propagation Listing consumer")
    },
    "noreturn_error" => {
        on_off!(analysis_noreturn_error, "error(nonzero,...) conditional no-return recognizer")
    },
    "noreturn_reach" => {
        on_off!(analysis_noreturn_reach, "CFG-reachability no-return rule (Ghidra targetOnlyCallsNoReturn)")
    },
    "fid" => on_off!(analysis_fid, "FID fingerprint matcher Listing consumer"),
    "rtti" => on_off!(analysis_rtti, "MSVC RTTI / vftable class-name recovery pass"),
    "itaniumrtti" => {
        on_off!(analysis_itaniumrtti, "Itanium (GCC/Clang) RTTI / vtable recovery pass")
    },
    "aif" => {
        on_off!(analysis_aif, "Aggressive Instruction Finder gap-walk Listing consumer")
    },
    "aifstrict" => {
        on_off!(analysis_aifstrict, "AIF gap-cursor aligned slide (GH-299)")
    },
    "aifcorroborate" => {
        on_off!(analysis_aifcorroborate, "AIF accept corroboration test (GH-313)")
    },
    "aifbracket" => {
        on_off!(analysis_aifbracket, "AIF bracketed-candidate reject (GH-299)")
    },
    "aifnoppad" => {
        on_off!(analysis_aifnoppad, "AIF padding/zero-fill candidate reject (GH-299)")
    },
    "aifpoolslide" => {
        on_off!(analysis_aifpoolslide, "AIF literal-pool candidate slide (GH-299)")
    },
    "tailcallentry" => {
        on_off!(analysis_tailcallentry, "Tail-call function-entry recovery Listing consumer")
    },
    "gopclntab" => {
        on_off!(analysis_gopclntab, "Go pclntab function-name recovery pass")
    },
    "gotypes" => {
        let val = on_or_off(p1)?;
        self.analysis_gotypes = val;
        crate::kuna_gotypes::set_gotypes_env(val);
        Ok(format!("Go runtime-type harvest turned {}", if val { "on" } else { "off" }))
    },
    "gonoreturn" => {
        on_off!(analysis_gonoreturn, "Go pclntab no-return list pass")
    },
    "objc" => on_off!(analysis_objc, "Mach-O Objective-C metadata recovery pass"),
    "objcrefconst" => on_off!(analysis_objcrefconst, "Objective-C reference-slot folding"),
    "swiftmetadata" => on_off!(analysis_swiftmetadata, "Swift runtime-metadata recovery pass"),
    "cfstrings" => {
        let (pass, annotate) = match p1 {
            "annotate" => (true, true),
            other => (on_or_off(other)?, false),
        };
        self.analysis_cfstrings = pass;
        self.cfstrings_annotate = annotate;
        let state = if annotate { "on, annotated" } else if pass { "on" } else { "off" };
        Ok(format!("Mach-O __cfstring constant-string recovery pass turned {state}"))
    },
    "swifttypes" => on_off!(analysis_swifttypes, "Swift type layouts from static metadata"),
    "objcivars" => {
        let val = on_or_off(p1)?;
        self.analysis_objcivars = val;
        crate::kuna_objcivars::set_objcivars_env(val);
        Ok(format!("Objective-C instance layouts turned {}", if val { "on" } else { "off" }))
    },
    "swiftcall" => on_off!(analysis_swiftcall, "Swift calling convention on methods and witnesses"),
    "objcsdk" => {
        let path = crate::kuna_objcsdk::parse_option_value(p1).map_err(KunaError::parse)?;
        crate::kuna_objcsdk::set_objcsdk_env(path.as_deref().and_then(|p| p.to_str()));
        Ok(match path {
            Some(p) => format!("Objective-C signatures read from {}", p.display()),
            None => "Objective-C SDK headers off".to_string(),
        })
    },
    "pdb" => on_off!(analysis_pdb, "PE PDB metadata recovery pass"),
    "relocobjects" => {
        let val = on_or_off(p1)?;
        std::env::set_var(
            crate::options::RELOC_OBJECTS_ENV,
            if val { "1" } else { "0" },
        );
        Ok(format!(
            "ET_REL relocatable-object loading turned {}",
            if val { "on" } else { "off" }
        ))
    },
    "macho-arm64e" => on_off!(macho_arm64e, "Mach-O arm64e Apple-Silicon spec selection"),
}
