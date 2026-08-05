//! (kuna) `libcsigs` — the measured extension of the built-in libc prototype table.
//!
//! [`super::LibProtoPass`] ships 27 signatures: a deliberate minimal stand-in for
//! the Ghidra `.gdt` archives kuna cannot vendor. Every libc callee outside those
//! 27 leaves its caller's arguments as an inferred `unsigned long`, which is the
//! single largest source of kuna's `type_match` gap on plain C.
//!
//! This pass adds the measured remainder. The entries were chosen by frequency,
//! not intuition: `objdump` over the 547 C binaries of the frozen decbench corpus
//! gives a PLT call-site histogram, and the 1587 cases where IDA scores a perfect
//! `type_match` and kuna does not give a per-case callee ranking. A name is in the
//! table when it clears **>= 100 corpus call sites or >= 3 of those cases**.
//!
//! ## Where the signatures come from
//!
//! A wrong prototype is worse than a missing one — it asserts a false type where
//! `unsigned long` was merely uninformative — so no signature here was written from
//! memory. Each was reduced from a machine-readable declaration:
//!
//! * the bulk from `gcc -aux-info` over a translation unit including the platform
//!   headers with `_GNU_SOURCE` + `_FORTIFY_SOURCE=2`, reduced to this module's
//!   vocabulary by a fixed rule;
//! * `__isoc99_{scanf,sscanf,fscanf}` from the `__REDIRECT` in `<stdio.h>`, which
//!   *proves* them ABI-identical to the standard names they replace;
//! * the FORTIFY `*_chk` family from GCC's own builtin types, checked with
//!   `__builtin_types_compatible_p`;
//! * `__stack_chk_fail` from glibc `debug/stack_chk_fail.c` (`void (void)`).
//!
//! A declaration whose every slot is not width-stable is **rejected**, not
//! approximated: `off_t`/`time_t`/`long long`/`char` parameters have no honest
//! spelling in [`Ty`], so `lseek`, `time`, `strtoll`, `fseeko`, `mmap`, `signal`
//! and `qsort` are deliberately absent. The full derivation, the ranking and the
//! rejected set are in `docs/features/libcsigs/`.
//!
//! ## Imports only
//!
//! Unlike the 27-entry base table, an entry here is applied **only to a name the
//! image imports** and does not itself define. That is the whole wrongness axis:
//! a PLT/IAT import named `error` is definitively the platform's
//! `error(int, int, const char *, ...)`, but a *defined* `error` is the image's own
//! function and may only share the spelling — zlib's `minigzip` defines
//! `void error(const char *msg)`, and typing that call `error(0, 0, …)` would be
//! strictly worse than the `unsigned long` it replaces. The base table keeps its
//! defined-or-imported matching untouched.

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;
use std::collections::HashSet;

use super::{build_pieces, Sig, Ty};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// Seed the measured libc signature extension onto imported functions.
pub struct LibcSigsPass;

/// The measured extension to the base `LIBC` table. Disjoint from it by
/// construction (`table_is_disjoint_from_base`).
const LIBC_EXT: &[(&str, Sig)] = &[
    // stdio.h
    ("__fpending", Sig { ret: Ty::Size, params: &[Ty::VoidPtr], vararg: -1 }),
    ("__fpurge", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("__freading", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("__isoc99_fscanf", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::CharPtr], vararg: 2 }),
    ("__isoc99_scanf", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: 1 }),
    ("__isoc99_sscanf", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: 2 }),
    ("__overflow", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Int], vararg: -1 }),
    ("asprintf", Sig { ret: Ty::Int, params: &[Ty::CharPtrPtr, Ty::CharPtr], vararg: 2 }),
    ("clearerr_unlocked", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fclose", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fdopen", Sig { ret: Ty::VoidPtr, params: &[Ty::Int, Ty::CharPtr], vararg: -1 }),
    ("feof", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("ferror", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("ferror_unlocked", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fflush", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fflush_unlocked", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fgets", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("fileno", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fputc", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("fputs_unlocked", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("fread", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::Size, Ty::Size, Ty::VoidPtr], vararg: -1 }),
    ("fscanf", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::CharPtr], vararg: 2 }),
    ("fseek", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Long, Ty::Int], vararg: -1 }),
    ("ftell", Sig { ret: Ty::Long, params: &[Ty::VoidPtr], vararg: -1 }),
    ("fwrite", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::Size, Ty::Size, Ty::VoidPtr], vararg: -1 }),
    ("fwrite_unlocked", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::Size, Ty::Size, Ty::VoidPtr], vararg: -1 }),
    ("getc", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("getc_unlocked", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("getline", Sig { ret: Ty::Long, params: &[Ty::CharPtrPtr, Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("putc", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("putc_unlocked", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("putchar_unlocked", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("rename", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("setvbuf", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::CharPtr, Ty::Int, Ty::Size], vararg: -1 }),
    ("ungetc", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("vasprintf", Sig { ret: Ty::Int, params: &[Ty::CharPtrPtr, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("vsnprintf", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Size, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    // the FORTIFY (`_chk`) and glibc-internal ABI entry points
    ("__asprintf_chk", Sig { ret: Ty::Int, params: &[Ty::CharPtrPtr, Ty::Int, Ty::CharPtr], vararg: 3 }),
    ("__assert_fail", Sig { ret: Ty::Void, params: &[Ty::CharPtr, Ty::CharPtr, Ty::UInt, Ty::CharPtr], vararg: -1 }),
    ("__explicit_bzero_chk", Sig { ret: Ty::Void, params: &[Ty::VoidPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("__fprintf_chk", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Int, Ty::CharPtr], vararg: 3 }),
    ("__memcpy_chk", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("__memmove_chk", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("__mempcpy_chk", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("__memset_chk", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Int, Ty::Size, Ty::Size], vararg: -1 }),
    ("__printf_chk", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtr], vararg: 2 }),
    ("__snprintf_chk", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Size, Ty::Int, Ty::Size, Ty::CharPtr], vararg: 5 }),
    ("__sprintf_chk", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Int, Ty::Size, Ty::CharPtr], vararg: 4 }),
    ("__stack_chk_fail", Sig { ret: Ty::Void, params: &[], vararg: -1 }),
    ("__stpcpy_chk", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("__strcat_chk", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("__strcpy_chk", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("__strncat_chk", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("__strncpy_chk", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("__syslog_chk", Sig { ret: Ty::Void, params: &[Ty::Int, Ty::Int, Ty::CharPtr], vararg: 3 }),
    ("__vasprintf_chk", Sig { ret: Ty::Int, params: &[Ty::CharPtrPtr, Ty::Int, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("__vfprintf_chk", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Int, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("__vsnprintf_chk", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Size, Ty::Int, Ty::Size, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    // string.h / strings.h
    ("bzero", Sig { ret: Ty::Void, params: &[Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("explicit_bzero", Sig { ret: Ty::Void, params: &[Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("index", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("memchr", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Int, Ty::Size], vararg: -1 }),
    ("memcmp", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("mempcpy", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("rawmemchr", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Int], vararg: -1 }),
    ("rindex", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("stpcpy", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("stpncpy", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("strcasecmp", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strcasestr", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strcoll", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strcspn", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strdup", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("strerror", Sig { ret: Ty::CharPtr, params: &[Ty::Int], vararg: -1 }),
    ("strncasecmp", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("strndup", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("strnlen", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("strpbrk", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strrchr", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("strsep", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtrPtr, Ty::CharPtr], vararg: -1 }),
    ("strspn", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strtok", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strtok_r", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::CharPtrPtr], vararg: -1 }),
    // stdlib.h
    ("_exit", Sig { ret: Ty::Void, params: &[Ty::Int], vararg: -1 }),
    ("abort", Sig { ret: Ty::Void, params: &[], vararg: -1 }),
    ("abs", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("atol", Sig { ret: Ty::Long, params: &[Ty::CharPtr], vararg: -1 }),
    ("canonicalize_file_name", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("exit", Sig { ret: Ty::Void, params: &[Ty::Int], vararg: -1 }),
    ("getenv", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("getopt_long", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtrPtr, Ty::CharPtr, Ty::VoidPtr, Ty::IntPtr], vararg: -1 }),
    ("labs", Sig { ret: Ty::Long, params: &[Ty::Long], vararg: -1 }),
    ("reallocarray", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Size, Ty::Size], vararg: -1 }),
    ("realpath", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("setenv", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("strtol", Sig { ret: Ty::Long, params: &[Ty::CharPtr, Ty::CharPtrPtr, Ty::Int], vararg: -1 }),
    ("strtoul", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::CharPtrPtr, Ty::Int], vararg: -1 }),
    ("unsetenv", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: -1 }),
    // ctype.h / wctype.h
    ("__ctype_b_loc", Sig { ret: Ty::VoidPtr, params: &[], vararg: -1 }),
    ("__ctype_get_mb_cur_max", Sig { ret: Ty::Size, params: &[], vararg: -1 }),
    ("__ctype_tolower_loc", Sig { ret: Ty::VoidPtr, params: &[], vararg: -1 }),
    ("__ctype_toupper_loc", Sig { ret: Ty::VoidPtr, params: &[], vararg: -1 }),
    ("iswprint", Sig { ret: Ty::Int, params: &[Ty::UInt], vararg: -1 }),
    ("iswspace", Sig { ret: Ty::Int, params: &[Ty::UInt], vararg: -1 }),
    ("tolower", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("toupper", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("towlower", Sig { ret: Ty::UInt, params: &[Ty::UInt], vararg: -1 }),
    ("wcwidth", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    // libintl.h / locale.h / time.h / wchar.h
    ("bindtextdomain", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("dcgettext", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("dgettext", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("gettext", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("localtime", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("localtime_r", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("mbrlen", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::Size, Ty::VoidPtr], vararg: -1 }),
    ("mbrtowc", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::CharPtr, Ty::Size, Ty::VoidPtr], vararg: -1 }),
    ("mbsinit", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("nl_langinfo", Sig { ret: Ty::CharPtr, params: &[Ty::Int], vararg: -1 }),
    ("strftime", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::Size, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("textdomain", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr], vararg: -1 }),
    // unistd.h
    ("access", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("alarm", Sig { ret: Ty::UInt, params: &[Ty::UInt], vararg: -1 }),
    ("chdir", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: -1 }),
    ("close", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("dup2", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int], vararg: -1 }),
    ("execlp", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: 2 }),
    ("execve", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtrPtr, Ty::CharPtrPtr], vararg: -1 }),
    ("fchdir", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("fchmod", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::UInt], vararg: -1 }),
    ("fchown", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::UInt, Ty::UInt], vararg: -1 }),
    ("fork", Sig { ret: Ty::Int, params: &[], vararg: -1 }),
    ("fsync", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("getcwd", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("geteuid", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("getgid", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("getpagesize", Sig { ret: Ty::Int, params: &[], vararg: -1 }),
    ("getpid", Sig { ret: Ty::Int, params: &[], vararg: -1 }),
    ("getuid", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("isatty", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("link", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("pipe", Sig { ret: Ty::Int, params: &[Ty::IntPtr], vararg: -1 }),
    ("read", Sig { ret: Ty::Long, params: &[Ty::Int, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("readlink", Sig { ret: Ty::Long, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("sleep", Sig { ret: Ty::UInt, params: &[Ty::UInt], vararg: -1 }),
    ("sysconf", Sig { ret: Ty::Long, params: &[Ty::Int], vararg: -1 }),
    ("umask", Sig { ret: Ty::UInt, params: &[Ty::UInt], vararg: -1 }),
    ("unlink", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: -1 }),
    ("unlinkat", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("write", Sig { ret: Ty::Long, params: &[Ty::Int, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    // fcntl.h / sys/stat.h / dirent.h
    ("closedir", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("dirfd", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("dirname", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("fcntl", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int], vararg: 2 }),
    ("fdopendir", Sig { ret: Ty::VoidPtr, params: &[Ty::Int], vararg: -1 }),
    ("fnmatch", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("fstat", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("fstatat", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtr, Ty::VoidPtr, Ty::Int], vararg: -1 }),
    ("ioctl", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Size], vararg: 2 }),
    ("lstat", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("mkdir", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::UInt], vararg: -1 }),
    ("munmap", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("open", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Int], vararg: 2 }),
    ("openat", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtr, Ty::Int], vararg: 3 }),
    ("opendir", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("readdir", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("stat", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    // signal.h / sys/wait.h
    ("kill", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int], vararg: -1 }),
    ("raise", Sig { ret: Ty::Int, params: &[Ty::Int], vararg: -1 }),
    ("sigaction", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("sigaddset", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Int], vararg: -1 }),
    ("sigemptyset", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("sigprocmask", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("waitpid", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::IntPtr, Ty::Int], vararg: -1 }),
    // sys/socket.h / netdb.h / arpa/inet.h
    ("freeaddrinfo", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("getaddrinfo", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr, Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("getnameinfo", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt, Ty::CharPtr, Ty::UInt, Ty::CharPtr, Ty::UInt, Ty::Int], vararg: -1 }),
    ("getsockopt", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int, Ty::Int, Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("htonl", Sig { ret: Ty::UInt, params: &[Ty::UInt], vararg: -1 }),
    ("ntohl", Sig { ret: Ty::UInt, params: &[Ty::UInt], vararg: -1 }),
    ("send", Sig { ret: Ty::Long, params: &[Ty::Int, Ty::VoidPtr, Ty::Size, Ty::Int], vararg: -1 }),
    ("setsockopt", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int, Ty::Int, Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("socket", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int, Ty::Int], vararg: -1 }),
    // the rest
    ("__errno_location", Sig { ret: Ty::IntPtr, params: &[], vararg: -1 }),
    ("clock_gettime", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("closelog", Sig { ret: Ty::Void, params: &[], vararg: -1 }),
    ("dlclose", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("dlerror", Sig { ret: Ty::CharPtr, params: &[], vararg: -1 }),
    ("dlsym", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::CharPtr], vararg: -1 }),
    ("endpwent", Sig { ret: Ty::Void, params: &[], vararg: -1 }),
    ("err", Sig { ret: Ty::Void, params: &[Ty::Int, Ty::CharPtr], vararg: 2 }),
    ("error", Sig { ret: Ty::Void, params: &[Ty::Int, Ty::Int, Ty::CharPtr], vararg: 3 }),
    ("errx", Sig { ret: Ty::Void, params: &[Ty::Int, Ty::CharPtr], vararg: 2 }),
    ("getgrgid", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt], vararg: -1 }),
    ("getgrnam", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("getpwnam", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("getpwuid", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt], vararg: -1 }),
    ("gettimeofday", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("openlog", Sig { ret: Ty::Void, params: &[Ty::CharPtr, Ty::Int, Ty::Int], vararg: -1 }),
    ("pthread_mutex_lock", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("pthread_mutex_unlock", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("syslog", Sig { ret: Ty::Void, params: &[Ty::Int, Ty::CharPtr], vararg: 2 }),
    ("tcsetattr", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("verr", Sig { ret: Ty::Void, params: &[Ty::Int, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("vwarn", Sig { ret: Ty::Void, params: &[Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("warn", Sig { ret: Ty::Void, params: &[Ty::CharPtr], vararg: 1 }),
    ("warnx", Sig { ret: Ty::Void, params: &[Ty::CharPtr], vararg: 1 }),
];

/// The names the image **imports** and does not itself define.
///
/// The undefined FUNC symbols (ELF `.dynsym`/`.symtab`, the COFF symtab, Mach-O
/// `LC_SYMTAB`) unioned with the §3 import resolver's names (PE IAT/INT, Mach-O
/// `__stubs`), minus every name the image defines. A binary that both defines and
/// imports a spelling is answering the question itself: the calls go to its own
/// definition, so nothing is asserted.
fn imported_function_names(file: &object::File, bytes: &[u8]) -> HashSet<String> {
    let mut imported = HashSet::new();
    let mut defined = HashSet::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let Ok(n) = sym.name() else { continue };
        let Ok(n) = String::from_utf8(crate::loader::elf_plt::strip_version(n.as_bytes())) else {
            continue;
        };
        if sym.is_undefined() {
            imported.insert(n);
        } else {
            defined.insert(n);
        }
    }
    for imp in crate::loader::format::resolve_imports(file, bytes) {
        if let Ok(n) = String::from_utf8(imp.name) {
            imported.insert(n);
        }
    }
    imported.retain(|n| !defined.contains(n));
    imported
}

impl AnalysisPass for LibcSigsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "libcsigs"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let imported = imported_function_names(ctx.file, ctx.bytes);
        if imported.is_empty() {
            return out;
        }
        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();
        for (name, sig) in LIBC_EXT {
            if !imported.contains(*name) {
                continue;
            }
            if let Ok(pieces) = build_pieces(name, sig, types, word_size) {
                out.prototypes.push(pieces);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_disjoint_from_base() {
        for (name, _) in LIBC_EXT {
            assert!(
                !super::super::LIBC.iter().any(|(n, _)| n == name),
                "{name} is already in the base table - a duplicate prototype would be committed twice"
            );
        }
    }

    #[test]
    fn table_names_are_unique() {
        let mut seen: HashSet<&str> = HashSet::new();
        for (name, _) in LIBC_EXT {
            assert!(seen.insert(name), "{name} appears twice in LIBC_EXT");
        }
    }

    #[test]
    fn vararg_slots_are_the_slot_past_the_fixed_parameters() {
        for (name, sig) in LIBC_EXT {
            if sig.vararg >= 0 {
                assert_eq!(
                    sig.vararg as usize,
                    sig.params.len(),
                    "{name}: the variadic slot is the one past the fixed parameters"
                );
            }
        }
    }

    #[test]
    fn fortify_printf_family_carries_the_extra_flag_parameter() {
        // The FORTIFY entry points are NOT aliases of the names they wrap: each
        // takes an extra `int flag` (and `__snprintf_chk` two extra `size_t`s), so
        // the format string sits at a different slot. Getting this wrong would
        // shift every argument of the corpus's highest-frequency printf call.
        let get = |want: &str| LIBC_EXT.iter().find(|(n, _)| *n == want).expect(want).1.vararg;
        assert_eq!(get("__printf_chk"), 2, "int flag, const char *fmt, ...");
        assert_eq!(get("__fprintf_chk"), 3, "FILE *, int flag, const char *fmt, ...");
        assert_eq!(get("__sprintf_chk"), 4, "char *, int flag, size_t, const char *fmt, ...");
        assert_eq!(
            get("__snprintf_chk"),
            5,
            "char *, size_t, int flag, size_t, const char *fmt, ..."
        );
    }

    #[test]
    fn only_imported_names_are_seeded() {
        // The wrongness axis, pinned: zlib's `minigzip` DEFINES
        // `void error(const char *msg)` while the table carries glibc's
        // `void error(int, int, const char *, ...)`. Seeding a name the image
        // defines would assert two phantom leading `int`s on it.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let imported = imported_function_names(&file, &bytes);
        for want in ["read", "open", "exit"] {
            assert!(imported.contains(want), "fauxware imports {want}: {imported:?}");
            assert!(LIBC_EXT.iter().any(|(n, _)| n == &want), "table should know {want}");
        }
        for defined in ["main", "authenticate"] {
            assert!(!imported.contains(defined), "{defined} is defined, not imported");
        }
    }
}
