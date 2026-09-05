//! **linuxsyscall** — render a 32-bit Linux `int 0x80` as the syscall it is,
//! instead of an indirect call through the `swi` userop.
//!
//! ```text
//!   v364 = 0x16b;                          v364 = 0x16b;
//!   (*(void *)swi(0x80))();          -->   sys_write(1,&v10,v364);
//! ```
//!
//! # The defect
//!
//! x86 SLEIGH lowers `INT imm8` to a black-box userop feeding an indirect call
//! (`ia.sinc:3671`):
//!
//! ```text
//!   tmp:1 = imm8;
//!   intloc:4 = swi(tmp);
//!   call [intloc];
//! ```
//!
//! That is the honest lifting of a *general* software interrupt — SLEIGH has no
//! way to know which operating system is behind the vector. On 32-bit Linux the
//! vector is fixed (`0x80`) and the ABI is fixed too: the syscall number is in
//! `EAX` and the arguments are `EBX, ECX, EDX, ESI, EDI, EBP` in that order, with
//! the result back in `EAX`. Left alone, every syscall in the binary renders as
//! `(*(void *)swi(0x80))();` — a call through a pointer nothing assigns, with the
//! number and the arguments erased: the register writes that set them up have no
//! reader left, so the dead-code fixpoint collects them. On a hand-written
//! `int 0x80` binary that is most of the program.
//!
//! # The rewrite
//!
//! The pass runs in the **pre-SSA window**, immediately after `ActionConstbase`
//! and before `ActionFuncLink`, which is what makes it cheap and what makes the
//! argument setup survive:
//!
//!   * pre-SSA there is no def-use edge to unpick and no `MULTIEQUAL` to patch —
//!     the pattern is recognized structurally, by op adjacency inside one
//!     instruction, not by walking a def chain that does not exist yet;
//!   * the `FuncCallSpecs` it builds carries a **locked** input prototype, so the
//!     later `ActionFuncLink` materializes the argument Varnodes itself, exactly
//!     as it does for a declared callee — the `mov $1,%ebx` that feeds the call
//!     keeps a reader and stays in the output.
//!
//! The `CALLIND` is retargeted in place (it already owns a `FuncCallSpecs` slot
//! and a block position) and the now-unread `swi` `CALLOTHER` is destroyed.
//!
//! # extrapop: why the synthesized call pops nothing
//!
//! `int 0x80` pushes no return address. A normal x86 `CALL rel32` lifts as
//! `push &next; call target`, and the default `__cdecl` `extrapop = 4` is what
//! `ActionExtraPopSetup` uses to give `ESP` back those four bytes after the call.
//! Applying it here — which is what happens today, the `CALLIND` being an
//! ordinary modelled call — moves `ESP` by four across every syscall and shifts
//! every later `ESP`-relative reference in the function. The synthesized spec
//! sets `extrapop = 0`, so `ActionExtraPopSetup` emits no adjustment at all.
//!
//! # What it will not do
//!
//! Three separate refusals, each of which leaves the old rendering untouched
//! rather than guessing:
//!
//!   1. **The number must be a constant reaching the call locally.** The backward
//!      scan starts at the `CALLOTHER` and walks its basic block; it accepts only
//!      a full-width `EAX = <constant>` and stops at the first op that writes any
//!      part of `EAX`, at any call or branch, and at the top of the block. A
//!      syscall number computed at run time, or set before a loop, is declined.
//!   2. **The number must be in [`SYSCALL_TABLE`].** The table is derived from the
//!      installed `asm/unistd_32.h` and the section-2 man pages (see below); a
//!      number with no vetted entry — an out-of-range number, a private vendor
//!      call, a syscall whose i386 entry point takes a different register set
//!      than its documented wrapper — is declined.
//!   3. **The language must be x86-32.** Every ABI register must resolve and the
//!      default code space must be 4 bytes wide, so an x86-64 image (where `EAX`
//!      resolves as a sub-register but `int 0x80` is not the syscall path) and
//!      every non-x86 language are excluded.
//!
//! # The table
//!
//! [`SYSCALL_TABLE`] is `(number, name, argument count)`. Names and numbers come
//! from the kernel's own `asm/unistd_32.h`; the argument count is the arity of
//! the syscall's documented prototype, read out of the section-2 manual page's
//! SYNOPSIS. A number is present only when that reading is unambiguous, so the
//! table is a *subset* of the syscall space by construction — 332 of the 438
//! `__NR_` names — and the four numbers whose i386 entry point is known to take a
//! different register set than the documented wrapper (`select`/`mmap`, which
//! take one pointer to an argument struct; `sigsuspend`, which carries two unused
//! history words; `ipc`, a multiplexer) are removed by hand rather than trusted.
//! Nineteen more are set by hand from the kernel entry point where the manual
//! page yields nothing usable or documents the wrapper instead (`exit`, `open`,
//! `ioctl`, `clone`, `rt_sigaction`, the `*64` stat family, `openat`, …).
//!
//! Arity matters because it is the difference between `sys_write(1,&v10,v364)`
//! and a claim about registers the program never set. Where the arity is not
//! known the pass does not fall back to "print the name with no arguments" — that
//! is a false statement about the call, not a partial one.
//!
//! # The option
//!
//! `option linuxsyscall on|off`, live flag
//! [`ArchContext::linux_syscall`](crate::context::ArchContext).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::OpId;
use crate::funcdata::Funcdata;

/// Marshaling element `<linuxsyscall>` (kuna). ElementIds live in the 4000+
/// range; 4131 was the previous max.
pub const ELEM_LINUXSYSCALL: ElementId = ElementId::new("linuxsyscall", 4134);

/// The interrupt vector 32-bit Linux reserves for the syscall entry point.
pub const LINUX_SYSCALL_VECTOR: u64 = 0x80;

/// The i386 syscall argument registers, in ABI order; the number arrives in
/// `EAX` and the result comes back in it.
pub const ARG_REGISTERS: [&str; 6] = ["EBX", "ECX", "EDX", "ESI", "EDI", "EBP"];
/// The register carrying the syscall number on the way in and the result on the
/// way out.
pub const NUM_REGISTER: &str = "EAX";

/// The name prefix the synthesized call carries.
///
/// The kernel's own spelling for the entry point, and deliberately not the bare
/// libc name: the raw syscall returns `-errno` where the wrapper returns `-1`
/// and sets `errno`, so rendering it as `write(...)` would assert a call that is
/// not being made.
pub const SYSCALL_NAME_PREFIX: &str = "sys_";

/// `(number, name, argument count)` for the 32-bit Linux syscall entry point,
/// sorted by number.  Provenance and the vetting rules are in the module header;
/// the table is deliberately a subset of the syscall space, and a number that is
/// not in it is declined rather than guessed.
pub const SYSCALL_TABLE: &[(u32, &str, u8)] = &[
    (0, "restart_syscall", 0),
    (1, "exit", 1),
    (2, "fork", 0),
    (3, "read", 3),
    (4, "write", 3),
    (5, "open", 3),
    (6, "close", 1),
    (7, "waitpid", 3),
    (8, "creat", 2),
    (9, "link", 2),
    (10, "unlink", 1),
    (11, "execve", 3),
    (12, "chdir", 1),
    (13, "time", 1),
    (14, "mknod", 3),
    (15, "chmod", 2),
    (16, "lchown", 3),
    (19, "lseek", 3),
    (20, "getpid", 0),
    (21, "mount", 5),
    (22, "umount", 1),
    (23, "setuid", 1),
    (24, "getuid", 0),
    (25, "stime", 1),
    (26, "ptrace", 4),
    (27, "alarm", 1),
    (29, "pause", 0),
    (30, "utime", 2),
    (33, "access", 2),
    (34, "nice", 1),
    (36, "sync", 0),
    (37, "kill", 2),
    (38, "rename", 2),
    (39, "mkdir", 2),
    (40, "rmdir", 1),
    (41, "dup", 1),
    (43, "times", 1),
    (45, "brk", 1),
    (46, "setgid", 1),
    (47, "getgid", 0),
    (48, "signal", 2),
    (49, "geteuid", 0),
    (50, "getegid", 0),
    (51, "acct", 1),
    (52, "umount2", 2),
    (54, "ioctl", 3),
    (55, "fcntl", 3),
    (57, "setpgid", 2),
    (60, "umask", 1),
    (61, "chroot", 1),
    (62, "ustat", 2),
    (63, "dup2", 2),
    (64, "getppid", 0),
    (66, "setsid", 0),
    (67, "sigaction", 3),
    (68, "sgetmask", 0),
    (69, "ssetmask", 1),
    (70, "setreuid", 2),
    (71, "setregid", 2),
    (73, "sigpending", 1),
    (74, "sethostname", 2),
    (75, "setrlimit", 2),
    (76, "getrlimit", 2),
    (77, "getrusage", 2),
    (78, "gettimeofday", 2),
    (79, "settimeofday", 2),
    (80, "getgroups", 2),
    (81, "setgroups", 2),
    (83, "symlink", 2),
    (85, "readlink", 3),
    (86, "uselib", 1),
    (87, "swapon", 2),
    (89, "readdir", 3),
    (91, "munmap", 2),
    (92, "truncate", 2),
    (93, "ftruncate", 2),
    (94, "fchmod", 2),
    (95, "fchown", 3),
    (96, "getpriority", 2),
    (97, "setpriority", 3),
    (99, "statfs", 2),
    (100, "fstatfs", 2),
    (101, "ioperm", 3),
    (102, "socketcall", 2),
    (103, "syslog", 3),
    (104, "setitimer", 3),
    (105, "getitimer", 2),
    (106, "stat", 2),
    (107, "lstat", 2),
    (108, "fstat", 2),
    (110, "iopl", 1),
    (111, "vhangup", 0),
    (112, "idle", 0),
    (113, "vm86old", 1),
    (114, "wait4", 4),
    (115, "swapoff", 1),
    (116, "sysinfo", 1),
    (118, "fsync", 1),
    (120, "clone", 5),
    (121, "setdomainname", 2),
    (122, "uname", 1),
    (123, "modify_ldt", 3),
    (124, "adjtimex", 1),
    (125, "mprotect", 3),
    (126, "sigprocmask", 3),
    (127, "create_module", 2),
    (128, "init_module", 3),
    (129, "delete_module", 2),
    (130, "get_kernel_syms", 1),
    (131, "quotactl", 4),
    (132, "getpgid", 1),
    (133, "fchdir", 1),
    (134, "bdflush", 2),
    (136, "personality", 1),
    (138, "setfsuid", 1),
    (139, "setfsgid", 1),
    (140, "_llseek", 5),
    (142, "_newselect", 5),
    (143, "flock", 2),
    (144, "msync", 3),
    (145, "readv", 3),
    (146, "writev", 3),
    (147, "getsid", 1),
    (148, "fdatasync", 1),
    (149, "_sysctl", 1),
    (150, "mlock", 2),
    (151, "munlock", 2),
    (152, "mlockall", 1),
    (153, "munlockall", 0),
    (154, "sched_setparam", 2),
    (155, "sched_getparam", 2),
    (156, "sched_setscheduler", 3),
    (157, "sched_getscheduler", 1),
    (158, "sched_yield", 0),
    (159, "sched_get_priority_max", 1),
    (160, "sched_get_priority_min", 1),
    (161, "sched_rr_get_interval", 2),
    (162, "nanosleep", 2),
    (163, "mremap", 5),
    (164, "setresuid", 3),
    (165, "getresuid", 3),
    (166, "vm86", 2),
    (167, "query_module", 5),
    (168, "poll", 3),
    (169, "nfsservctl", 3),
    (170, "setresgid", 3),
    (171, "getresgid", 3),
    (172, "prctl", 5),
    (173, "rt_sigreturn", 0),
    (174, "rt_sigaction", 4),
    (175, "rt_sigprocmask", 4),
    (176, "rt_sigpending", 2),
    (178, "rt_sigqueueinfo", 3),
    (179, "rt_sigsuspend", 2),
    (182, "chown", 3),
    (183, "getcwd", 2),
    (184, "capget", 2),
    (185, "capset", 2),
    (186, "sigaltstack", 2),
    (187, "sendfile", 4),
    (190, "vfork", 0),
    (191, "ugetrlimit", 2),
    (192, "mmap2", 6),
    (195, "stat64", 2),
    (196, "lstat64", 2),
    (197, "fstat64", 2),
    (199, "getuid32", 0),
    (200, "getgid32", 0),
    (201, "geteuid32", 0),
    (202, "getegid32", 0),
    (217, "pivot_root", 2),
    (218, "mincore", 3),
    (219, "madvise", 3),
    (220, "getdents64", 3),
    (221, "fcntl64", 3),
    (224, "gettid", 0),
    (225, "readahead", 3),
    (226, "setxattr", 5),
    (227, "lsetxattr", 5),
    (228, "fsetxattr", 5),
    (229, "getxattr", 4),
    (230, "lgetxattr", 4),
    (231, "fgetxattr", 4),
    (232, "listxattr", 3),
    (233, "llistxattr", 3),
    (234, "flistxattr", 3),
    (235, "removexattr", 2),
    (236, "lremovexattr", 2),
    (237, "fremovexattr", 2),
    (240, "futex", 6),
    (241, "sched_setaffinity", 3),
    (242, "sched_getaffinity", 3),
    (243, "set_thread_area", 1),
    (245, "io_setup", 2),
    (246, "io_destroy", 1),
    (247, "io_getevents", 5),
    (248, "io_submit", 3),
    (249, "io_cancel", 3),
    (252, "exit_group", 1),
    (253, "lookup_dcookie", 3),
    (254, "epoll_create", 1),
    (255, "epoll_ctl", 4),
    (256, "epoll_wait", 4),
    (257, "remap_file_pages", 5),
    (258, "set_tid_address", 1),
    (259, "timer_create", 3),
    (260, "timer_settime", 4),
    (261, "timer_gettime", 2),
    (262, "timer_getoverrun", 1),
    (263, "timer_delete", 1),
    (264, "clock_settime", 2),
    (265, "clock_gettime", 2),
    (266, "clock_getres", 2),
    (267, "clock_nanosleep", 4),
    (270, "tgkill", 3),
    (271, "utimes", 2),
    (274, "mbind", 6),
    (275, "get_mempolicy", 5),
    (276, "set_mempolicy", 3),
    (278, "mq_unlink", 1),
    (279, "mq_timedsend", 5),
    (280, "mq_timedreceive", 5),
    (281, "mq_notify", 2),
    (282, "mq_getsetattr", 3),
    (283, "kexec_load", 4),
    (284, "waitid", 4),
    (286, "add_key", 5),
    (287, "request_key", 4),
    (289, "ioprio_set", 3),
    (290, "ioprio_get", 2),
    (291, "inotify_init", 0),
    (292, "inotify_add_watch", 3),
    (293, "inotify_rm_watch", 2),
    (294, "migrate_pages", 4),
    (295, "openat", 4),
    (296, "mkdirat", 3),
    (297, "mknodat", 4),
    (298, "fchownat", 5),
    (299, "futimesat", 3),
    (301, "unlinkat", 3),
    (302, "renameat", 4),
    (303, "linkat", 5),
    (304, "symlinkat", 3),
    (305, "readlinkat", 4),
    (306, "fchmodat", 4),
    (307, "faccessat", 4),
    (309, "ppoll", 4),
    (310, "unshare", 1),
    (311, "set_robust_list", 2),
    (312, "get_robust_list", 3),
    (313, "splice", 6),
    (314, "sync_file_range", 4),
    (315, "tee", 4),
    (316, "vmsplice", 4),
    (317, "move_pages", 6),
    (318, "getcpu", 3),
    (319, "epoll_pwait", 5),
    (320, "utimensat", 4),
    (321, "signalfd", 3),
    (322, "timerfd_create", 2),
    (323, "eventfd", 2),
    (324, "fallocate", 4),
    (325, "timerfd_settime", 4),
    (326, "timerfd_gettime", 2),
    (329, "epoll_create1", 1),
    (330, "dup3", 3),
    (331, "pipe2", 2),
    (332, "inotify_init1", 1),
    (333, "preadv", 4),
    (334, "pwritev", 4),
    (335, "rt_tgsigqueueinfo", 4),
    (336, "perf_event_open", 5),
    (337, "recvmmsg", 5),
    (338, "fanotify_init", 2),
    (339, "fanotify_mark", 5),
    (341, "name_to_handle_at", 5),
    (342, "open_by_handle_at", 3),
    (343, "clock_adjtime", 2),
    (344, "syncfs", 1),
    (345, "sendmmsg", 4),
    (346, "setns", 2),
    (347, "process_vm_readv", 6),
    (348, "process_vm_writev", 6),
    (349, "kcmp", 5),
    (351, "sched_setattr", 3),
    (352, "sched_getattr", 4),
    (353, "renameat2", 5),
    (354, "seccomp", 3),
    (355, "getrandom", 3),
    (356, "memfd_create", 2),
    (357, "bpf", 3),
    (358, "execveat", 5),
    (359, "socket", 3),
    (360, "socketpair", 4),
    (361, "bind", 3),
    (362, "connect", 3),
    (363, "listen", 2),
    (364, "accept4", 4),
    (365, "getsockopt", 5),
    (366, "setsockopt", 5),
    (367, "getsockname", 3),
    (368, "getpeername", 3),
    (369, "sendto", 6),
    (370, "sendmsg", 3),
    (371, "recvfrom", 6),
    (372, "recvmsg", 3),
    (373, "shutdown", 2),
    (374, "userfaultfd", 1),
    (375, "membarrier", 3),
    (376, "mlock2", 3),
    (377, "copy_file_range", 6),
    (378, "preadv2", 5),
    (379, "pwritev2", 5),
    (380, "pkey_mprotect", 4),
    (381, "pkey_alloc", 2),
    (382, "pkey_free", 1),
    (383, "statx", 5),
    (384, "arch_prctl", 2),
    (393, "semget", 3),
    (395, "shmget", 3),
    (396, "shmctl", 3),
    (397, "shmat", 3),
    (398, "shmdt", 1),
    (399, "msgget", 2),
    (400, "msgsnd", 4),
    (401, "msgrcv", 5),
    (402, "msgctl", 3),
    (424, "pidfd_send_signal", 4),
    (434, "pidfd_open", 2),
    (437, "openat2", 4),
    (438, "pidfd_getfd", 3),
    (439, "faccessat2", 4),
];

/// The name and argument count of a syscall number, or `None` when the number
/// has no vetted entry.
pub fn syscall_entry(num: u32) -> Option<(&'static str, usize)> {
    SYSCALL_TABLE
        .binary_search_by_key(&num, |&(n, _, _)| n)
        .ok()
        .map(|i| (SYSCALL_TABLE[i].1, SYSCALL_TABLE[i].2 as usize))
}

/// The storage the i386 syscall ABI uses: the number/result register and the six
/// argument registers, in ABI order.
pub struct SyscallAbi {
    /// `EAX` — the syscall number on the way in, the result on the way out.
    pub num: Address,
    /// `EBX, ECX, EDX, ESI, EDI, EBP`.
    pub args: Vec<Address>,
}

/// Resolve the i386 syscall ABI against this language, or `None` when the
/// language is not x86-32.
///
/// Every ABI register must resolve at its full 32-bit width and the default code
/// space must be 4 bytes wide.  x86-64 resolves `EAX`..`EBP` as sub-registers,
/// so the address-size test is what excludes it — there `int 0x80` is a
/// compatibility path, not the syscall ABI this models.
///
/// The lookup is the speculative probe, so in ghidra mode it sees only names
/// the register cache already holds; these seven are there because every
/// `<prototype>` in `x86gcc.cspec`/`x86win.cspec` is decoded during
/// registerProgram, before the first function is lifted (pinned by
/// `kuna-ghidra/tests/register_probe_e2e.rs`).
pub fn resolve_abi(data: &Funcdata) -> Option<SyscallAbi> {
    let manage = data.get_arch().manage();
    if manage.get_default_code_space()?.get_addr_size() != 4 {
        return None;
    }
    let lookup = manage.register_lookup()?;
    let reg = |nm: &str| -> Option<Address> {
        let st = lookup.probe_register(nm)?;
        if st.size != 4 {
            return None;
        }
        Some(Address::new(st.space.clone()?, st.offset))
    };
    let num = reg(NUM_REGISTER)?;
    let mut args = Vec::with_capacity(ARG_REGISTERS.len());
    for nm in ARG_REGISTERS {
        args.push(reg(nm)?);
    }
    Some(SyscallAbi { num, args })
}

/// One recognized `int 0x80` site.
struct Site {
    /// The `CALLIND` the SLEIGH lowering produced; retargeted in place.
    callind: OpId,
    /// The `swi` `CALLOTHER` feeding it; destroyed.
    callother: OpId,
    /// The resolved syscall name (without the [`SYSCALL_NAME_PREFIX`]).
    name: &'static str,
    /// How many of [`SyscallAbi::args`] the syscall takes.
    argc: usize,
}

/// Do two storage locations overlap?
fn overlaps(a: &Address, asz: int4, b: &Address, bsz: int4) -> bool {
    match (a.get_space(), b.get_space()) {
        (Some(x), Some(y)) if Rc::ptr_eq(x, y) => {
            let (ao, bo) = (a.get_offset(), b.get_offset());
            ao < bo + bsz as u64 && bo < ao + asz as u64
        }
        _ => false,
    }
}

/// The constant `EAX` carries at `from`, by a backward walk of `from`'s basic
/// block.
///
/// Stops — declining — at the first op that writes any part of `EAX` unless that
/// op is a full-width `EAX = <constant>`, at any call or branch (whose effect on
/// `EAX` is not visible here), and at the top of the block.  Pre-SSA there is no
/// def-use edge to follow, which is the point: this is a local, syntactic read of
/// the instruction sequence the assembler wrote, not a dataflow query.
fn syscall_number_before(data: &Funcdata, from: OpId, abi: &SyscallAbi) -> Option<u32> {
    let mut cur = data.op_previous_op(from)?;
    loop {
        let op = data.obank().get(cur)?;
        match op.code() {
            OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_RETURN => return None,
            _ => {}
        }
        if let Some(outid) = op.get_out() {
            let out = data.vbank().get(outid)?;
            if overlaps(out.get_addr(), out.get_size(), &abi.num, 4) {
                if op.code() != OpCode::CPUI_COPY
                    || out.get_addr() != &abi.num
                    || out.get_size() != 4
                {
                    return None;
                }
                let src = data.vbank().get(op.get_in(0)?)?;
                if !src.is_constant() {
                    return None;
                }
                return u32::try_from(src.get_offset()).ok();
            }
        }
        cur = data.op_previous_op(cur)?;
    }
}

/// Every `int 0x80` site in the function whose number resolves to a table entry.
///
/// The lowering is recognized structurally, by adjacency inside one instruction:
/// a two-input `CALLOTHER` whose second input is the one-byte vector constant,
/// whose output is consumed as the target of the very next op, a `CALLIND` at the
/// same address in the same block.  Nothing else in the x86 lifting has that
/// shape.  The walk starts from the op bank's `CALLOTHER` list because that is
/// the one of the two op-codes the bank indexes.
fn sites(data: &Funcdata, abi: &SyscallAbi) -> Vec<Site> {
    let mut found = Vec::new();
    let callothers: Vec<OpId> = data.obank().iter_code(OpCode::CPUI_CALLOTHER).collect();
    for other in callothers {
        let Some(oth) = data.obank().get(other) else { continue };
        let Some(block) = oth.get_parent() else { continue };
        if oth.num_input() != 2 {
            continue;
        }
        let addr = oth.get_addr().clone();
        let vector_ok = oth
            .get_in(1)
            .and_then(|v| data.vbank().get(v))
            .map(|v| v.is_constant() && v.get_size() == 1 && v.get_offset() == LINUX_SYSCALL_VECTOR)
            .unwrap_or(false);
        if !vector_ok {
            continue;
        }
        let Some(outvn) = oth.get_out() else { continue };
        let Some(callind) = data.op_next_op(other) else { continue };
        let Some(call) = data.obank().get(callind) else { continue };
        if call.code() != OpCode::CPUI_CALLIND
            || call.get_parent() != Some(block)
            || call.get_addr() != &addr
        {
            continue;
        }
        let same_storage = match (data.vbank().get(outvn), call.get_in(0).and_then(|v| data.vbank().get(v))) {
            (Some(o), Some(t)) => o.get_addr() == t.get_addr() && o.get_size() == t.get_size(),
            _ => false,
        };
        if !same_storage {
            continue;
        }
        let Some(num) = syscall_number_before(data, other, abi) else { continue };
        let Some((name, argc)) = syscall_entry(num) else { continue };
        found.push(Site { callind, callother: other, name, argc });
    }
    found
}

/// Retarget one site's `CALLIND` onto a named syscall call spec.
fn rewrite(data: &mut Funcdata, site: &Site, abi: &SyscallAbi) -> bool {
    use crate::dtype::type_metatype;
    use crate::fspec::ParameterPieces;

    let Some(idx) = data.get_call_specs_index(site.callind) else { return false };
    if data.get_call_specs(idx).has_funcdata() {
        return false;
    }
    let Some(types) = data.get_arch().types() else { return false };
    let (Ok(void_ty), Ok(word_ty)) =
        (types.get_type_void(), types.get_base(4, type_metatype::TYPE_UNKNOWN))
    else {
        return false;
    };
    let Some(model) =
        data.get_arch().eval_fp_called().or_else(|| data.get_arch().default_fp()).cloned()
    else {
        return false;
    };
    let entry = data.get_call_specs(idx).get_entry_address().clone();
    let name = format!("{SYSCALL_NAME_PREFIX}{}", site.name);

    let fc = data.get_call_specs_mut(idx);
    fc.proto_mut().set_internal(model, void_ty);
    // `int 0x80` pushes no return address, so the call restores nothing.
    fc.proto_mut().set_extra_pop(0);
    for (slot, reg) in abi.args.iter().take(site.argc).enumerate() {
        let piece =
            ParameterPieces { addr: reg.clone(), type_: Some(word_ty.clone()), flags: 0 };
        fc.proto_mut().set_param(slot as int4, "", &piece);
    }
    fc.proto_mut().set_input_lock(true);
    let out = ParameterPieces { addr: abi.num.clone(), type_: Some(word_ty), flags: 0 };
    fc.proto_mut().set_output(&out);
    fc.proto_mut().set_output_lock(true);
    if fc.set_funcdata(entry, &name).is_err() {
        return false;
    }

    data.op_set_opcode_code(site.callind, OpCode::CPUI_CALL);
    let handle = crate::flow::next_fspec_handle();
    let style = data.get_arch().kuna_name_style();
    data.get_call_specs(idx).register_in_fspec_space(handle, style);
    let fspecvn = data.new_varnode_call_specs(handle);
    if data.op_set_input(site.callind, fspecvn, 0).is_err() {
        return false;
    }
    data.op_destroy(site.callother);
    true
}

/// (kuna) `ActionLinuxSyscall` — name the 32-bit Linux `int 0x80` sites
/// (option `linuxsyscall`).
pub struct ActionLinuxSyscall {
    base: ActionBase,
}

impl ActionLinuxSyscall {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLinuxSyscall {
            base: ActionBase::new(
                crate::action::ruleflags::rule_onceperfunc,
                "linuxsyscall",
                g,
            ),
        })
    }
}

impl Action for ActionLinuxSyscall {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionLinuxSyscall { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().linux_syscall || data.num_calls() == 0 {
            return 0;
        }
        let Some(abi) = resolve_abi(data) else { return 0 };
        let found = sites(data, &abi);
        if found.is_empty() {
            return 0;
        }
        let mut changed = 0;
        for site in &found {
            if rewrite(data, site, &abi) {
                changed += 1;
            }
        }
        if changed != 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests;
