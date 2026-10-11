// Shared integration-test executable. Process-sensitive tests stay separate.
mod common;

#[path = "setup_commands.rs"]
mod setup_commands;

#[path = "verify_aarch64_plt.rs"]
mod verify_aarch64_plt;

#[path = "verify_aif.rs"]
mod verify_aif;

#[path = "verify_aifcorroborate.rs"]
mod verify_aifcorroborate;

#[path = "verify_aifstrict.rs"]
mod verify_aifstrict;

#[path = "verify_arm_context.rs"]
mod verify_arm_context;

#[path = "verify_arm_pe_context.rs"]
mod verify_arm_pe_context;

#[path = "verify_arm_thumb_decode.rs"]
mod verify_arm_thumb_decode;

#[path = "verify_arm_xref_roots.rs"]
mod verify_arm_xref_roots;

#[path = "verify_armdiscseed.rs"]
mod verify_armdiscseed;

#[path = "verify_armlibcmain.rs"]
mod verify_armlibcmain;

#[path = "verify_assertflow.rs"]
mod verify_assertflow;

#[path = "verify_assertplane.rs"]
mod verify_assertplane;

#[path = "verify_assertranges.rs"]
mod verify_assertranges;

#[path = "verify_buildstamp.rs"]
mod verify_buildstamp;

#[path = "verify_bytehonest.rs"]
mod verify_bytehonest;

#[path = "verify_byteoverlay.rs"]
mod verify_byteoverlay;

#[path = "verify_coff_comdat.rs"]
mod verify_coff_comdat;

#[path = "verify_coff_object.rs"]
mod verify_coff_object;

#[path = "verify_coldentry.rs"]
mod verify_coldentry;

#[path = "verify_cortexmvectors.rs"]
mod verify_cortexmvectors;

#[path = "verify_cover_miscompile.rs"]
mod verify_cover_miscompile;

#[path = "verify_cppcallnames.rs"]
mod verify_cppcallnames;

#[path = "verify_cppproto.rs"]
mod verify_cppproto;

#[path = "verify_cppsig.rs"]
mod verify_cppsig;

#[path = "verify_crossarch_entry_main.rs"]
mod verify_crossarch_entry_main;

#[path = "verify_cxx_throw_noreturn.rs"]
mod verify_cxx_throw_noreturn;

#[path = "verify_data_global_symbols.rs"]
mod verify_data_global_symbols;

#[path = "verify_dataorg_sizes.rs"]
mod verify_dataorg_sizes;

#[path = "verify_declared_entry_batch.rs"]
mod verify_declared_entry_batch;

#[path = "verify_declared_vars.rs"]
mod verify_declared_vars;

#[path = "verify_declaredlibcproto.rs"]
mod verify_declaredlibcproto;

#[path = "verify_decode_engine.rs"]
mod verify_decode_engine;

#[path = "verify_decompile_all.rs"]
mod verify_decompile_all;

#[path = "verify_decompile_all_parity.rs"]
mod verify_decompile_all_parity;

#[path = "verify_decompile_project_helpers.rs"]
mod verify_decompile_project_helpers;

#[path = "verify_direction_flag.rs"]
mod verify_direction_flag;

#[path = "verify_dwarf_enums.rs"]
mod verify_dwarf_enums;

#[path = "verify_dwarf_prototypes.rs"]
mod verify_dwarf_prototypes;

#[path = "verify_eh_frame_full.rs"]
mod verify_eh_frame_full;

#[path = "verify_elf_language_ids.rs"]
mod verify_elf_language_ids;

#[path = "verify_entry_selectors.rs"]
mod verify_entry_selectors;

#[path = "verify_entrymainproto.rs"]
mod verify_entrymainproto;

#[path = "verify_entrythumbflow_pe.rs"]
mod verify_entrythumbflow_pe;

#[path = "verify_et_rel_sparc_calls.rs"]
mod verify_et_rel_sparc_calls;

#[path = "verify_et_rel_status_return.rs"]
mod verify_et_rel_status_return;

#[path = "verify_external_entries.rs"]
mod verify_external_entries;

#[path = "verify_fdeinterior.rs"]
mod verify_fdeinterior;

#[path = "verify_fid_build.rs"]
mod verify_fid_build;

#[path = "verify_floatglobals.rs"]
mod verify_floatglobals;

#[path = "verify_flowreuse.rs"]
mod verify_flowreuse;

#[path = "verify_formatstring_crossarch.rs"]
mod verify_formatstring_crossarch;

#[path = "verify_framelayout.rs"]
mod verify_framelayout;

#[path = "verify_funcbounds.rs"]
mod verify_funcbounds;

#[path = "verify_funcstart_patterns.rs"]
mod verify_funcstart_patterns;

#[path = "verify_global_regmerge.rs"]
mod verify_global_regmerge;

#[path = "verify_go_pclntab.rs"]
mod verify_go_pclntab;

#[path = "verify_hostile_symbol_sizes.rs"]
mod verify_hostile_symbol_sizes;

#[path = "verify_i386_pie_plt.rs"]
mod verify_i386_pie_plt;

#[path = "verify_iatcall.rs"]
mod verify_iatcall;

#[path = "verify_indirect_callers.rs"]
mod verify_indirect_callers;

#[path = "verify_itaniumrtti.rs"]
mod verify_itaniumrtti;

#[path = "verify_libc_proto_addresses.rs"]
mod verify_libc_proto_addresses;

#[path = "verify_libcsigs.rs"]
mod verify_libcsigs;

#[path = "verify_listing_context.rs"]
mod verify_listing_context;

#[path = "verify_listing_core.rs"]
mod verify_listing_core;

#[path = "verify_listing_fallthrough.rs"]
mod verify_listing_fallthrough;

#[path = "verify_listing_parity.rs"]
mod verify_listing_parity;

#[path = "verify_listing_queries.rs"]
mod verify_listing_queries;

#[path = "verify_litpoolconst.rs"]
mod verify_litpoolconst;

#[path = "verify_loader_data_symbols.rs"]
mod verify_loader_data_symbols;

#[path = "verify_macho_import_slots.rs"]
mod verify_macho_import_slots;

#[path = "verify_macho_imports.rs"]
mod verify_macho_imports;

#[path = "verify_machomain.rs"]
mod verify_machomain;

#[path = "verify_mappedflowboundary.rs"]
mod verify_mappedflowboundary;

#[path = "verify_midstring_literals.rs"]
mod verify_midstring_literals;

#[path = "verify_mips16_isa.rs"]
mod verify_mips16_isa;

#[path = "verify_mips_plt.rs"]
mod verify_mips_plt;

#[path = "verify_msvc_demangle.rs"]
mod verify_msvc_demangle;

#[path = "verify_multiformat_dwarf.rs"]
mod verify_multiformat_dwarf;

#[path = "verify_multiformat_entry.rs"]
mod verify_multiformat_entry;

#[path = "verify_multiformat_passes.rs"]
mod verify_multiformat_passes;

#[path = "verify_multiformat_sourcelang.rs"]
mod verify_multiformat_sourcelang;

#[path = "verify_noreturn_demangle.rs"]
mod verify_noreturn_demangle;

#[path = "verify_noreturn_disc.rs"]
mod verify_noreturn_disc;

#[path = "verify_noreturn_discstrict.rs"]
mod verify_noreturn_discstrict;

#[path = "verify_noreturn_error.rs"]
mod verify_noreturn_error;

#[path = "verify_noreturn_propagate.rs"]
mod verify_noreturn_propagate;

#[path = "verify_objc.rs"]
mod verify_objc;

#[path = "verify_object_formats.rs"]
mod verify_object_formats;

#[path = "verify_operand_refs.rs"]
mod verify_operand_refs;

#[path = "verify_outlang_rust_syntax.rs"]
mod verify_outlang_rust_syntax;

#[path = "verify_pdbinterior.rs"]
mod verify_pdbinterior;

#[path = "verify_pdecode.rs"]
mod verify_pdecode;

#[path = "verify_pe_import_readonly.rs"]
mod verify_pe_import_readonly;

#[path = "verify_pe_imports.rs"]
mod verify_pe_imports;

#[path = "verify_pebnames.rs"]
mod verify_pebnames;

#[path = "verify_picbase.rs"]
mod verify_picbase;

#[path = "verify_picpool.rs"]
mod verify_picpool;

#[path = "verify_poolentry.rs"]
mod verify_poolentry;

#[path = "verify_poolref.rs"]
mod verify_poolref;

#[path = "verify_ppc64_plt.rs"]
mod verify_ppc64_plt;

#[path = "verify_ppclocalentry.rs"]
mod verify_ppclocalentry;

#[path = "verify_ptrarray_declarators.rs"]
mod verify_ptrarray_declarators;

#[path = "verify_ptrentry.rs"]
mod verify_ptrentry;

#[path = "verify_raw_image.rs"]
mod verify_raw_image;

#[path = "verify_relocrebase.rs"]
mod verify_relocrebase;

#[path = "verify_retcallchain.rs"]
mod verify_retcallchain;

#[path = "verify_return_uncomputed.rs"]
mod verify_return_uncomputed;

#[path = "verify_riscv64_plt.rs"]
mod verify_riscv64_plt;

#[path = "verify_rtti.rs"]
mod verify_rtti;

#[path = "verify_rustabi_pair.rs"]
mod verify_rustabi_pair;

#[path = "verify_s1_callfixup.rs"]
mod verify_s1_callfixup;

#[path = "verify_s1_dwarf.rs"]
mod verify_s1_dwarf;

#[path = "verify_s1_entry.rs"]
mod verify_s1_entry;

#[path = "verify_s1_formatstring.rs"]
mod verify_s1_formatstring;

#[path = "verify_s1_strings.rs"]
mod verify_s1_strings;

#[path = "verify_sectionlessextent.rs"]
mod verify_sectionlessextent;

#[path = "verify_short_utf16_window.rs"]
mod verify_short_utf16_window;

#[path = "verify_slotptr.rs"]
mod verify_slotptr;

#[path = "verify_sparc_plt.rs"]
mod verify_sparc_plt;

#[path = "verify_srcmap.rs"]
mod verify_srcmap;

#[path = "verify_switchtable.rs"]
mod verify_switchtable;

#[path = "verify_symbolstream.rs"]
mod verify_symbolstream;

#[path = "verify_tailcallentry.rs"]
mod verify_tailcallentry;

#[path = "verify_tailcallframe.rs"]
mod verify_tailcallframe;

#[path = "verify_te_image.rs"]
mod verify_te_image;

#[path = "verify_typed_macho_tail.rs"]
mod verify_typed_macho_tail;

#[path = "verify_unmappedbranch.rs"]
mod verify_unmappedbranch;

#[path = "verify_unmappedentry.rs"]
mod verify_unmappedentry;

#[path = "verify_varsources.rs"]
mod verify_varsources;

#[path = "verify_w10_partial_types_console.rs"]
mod verify_w10_partial_types_console;

#[path = "verify_w10_union_truncation.rs"]
mod verify_w10_union_truncation;

#[path = "verify_w11_elf_loader.rs"]
mod verify_w11_elf_loader;

#[path = "verify_w11_elf_plt_names.rs"]
mod verify_w11_elf_plt_names;

#[path = "verify_widestrings.rs"]
mod verify_widestrings;

#[path = "wrapped_stack_encode.rs"]
mod wrapped_stack_encode;
