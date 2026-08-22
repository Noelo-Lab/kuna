//! Fixture: a caller that reads the SECOND return register after calling a
//! callee that provably never writes it.
//!
//! `scalar_callee` returns one value in RAX and touches nothing else;
//! `pair_shaped_reader` nevertheless reads RDX twice after the call and tests
//! the low byte of RAX -- byte for byte the shape a rustc `Result` consumer has.
//! Only the callee's body separates the two, which is what the `rustabi` call
//! seam has to consult. Hand-written asm because no compiler emits this.
//!
//! Build (see README.md):
//!   rustc --target x86_64-unknown-linux-gnu -C opt-level=2 -C panic=abort \
//!         -C relocation-model=static -C link-arg=-nostartfiles \
//!         -o rust_clobber_pair_x86_64 rust_clobber_pair_x86_64.rs
#![no_std]
#![no_main]
use core::arch::global_asm;
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

global_asm!(
    r#"
.globl scalar_callee
.type scalar_callee, @function
.p2align 4
scalar_callee:
    movq %rdi, %rax
    addq $7, %rax
    ret
.size scalar_callee, .-scalar_callee

.globl pair_shaped_reader
.type pair_shaped_reader, @function
.p2align 4
pair_shaped_reader:
    pushq %rax
    call scalar_callee
    leaq 1(%rdx), %rcx
    leaq 100(%rdx), %rsi
    testb $1, %al
    cmoveq %rsi, %rcx
    movq %rcx, %rax
    popq %rcx
    ret
.size pair_shaped_reader, .-pair_shaped_reader
"#,
    options(att_syntax)
);

extern "C" {
    fn pair_shaped_reader(x: u64) -> u64;
}
static mut SINK: u64 = 0;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        SINK = pair_shaped_reader(3);
    }
    loop {}
}
