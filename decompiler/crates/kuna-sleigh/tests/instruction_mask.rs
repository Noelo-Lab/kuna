//! Unit gate for `Sleigh::instruction_mask` (the FID / AIF fingerprinting
//! prerequisite, PR1).  Decodes a handful of known x86-64 instructions against
//! the built `x86-64.sla` and asserts the returned `fixed_mask` (a) zeroes the
//! operand bytes — the imm32 of an `add $imm,reg` / `mov $imm,reg`, the disp32
//! of a `mov reg,[base+disp32]` — and (b) pins the opcode bits; plus that a
//! register-operand instruction classifies an `OpObject::Register` and an
//! immediate-operand instruction an `OpObject::Scalar`.
//!
//! Expected masks are hand-checked against the encodings (verified with
//! `as --64` / `objdump -d -M intel`):
//!   add eax, 0x12345678        = 05 78 56 34 12      (opcode 05 + imm32)
//!   mov ecx, 0x11223344        = b9 44 33 22 11      (b8+rd opcode + imm32)
//!   add eax, ecx               = 01 c8               (opcode 01 + modrm c8)
//!   mov eax, [rbx+0x12345678]  = 8b 83 78 56 34 12   (opcode 8b + modrm 83 + disp32)

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::{OpObject, Sleigh};
use kuna_sleigh::translate::{AssemblyEmit, Translate};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// A trivial in-memory load image: bytes at a fixed VMA, zero elsewhere.
struct MemImg {
    vma: u64,
    bytes: Vec<u8>,
}

impl LoadImage for MemImg {
    fn get_file_name(&self) -> &str {
        "memimg"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let base = addr.get_offset();
        for (i, b) in ptr.iter_mut().enumerate() {
            let a = base.wrapping_add(i as u64);
            *b = if a >= self.vma && (a - self.vma) < self.bytes.len() as u64 {
                self.bytes[(a - self.vma) as usize]
            } else {
                0
            };
        }
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

struct MissingImg;

impl LoadImage for MissingImg {
    fn get_file_name(&self) -> &str {
        "missing"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("missing"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

#[derive(Default)]
struct TextEmit {
    mnemonic: String,
    body: String,
}

impl AssemblyEmit for TextEmit {
    fn dump(&mut self, _addr: &Address, mnemonic: &str, body: &str) {
        self.mnemonic.clear();
        self.mnemonic.push_str(mnemonic);
        self.body.clear();
        self.body.push_str(body);
    }
}

/// The full x86-64 context block the language needs to decode 64-bit code
/// (copied verbatim from the `floatprint` golden-lift fixture's context).
const X86_64_CONTEXT: &[(&str, u32)] = &[
    ("addrsize", 2),
    ("bit64", 1),
    ("evexAAA", 0),
    ("evexB", 0),
    ("evexBType", 0),
    ("evexD8Type", 0),
    ("evexDisp8", 0),
    ("evexL", 0),
    ("evexLp", 0),
    ("evexRp", 0),
    ("evexTType", 0),
    ("evexV5", 0),
    ("evexVp", 0),
    ("evexZ", 0),
    ("highseg", 0),
    ("instrPhase", 0),
    ("lockprefx", 0),
    ("longMode", 1),
    ("mandover", 0),
    ("opsize", 1),
    ("prefix_66", 0),
    ("prefix_f2", 0),
    ("prefix_f3", 0),
    ("protectedMode", 0),
    ("repneprefx", 0),
    ("repprefx", 0),
    ("reserved", 0),
    ("reservedHigh", 0),
    ("rexBprefix", 0),
    ("rexRprefix", 0),
    ("rexWRXBprefix", 0),
    ("rexWprefix", 0),
    ("rexXprefix", 0),
    ("rexprefix", 0),
    ("segover", 0),
    ("suffix3D", 0),
    ("vexHighV", 0),
    ("vexL", 0),
    ("vexMMMMM", 0),
    ("vexMode", 0),
    ("vexVVVV", 0),
    ("xacquireprefx", 0),
    ("xreleaseprefx", 0),
];

const VMA: u64 = 0x401000;

fn build_sleigh(code: &[u8], language: &str) -> Sleigh {
    let path = repo_root().join("specs/Ghidra/Processors").join(language);
    let sla = std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let ctx = Box::new(ContextInternal::new());
    let img = MemImg { vma: VMA, bytes: code.to_vec() };
    let mut sleigh = Sleigh::new(Box::new(img), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize from sla");
    sleigh
}

fn build_x86_64(code: &[u8]) -> Sleigh {
    let mut sleigh = build_sleigh(code, "x86/data/languages/x86-64.sla");
    for (name, val) in X86_64_CONTEXT {
        sleigh.set_context_default(name, *val);
    }
    sleigh
}

fn code_addr(sleigh: &Sleigh, off: u64) -> Address {
    let spc = Rc::clone(
        sleigh.base().manager().get_default_code_space().expect("default code space"),
    );
    Address::new(spc, off)
}

#[test]
fn add_imm32_masks_operand_pins_opcode() {
    // add eax, 0x12345678 == 05 78 56 34 12
    let code = [0x05, 0x78, 0x56, 0x34, 0x12];
    let sleigh = build_x86_64(&code);
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");

    eprintln!("add imm32: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 5, "add eax,imm32 is 5 bytes");
    assert_eq!(m.bytes, code);
    // Opcode byte 0 is pinned; the imm32 bytes 1..5 are operand (zeroed).
    assert_eq!(m.fixed_mask[0], 0xff, "opcode byte must be fully pinned");
    assert_eq!(&m.fixed_mask[1..5], &[0, 0, 0, 0], "imm32 operand bytes must be zeroed");

    // The immediate operand classifies a Scalar carrying the real value.
    let scalars: Vec<&OpObject> = m
        .operands
        .iter()
        .flat_map(|o| o.objects.iter())
        .filter(|o| matches!(o, OpObject::Scalar { .. }))
        .collect();
    assert!(
        scalars.iter().any(|o| matches!(o, OpObject::Scalar { signed: 0x12345678, .. })),
        "expected a Scalar operand == 0x12345678, got {:?}",
        m.operands
    );
}

#[test]
fn mov_imm32_masks_operand_pins_opcode() {
    // mov ecx, 0x11223344 == b9 44 33 22 11
    let code = [0xb9, 0x44, 0x33, 0x22, 0x11];
    let sleigh = build_x86_64(&code);
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");

    eprintln!("mov imm32: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 5, "mov ecx,imm32 is 5 bytes");
    // The `mov r32,imm32` opcode is `b8+rd`: the high 5 bits are the pinned
    // opcode, the low 3 bits are the destination-register SELECTOR (an operand).
    // So byte 0's mask is 0xf8 — pins the opcode, leaves the register-selector
    // bits free.  This is the accessor correctly distinguishing the embedded
    // register operand from the opcode within a single byte.
    assert_eq!(m.fixed_mask[0], 0xf8, "opcode (high 5 bits) pinned, reg selector (low 3) free");
    assert_eq!(&m.fixed_mask[1..5], &[0, 0, 0, 0], "imm32 operand bytes must be zeroed");

    assert!(
        m.operands
            .iter()
            .flat_map(|o| o.objects.iter())
            .any(|o| matches!(o, OpObject::Scalar { signed: 0x11223344, .. })),
        "expected a Scalar operand == 0x11223344, got {:?}",
        m.operands
    );
}

#[test]
fn add_reg_reg_classifies_register() {
    // add eax, ecx == 01 c8  (opcode 01, modrm 0xc8 = mod=11 reg=001 rm=000)
    let code = [0x01, 0xc8];
    let sleigh = build_x86_64(&code);
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");

    eprintln!("add reg,reg: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 2, "add eax,ecx is 2 bytes");
    assert_eq!(m.fixed_mask[0], 0xff, "opcode byte 0 must be pinned");
    // The modrm byte has the mod bits pinned (0b11xxxxxx) but reg/rm selectors
    // are operand bits — so it must NOT be fully pinned.
    assert_ne!(m.fixed_mask[1], 0xff, "modrm reg/rm selector bits must be operand bits");
    assert_ne!(m.fixed_mask[1], 0x00, "modrm mod bits must be pinned");

    // Both operands resolve to registers.
    let regs: Vec<&OpObject> = m
        .operands
        .iter()
        .flat_map(|o| o.objects.iter())
        .filter(|o| matches!(o, OpObject::Register { .. }))
        .collect();
    assert!(
        !regs.is_empty(),
        "expected at least one Register operand, got {:?}",
        m.operands
    );
}

// --- x86-64 REX-prefixed forms (the FID PR1-fix regression gate) ---------
//
// These exercise the multi-phase x86-64 decode: a REX prefix (0x40..0x4f)
// advances the parser's `instrPhase` context to a second phase before the
// real opcode is decoded.  Before the fix, `instruction_mask`'s post-decode
// decision-tree re-walk read the *final* (advanced) context and could not
// re-resolve the constructor — every REX form errored "Unable to resolve
// constructor".  With the matched pattern captured DURING decode (correct
// per-phase context), the mask is computed from the real leaf and these pass.
// (They FAIL on `main` / pre-fix; the four 32-bit-style forms above always
// passed — x86-64.sla, no REX prefix.)

#[test]
fn rex_add_r64_r64_masks_modrm_pins_opcode_and_rexw() {
    // add rsi, rdi == 48 01 fe
    //   48   = REX prefix, REX.W set (64-bit operand size)
    //   01   = opcode ADD r/m64, r64
    //   fe   = modrm mod=11 reg=111(rdi) rm=110(rsi)  (register-direct)
    let code = [0x48, 0x01, 0xfe];
    let sleigh = build_x86_64(&code);
    let m = sleigh
        .instruction_mask(&code_addr(&sleigh, VMA))
        .expect("instruction_mask must succeed on a REX-prefixed x86-64 insn");

    eprintln!("REX add rsi,rdi: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 3, "add rsi,rdi (REX.W) is 3 bytes");
    assert_eq!(m.bytes, code);
    // REX byte: the prefix marker + REX.W (high 5 bits) are PINNED; the low 3
    // bits (REX.R/X/B register-extension selectors) are operand bits — so the
    // byte is partially pinned, never fully fixed and never fully free.
    assert_ne!(m.fixed_mask[0], 0x00, "REX prefix marker / REX.W must be pinned");
    assert_ne!(m.fixed_mask[0], 0xff, "REX.R/X/B register-extension bits are operand bits");
    assert_eq!(m.fixed_mask[0] & 0x08, 0x08, "REX.W bit (0x08) must be pinned");
    // Opcode byte fully pinned.
    assert_eq!(m.fixed_mask[1], 0xff, "the ADD opcode (01) must be fully pinned");
    // modrm: mod bits (top 2) pinned, reg/rm register selectors are operand.
    assert_ne!(m.fixed_mask[2], 0x00, "modrm mod bits must be pinned");
    assert_ne!(m.fixed_mask[2], 0xff, "modrm reg/rm selector bits must be operand bits");

    // Both register-direct operands classify as registers.
    let regs: Vec<&OpObject> = m
        .operands
        .iter()
        .flat_map(|o| o.objects.iter())
        .filter(|o| matches!(o, OpObject::Register { .. }))
        .collect();
    assert!(!regs.is_empty(), "expected Register operands for add r64,r64; got {:?}", m.operands);
}

#[test]
fn rex_mov_r64_imm64_zeroes_imm64() {
    // mov rax, 0x8877665544332211 == 48 b8 11 22 33 44 55 66 77 88
    //   48   = REX.W
    //   b8+rd= mov r64, imm64 (rd selects rax)
    //   imm64 little-endian follows
    let code = [0x48, 0xb8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let sleigh = build_x86_64(&code);
    let m = sleigh
        .instruction_mask(&code_addr(&sleigh, VMA))
        .expect("instruction_mask must succeed on REX mov r64,imm64");

    eprintln!("REX mov rax,imm64: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 10, "mov r64,imm64 (REX.W) is 10 bytes");
    // REX.W pinned, opcode high bits pinned, register selector (low 3 of b8+rd)
    // free; the imm64 (bytes 2..10) is the operand and must be fully zeroed.
    assert_eq!(m.fixed_mask[0] & 0x08, 0x08, "REX.W (0x08) must be pinned");
    assert_ne!(m.fixed_mask[1], 0x00, "the b8+rd opcode bits must be pinned");
    assert_eq!(
        &m.fixed_mask[2..10],
        &[0, 0, 0, 0, 0, 0, 0, 0],
        "the imm64 operand bytes must be fully zeroed"
    );

    // The imm64 classifies a Scalar carrying the real value (0x8877665544332211
    // == -8613303245920329199 as i64).
    assert!(
        m.operands
            .iter()
            .flat_map(|o| o.objects.iter())
            .any(|o| matches!(o, OpObject::Scalar { signed: -8613303245920329199, .. })),
        "expected a Scalar operand == 0x8877665544332211, got {:?}",
        m.operands
    );
}

#[test]
fn rex_mov_r64_r64_classifies_both_registers() {
    // mov rsi, rdi == 48 89 fe  (REX.W + opcode 89 MOV r/m64,r64 + modrm fe)
    let code = [0x48, 0x89, 0xfe];
    let sleigh = build_x86_64(&code);
    let m = sleigh
        .instruction_mask(&code_addr(&sleigh, VMA))
        .expect("instruction_mask must succeed on REX mov r64,r64");

    eprintln!("REX mov rsi,rdi: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 3, "mov rsi,rdi (REX.W) is 3 bytes");
    assert_eq!(m.fixed_mask[0] & 0x08, 0x08, "REX.W (0x08) must be pinned");
    assert_eq!(m.fixed_mask[1], 0xff, "the MOV opcode (89) must be fully pinned");
    assert_ne!(m.fixed_mask[2], 0xff, "modrm reg/rm selectors are operand bits");

    assert_eq!(m.operands.len(), 2);
    assert_eq!(m.operands[0].objects, [OpObject::Register { offset: 0x30 }]); // RSI
    assert_eq!(m.operands[1].objects, [OpObject::Register { offset: 0x38 }]); // RDI
}

#[test]
fn mov_mem_disp32_masks_displacement() {
    // mov eax, dword ptr [rbx+0x12345678] == 8b 83 78 56 34 12
    // (opcode 8b, modrm 0x83 = mod=10 reg=000 rm=011, disp32 follows).  This is
    // the disp32-displacement analog of the design doc's lea example; the RIP-
    // relative `lea`/`mov` (mod=00 rm=101) forms need address arithmetic the
    // minimal standalone harness does not supply, so a register-base disp32 form
    // is used to demonstrate the displacement masking.
    let code = [0x8b, 0x83, 0x78, 0x56, 0x34, 0x12];
    let sleigh = build_x86_64(&code);
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");

    eprintln!("mov mem+disp32: len={} bytes={:02x?} fixed={:02x?}", m.length, m.bytes, m.fixed_mask);
    assert_eq!(m.length, 6, "mov eax,[rbx+disp32] is 6 bytes");
    // Opcode (8b) is pinned; the modrm byte has its mod/reg-encoding bits pinned
    // (0xc0) with the base-register selector free; the disp32 bytes 2..6 are the
    // displacement operand and must be zeroed.
    assert_eq!(m.fixed_mask[0], 0xff, "opcode byte must be pinned");
    assert_ne!(m.fixed_mask[1], 0x00, "modrm mode/reg bits must be pinned");
    assert_eq!(
        &m.fixed_mask[2..6],
        &[0, 0, 0, 0],
        "disp32 displacement bytes must be zeroed (operand)"
    );

    assert_eq!(m.operands.len(), 2);
    assert_eq!(m.operands[0].objects, [OpObject::Register { offset: 0 }]); // EAX
    assert_eq!(
        m.operands[1].objects,
        [
            OpObject::Register { offset: 0x18 }, // RBX
            OpObject::Scalar { signed: 0x12345678, whole_operand: false, is_address: false },
        ],
    );
}

#[test]
fn jnz_reports_only_the_promoted_branch_address() {
    let sleigh = build_x86_64(&[0x75, 0x05]); // jnz VMA + 7
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");
    assert_eq!(m.operands.len(), 1);
    assert_eq!(m.operands[0].objects, [OpObject::Address]);
}

#[test]
fn aarch64_cmn_preserves_a_forwarded_immediate() {
    // cmn x2, #5. The hidden SBIT_CZNO node occupies instruction slot 0;
    // slot 0 of the immediate's forwarding constructor is the immediate.
    let sleigh = build_sleigh(&[0x5f, 0x14, 0x00, 0xb1], "AARCH64/data/languages/AARCH64.sla");
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");
    assert_eq!(m.operands.len(), 2);
    assert_eq!(m.operands[0].objects, [OpObject::Register { offset: 0x4010 }]); // x2
    assert_eq!(
        m.operands[1].objects,
        [OpObject::Scalar { signed: 5, whole_operand: true, is_address: false }],
    );
}

#[test]
fn aarch64_ldr_does_not_report_invisible_registers_from_extend_syntax() {
    let sleigh = build_sleigh(
        &[
            0x41, 0xe8, 0x63, 0x3c, // ldr b1, [x2, x3, sxtx]
            0x41, 0x68, 0x63, 0x3c, // ldr b1, [x2, x3]
        ],
        "AARCH64/data/languages/AARCH64.sla",
    );
    for off in [VMA, VMA + 4] {
        let m = sleigh.instruction_mask(&code_addr(&sleigh, off)).expect("instruction_mask");
        assert_eq!(m.operands.len(), 5);
        assert_eq!(m.operands[0].objects, [OpObject::Register { offset: 0x5020 }]); // b1
        assert_eq!(m.operands[1].objects, [OpObject::Register { offset: 0x4010 }]); // x2
        assert_eq!(m.operands[2].objects, [OpObject::Register { offset: 0x4018 }]); // x3
        // Both the literal-only `sxtx` and empty extend_spec consume x3
        // for semantics without printing it; neither contributes an object.
        assert!(m.operands[3].objects.is_empty());
        assert!(m.operands[4].objects.is_empty()); // empty extend_amount
    }
}

#[test]
fn aarch64_vector_add_does_not_treat_printed_names_as_scalars() {
    // add v1.4s, v2.4s, v3.4s prints attached names; the actual registers
    // are hidden semantic operands in the vector-register constructors.
    let sleigh = build_sleigh(&[0x41, 0x84, 0xa3, 0x4e], "AARCH64/data/languages/AARCH64.sla");
    let m = sleigh.instruction_mask(&code_addr(&sleigh, VMA)).expect("instruction_mask");
    assert_eq!(m.operands.len(), 3);
    assert!(m.operands.iter().all(|operand| operand.objects.is_empty()));
}

#[test]
fn assembly_into_matches_legacy_and_clears_reused_buffers() {
    let code = [0x48, 0x01, 0xfe, 0xb9, 0x44, 0x33, 0x22, 0x11];
    let mut sleigh = build_x86_64(&code);
    let first = code_addr(&sleigh, VMA);

    let mut emitted = TextEmit::default();
    let legacy_len = sleigh.print_assembly(&mut emitted, &first).unwrap();
    let mut mnemonic = String::from("stale mnemonic");
    let mut body = String::from("stale body");
    let direct_len = {
        let translate: &dyn Translate = &sleigh;
        translate
            .print_assembly_into(&first, &mut mnemonic, &mut body)
            .unwrap()
    };
    assert_eq!(direct_len, legacy_len);
    assert_eq!(mnemonic, emitted.mnemonic);
    assert_eq!(body, emitted.body);

    let second = code_addr(&sleigh, VMA + legacy_len as u64);
    let mut second_emit = TextEmit::default();
    let second_len = sleigh.print_assembly(&mut second_emit, &second).unwrap();
    let direct_second_len = {
        let translate: &dyn Translate = &sleigh;
        translate
            .print_assembly_into(&second, &mut mnemonic, &mut body)
            .unwrap()
    };
    assert_eq!(direct_second_len, second_len);
    assert_eq!(mnemonic, second_emit.mnemonic);
    assert_eq!(body, second_emit.body);

    sleigh.set_loader(Box::new(MissingImg));
    mnemonic.push_str(" stale");
    body.push_str(" stale");
    assert!(sleigh
        .print_assembly_into(&first, &mut mnemonic, &mut body)
        .is_err());
    assert!(mnemonic.is_empty());
    assert!(body.is_empty());
}

#[test]
fn assembly_emitter_can_disassemble_again_without_changing_outer_text() {
    struct NestedEmit<'a> {
        sleigh: &'a Sleigh,
        next: Address,
        before: (String, String),
        after: (String, String),
        nested: TextEmit,
    }
    impl AssemblyEmit for NestedEmit<'_> {
        fn dump(&mut self, _addr: &Address, mnemonic: &str, body: &str) {
            self.before = (mnemonic.to_owned(), body.to_owned());
            self.sleigh.print_assembly(&mut self.nested, &self.next).unwrap();
            self.after = (mnemonic.to_owned(), body.to_owned());
        }
    }

    let code = [0x48, 0x01, 0xfe, 0xb9, 0x44, 0x33, 0x22, 0x11];
    let sleigh = build_x86_64(&code);
    let first = code_addr(&sleigh, VMA);
    let next = code_addr(&sleigh, VMA + 3);
    let mut want_first = (String::new(), String::new());
    let mut want_next = (String::new(), String::new());
    sleigh.print_assembly_into(&first, &mut want_first.0, &mut want_first.1).unwrap();
    sleigh.print_assembly_into(&next, &mut want_next.0, &mut want_next.1).unwrap();
    let mut emit = NestedEmit {
        sleigh: &sleigh,
        next,
        before: (String::new(), String::new()),
        after: (String::new(), String::new()),
        nested: TextEmit::default(),
    };
    assert_eq!(sleigh.print_assembly(&mut emit, &first).unwrap(), 3);
    assert_eq!(emit.before, want_first);
    assert_eq!(emit.after, want_first);
    assert_eq!((emit.nested.mnemonic, emit.nested.body), want_next);
    let mut again = TextEmit::default();
    assert_eq!(sleigh.print_assembly(&mut again, &first).unwrap(), 3);
    assert_eq!((again.mnemonic, again.body), want_first);
}
