//! A function that zero-extends the value it returns into a 64-bit register
//! reads, on its own, as returning the narrow value: x86-64 and AArch64 leave
//! the bits above a narrow return unspecified, so `int f(..)` compiles to the
//! same instructions. A caller that computes with more of the register (`call
//! f; add $1,%rax`) relies on the zero-extension, and `decompile-all` takes the
//! callee's return from it: computed with in 64 bits, the callee returns the
//! whole register; in 32, a narrower value is unsigned. Before, the printed
//! callers sign-extended an `int` or a `char` that the binary zero-extends, or
//! computed in 32 bits what the binary computes in 64.
use crate::common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

/// The images' source; each image names its functions without the `s_`.
const SOURCE: &str = r#"
#define NI __attribute__((noinline))
NI unsigned long s_z32m(unsigned long x) { return (x * 3) & 0xffffffffUL; }
NI unsigned long s_zn(unsigned x) { return x * 5u; }
NI unsigned long s_zw(unsigned long x) { return (x * 7) & 0xffffffffUL; }
NI unsigned s_ru(unsigned x) { return x * 3; }
NI int s_si(int x) { return x * 7; }
NI int s_lb(unsigned x) { return (unsigned char)(x * 3); }
NI unsigned long s_lq(unsigned x) { return (unsigned char)(x * 5); }
NI unsigned long s_w(unsigned long x) { return s_zw(x + 7); }
long s_cz(unsigned long x) { return s_z32m(x ^ 5) + 1; }
int s_ci(unsigned long x) { return (int)s_z32m(x ^ 1) < 0; }
char *s_cp(unsigned long x) { return (char *)0x1000 + s_zn(x + 2); }
unsigned long s_cru(unsigned x) { return s_ru(x + 1) + 1UL; }
long s_cs(int x) { return s_si(x + 1) + 1L; }
long s_cb(unsigned x) { return s_lb(x + 1) - 0x80L; }
long s_cq(unsigned x) { return s_lq(x + 1) * 0x10000001L; }
long s_cw(unsigned long x) { return s_w(x ^ 5) + 1; }
NI unsigned long s_z6(unsigned long x) { return (x * 11) & 0xffffffffUL; }
NI unsigned long s_z7(unsigned long x) { return (x * 13) & 0xffffffffUL; }
NI unsigned long s_z8(unsigned long x) { return (x * 17) & 0xffffffffUL; }
NI long s_sink(long v) { return v - 0x7fffffffL; }
long s_rsh(unsigned long x) { return (long)s_z6(x ^ 1) >> 1; }
unsigned long s_lsh(unsigned long x) { return s_z7(x ^ 1) << 3; }
long s_arg(unsigned long x) { return s_sink((long)s_z8(x ^ 1) >> 2) * 2; }
int s_eqv(unsigned long x, int v) { return (int)s_z32m(x ^ 9) == v; }
long s_ar(unsigned long x, int v) { return (long)((int)s_z32m(x ^ 11) + v); }
NI unsigned long s_w64(unsigned long x) { return x * 0x100000003UL; }
int s_eqw(unsigned long x, int v) { return (int)s_w64(x ^ 13) == v; }
int s_eqz(unsigned long x, unsigned long y) { return (unsigned long)((unsigned)s_z32m(x ^ 15) + 1u) == y; }
"#;

/// Each checked function with the argument it takes.
const ARGS: &[(&str, &str)] = &[
    ("z32m", "(unsigned long)v"),
    ("zn", "(unsigned)v"),
    ("lb", "(unsigned)v"),
    ("cz", "(unsigned long)v"),
    ("ci", "(unsigned long)v"),
    ("cp", "(unsigned long)v"),
    ("cru", "(unsigned)v"),
    ("cs", "v"),
    ("cb", "(unsigned)v"),
    ("lq", "(unsigned)v"),
    ("cq", "(unsigned)v"),
    ("cw", "(unsigned long)v"),
    ("z6", "(unsigned long)v"),
    ("z7", "(unsigned long)v"),
    ("z8", "(unsigned long)v"),
    ("rsh", "(unsigned long)v"),
    ("lsh", "(unsigned long)v"),
    ("arg", "(unsigned long)v"),
    ("eqv", "(unsigned long)v, (int)(((unsigned long)v ^ 9) * 3)"),
    ("ar", "(unsigned long)v, v"),
    ("eqw", "(unsigned long)v, (int)(((unsigned long)v ^ 13) * 0x100000003UL)"),
    ("eqz", "(unsigned long)v, (unsigned long)(unsigned)((((unsigned long)v ^ 15) * 3) + 1)"),
];

struct Image {
    name: &'static str,
    arch: Architecture,
    /// `.text` with the calls already relocated.
    code: &'static str,
    functions: &'static [(&'static str, u64, u64)],
    checked: &'static [&'static str],
}

const ALL: &[&str] = &[
    "z32m", "zn", "lb", "cz", "ci", "cp", "cru", "cs", "cb", "lq", "cq", "cw", "z6", "z7", "z8", "rsh", "lsh", "arg", "eqv",
    "ar", "eqw", "eqz",
];

/// gcc -O2 and clang -O0 for x86-64, clang -O2 and -O0 for AArch64. AArch64
/// `mov w8,w0; sub x0,x8,#0x80` after a call prints as `(f(..) & 0xffffffff) -
/// 0x80` in the callee's width whatever it returns, so `cru` and `cb` are left
/// out there.
const IMAGES: &[Image] = &[
    Image {
        name: "x86_64-gcc-O2",
        arch: Architecture::X86_64,
        code: "8d047fc366662e0f1f840000000000908d04bfc366662e0f1f84000000000090488d04fd00000000\
               29f8c30f1f4400008d047fc366662e0f1f840000000000908d04fd0000000029f8c3660f1f440000\
               8d3c7f400fb6c7c30f1f8400000000008d3cbf400fb6c7c30f1f8400000000004883c707ebaa662e\
               0f1f8400000000004883f705e877ffffff4883c001c366904883f701e867ffffffc1e81fc30f1f00\
               83c702e868ffffff480500100000c39083c701e878ffffff89c04883c001c39083c701e878ffffff\
               48984883c001c39083c701e878ffffff48984883c080c39083c701e878ffffff4889c248c1e21c48\
               01d0c366662e0f1f84000000000066904883f705e867ffffff4883c001c36690488d04bf8d0447c3\
               0f1f840000000000488d047f8d0487c30f1f8400000000004889f848c1e00401f889c0c30f1f4000\
               488d8701000080c30f1f8400000000004883f701e8b7ffffff48d1f8c30f1f004883f701e8b7ffff\
               ff48c1e003c366904883f701e8b7ffffff48c1f8024889c7e8bbffffff4801c0c30f1f8000000000\
               4883f709e867feffff39c60f94c00fb6c0c366662e0f1f8400000000000f1f004883f70be847feff\
               ff01c64863c6c3904889f848c1e01f4801f8488d0447c3904883f70de8e7ffffff39c60f94c00fb6\
               c0c366662e0f1f8400000000000f1f004883f70fe807feffff83c0014839f00f94c00fb6c0c3",
        functions: &[
            ("z32m", 0x0, 0x4),
            ("zn", 0x10, 0x4),
            ("zw", 0x20, 0xb),
            ("ru", 0x30, 0x4),
            ("si", 0x40, 0xa),
            ("lb", 0x50, 0x8),
            ("lq", 0x60, 0x8),
            ("w", 0x70, 0x6),
            ("cz", 0x80, 0xe),
            ("ci", 0x90, 0xd),
            ("cp", 0xa0, 0xf),
            ("cru", 0xb0, 0xf),
            ("cs", 0xc0, 0xf),
            ("cb", 0xd0, 0xf),
            ("cq", 0xe0, 0x13),
            ("cw", 0x100, 0xe),
            ("z6", 0x110, 0x8),
            ("z7", 0x120, 0x8),
            ("z8", 0x130, 0xc),
            ("sink", 0x140, 0x8),
            ("rsh", 0x150, 0xd),
            ("lsh", 0x160, 0xe),
            ("arg", 0x170, 0x19),
            ("eqv", 0x190, 0x12),
            ("ar", 0x1b0, 0xf),
            ("w64", 0x1c0, 0xf),
            ("eqw", 0x1d0, 0x12),
            ("eqz", 0x1f0, 0x16),
        ],
        checked: ALL,
    },
    Image {
        name: "x86_64-clang-O0",
        arch: Architecture::X86_64,
        code: "554889e548897df8486b45f80348b9ffffffff000000004821c85dc30f1f4000554889e5897dfc6b\
               45fc0589c05dc390554889e548897df8486b45f80748b9ffffffff000000004821c85dc30f1f4000\
               554889e5897dfc6b45fc035dc30f1f00554889e5897dfc6b45fc075dc30f1f00554889e5897dfc6b\
               45fc030fb6c05dc3554889e5897dfc6b45fc050fb6c05dc3554889e54883ec1048897df8488b7df8\
               4883c707e887ffffff4883c4105dc390554889e54883ec1048897df8488b7df84883f705e837ffff\
               ff4883c0014883c4105dc3662e0f1f8400000000000f1f00554889e54883ec1048897df8488b7df8\
               4883f701e807ffffff83f8000f9cc024010fb6c04883c4105dc3660f1f440000554889e54883ec10\
               48897df8488b45f84883c00289c7e8f5feffff4889c1b8001000004801c84883c4105dc30f1f4000\
               554889e54883ec10897dfc8b7dfc83c701e8fafeffff89c04883c0014883c4105dc3662e0f1f8400\
               000000000f1f4000554889e54883ec10897dfc8b7dfc83c701e8dafeffff48984883c0014883c410\
               5dc3662e0f1f8400000000000f1f4000554889e54883ec10897dfc8b7dfc83c701e8bafeffff4898\
               482d800000004883c4105dc3662e0f1f8400000000006690554889e54883ec10897dfc8b7dfc83c7\
               01e89afeffff4869c0010000104883c4105dc3662e0f1f8400000000000f1f00554889e54883ec10\
               48897df8488b7df84883f705e877feffff4883c0014883c4105dc3662e0f1f8400000000000f1f00\
               554889e548897df8486b45f80b48b9ffffffff000000004821c85dc30f1f4000554889e548897df8\
               486b45f80d48b9ffffffff000000004821c85dc30f1f4000554889e548897df8486b45f81148b9ff\
               ffffff000000004821c85dc30f1f4000554889e548897df8488b45f8482dffffff7f5dc3662e0f1f\
               8400000000006690554889e54883ec1048897df8488b7df84883f701e867ffffff48c1f8014883c4\
               105dc3662e0f1f8400000000000f1f00554889e54883ec1048897df8488b7df84883f701e857ffff\
               ff48c1e0034883c4105dc3662e0f1f8400000000000f1f00554889e54883ec1048897df8488b7df8\
               4883f701e847ffffff4889c748c1ff02e85bffffff48c1e0014883c4105dc390554889e54883ec10\
               48897df88975f4488b7df84883f709e8a4fcffff3b45f40f94c024010fb6c04883c4105dc30f1f00\
               554889e54883ec1048897df88975f4488b7df84883f70be874fcffff0345f448984883c4105dc366\
               0f1f840000000000554889e548897df848b80300000001000000480faf45f85dc30f1f8000000000\
               554889e54883ec1048897df88975f4488b7df84883f70de8c4ffffff3b45f40f94c024010fb6c048\
               83c4105dc30f1f00554889e54883ec1048897df8488975f0488b7df84883f70fe8f3fbffff83c001\
               89c0483b45f00f94c024010fb6c04883c4105dc3",
        functions: &[
            ("z32m", 0x0, 0x1c),
            ("zn", 0x20, 0xf),
            ("zw", 0x30, 0x1c),
            ("ru", 0x50, 0xd),
            ("si", 0x60, 0xd),
            ("lb", 0x70, 0x10),
            ("lq", 0x80, 0x10),
            ("w", 0x90, 0x1f),
            ("cz", 0xb0, 0x23),
            ("ci", 0xe0, 0x2a),
            ("cp", 0x110, 0x2c),
            ("cru", 0x140, 0x22),
            ("cs", 0x170, 0x22),
            ("cb", 0x1a0, 0x24),
            ("cq", 0x1d0, 0x23),
            ("cw", 0x200, 0x23),
            ("z6", 0x230, 0x1c),
            ("z7", 0x250, 0x1c),
            ("z8", 0x270, 0x1c),
            ("sink", 0x290, 0x14),
            ("rsh", 0x2b0, 0x23),
            ("lsh", 0x2e0, 0x23),
            ("arg", 0x310, 0x2f),
            ("eqv", 0x340, 0x2d),
            ("ar", 0x370, 0x27),
            ("w64", 0x3a0, 0x19),
            ("eqw", 0x3c0, 0x2d),
            ("eqz", 0x3f0, 0x34),
        ],
        checked: ALL,
    },
    Image {
        name: "aarch64-clang-O2",
        arch: Architecture::Aarch64,
        code: "0004000bc0035fd60008000bc0035fd608701d530001004bc0035fd60004000bc0035fd608701d53\
               0001004bc0035fd60804000b001d0012c0035fd60808000b001d0012c0035fd6001c0091f1ffff17\
               fd7bbfa9fd030091a8008052000008cae8ffff9700040091fd7bc1a8c0035fd6fd7bbfa9fd030091\
               000040d2e1ffff97007c5fd3fd7bc1a8c0035fd6fd7bbfa9fd03009100080011dcffff9700044091\
               fd7bc1a8c0035fd6fd7bbfa9fd03009100040011daffff97e803002a00050091fd7bc1a8c0035fd6\
               fd7bbfa9fd03009100040011d4ffff97087c409300050091fd7bc1a8c0035fd6fd7bbfa9fd030091\
               00040011cfffff97e803002a000102d1fd7bc1a8c0035fd6fd7bbfa9fd03009100040011caffff97\
               0070008bfd7bc1a8c0035fd6fd7bbfa9fd030091a8008052000008cac5ffff9700040091fd7bc1a8\
               c0035fd668018052007c081bc0035fd6a8018052007c081bc0035fd60010000bc0035fd6e88761b2\
               0000088bc0035fd6fd7bbfa9fd030091000040d2f2ffff9700fc4193fd7bc1a8c0035fd6fd7bbfa9\
               fd030091000040d2eeffff9700f07dd3fd7bc1a8c0035fd6fd7bbfa9fd030091000040d2eaffff97\
               00fc4293eaffff9700f87fd3fd7bc1a8c0035fd6fd7bbea9f30b00f9fd03009128018052f303012a\
               000008ca87ffff971f00136bf30b40f9e0179f1afd7bc2a8c0035fd6fd7bbea9f30b00f9fd030091\
               68018052f303012a000008ca7bffff970800130bf30b40f9007d4093fd7bc2a8c0035fd6680080d2\
               2800c0f2007c089bc0035fd6fd7bbea9f30b00f9fd030091a8018052f303012a000008caf6ffff97\
               1f00136bf30b40f9e0179f1afd7bc2a8c0035fd6fd7bbea9f30b00f9fd030091000c40d2f30301aa\
               60ffff97080400111f0113ebf30b40f9e0179f1afd7bc2a8c0035fd6",
        functions: &[
            ("z32m", 0x0, 0x8),
            ("zn", 0x8, 0x8),
            ("zw", 0x10, 0xc),
            ("ru", 0x1c, 0x8),
            ("si", 0x24, 0xc),
            ("lb", 0x30, 0xc),
            ("lq", 0x3c, 0xc),
            ("w", 0x48, 0x8),
            ("cz", 0x50, 0x20),
            ("ci", 0x70, 0x1c),
            ("cp", 0x8c, 0x1c),
            ("cru", 0xa8, 0x20),
            ("cs", 0xc8, 0x20),
            ("cb", 0xe8, 0x20),
            ("cq", 0x108, 0x1c),
            ("cw", 0x124, 0x20),
            ("z6", 0x144, 0xc),
            ("z7", 0x150, 0xc),
            ("z8", 0x15c, 0x8),
            ("sink", 0x164, 0xc),
            ("rsh", 0x170, 0x1c),
            ("lsh", 0x18c, 0x1c),
            ("arg", 0x1a8, 0x24),
            ("eqv", 0x1cc, 0x30),
            ("ar", 0x1fc, 0x30),
            ("w64", 0x22c, 0x10),
            ("eqw", 0x23c, 0x30),
            ("eqz", 0x26c, 0x30),
        ],
        checked: &[
            "z32m", "zn", "lb", "cz", "ci", "cp", "cs", "lq", "cq", "cw", "z6", "z7", "z8", "rsh", "lsh", "arg", "eqv",
            "ar", "eqw", "eqz",
        ],
    },
    Image {
        name: "aarch64-clang-O0",
        arch: Architecture::Aarch64,
        code: "ff4300d1e00700f9e80740f9690080d2087d099b007d4092ff430091c0035fd6ff4300d1e00f00b9\
               e80f40b9a9008052087d091be003082aff430091c0035fd6ff4300d1e00700f9e80740f9e90080d2\
               087d099b007d4092ff430091c0035fd6ff4300d1e00f00b9e80f40b969008052007d091bff430091\
               c0035fd6ff4300d1e00f00b9e80f40b9e9008052007d091bff430091c0035fd6ff4300d1e00f00b9\
               e80f40b969008052087d091b001d0012ff430091c0035fd6ff4300d1e00f00b9e80f40b9a9008052\
               097d091be803092a001d4092ff430091c0035fd6ff8300d1fd7b01a9fd430091e00700f9e80740f9\
               001d0091d3ffff97fd7b41a9ff830091c0035fd6ff8300d1fd7b01a9fd430091e00700f9e80740f9\
               a90080d2000109cab8ffff9700040091fd7b41a9ff830091c0035fd6ff8300d1fd7b01a9fd430091\
               e00700f9e80740f9000140d2adffff97e803002a08010071e8a79f1a00010012fd7b41a9ff830091\
               c0035fd6ff8300d1fd7b01a9fd430091e00700f9e80740f908090091e003082aa6ffff97080082d2\
               00044091fd7b41a9ff830091c0035fd6ff8300d1fd7b01a9fd430091a0c31fb8a8c35fb800050011\
               aaffff97e803002a00050091fd7b41a9ff830091c0035fd6ff8300d1fd7b01a9fd430091a0c31fb8\
               a8c35fb800050011a5ffff97e803002a087d409300050091fd7b41a9ff830091c0035fd6ff8300d1\
               fd7b01a9fd430091a0c31fb8a8c35fb8000500119fffff97e803002a087d4093000102f1fd7b41a9\
               ff830091c0035fd6ff8300d1fd7b01a9fd430091a0c31fb8a8c35fb8000500119affff97280080d2\
               0800a2f2007c089bfd7b41a9ff830091c0035fd6ff8300d1fd7b01a9fd430091e00700f9e80740f9\
               a90080d2000109ca95ffff9700040091fd7b41a9ff830091c0035fd6ff4300d1e00700f9e80740f9\
               690180d2087d099b007d4092ff430091c0035fd6ff4300d1e00700f9e80740f9a90180d2087d099b\
               007d4092ff430091c0035fd6ff4300d1e00700f9e80740f9290280d2087d099b007d4092ff430091\
               c0035fd6ff4300d1e00700f9e80740f9e97b40b2000109ebff430091c0035fd6ff8300d1fd7b01a9\
               fd430091e00700f9e80740f9000140d2dbffff9700fc4193fd7b41a9ff830091c0035fd6ff8300d1\
               fd7b01a9fd430091e00700f9e80740f9000140d2d8ffff9700f07dd3fd7b41a9ff830091c0035fd6\
               ff8300d1fd7b01a9fd430091e00700f9e80740f9000140d2d5ffff9700fc4293dbffff9700f87fd3\
               fd7b41a9ff830091c0035fd6ff8300d1fd7b01a9fd430091e00700f9e10700b9e80740f9290180d2\
               000109ca0fffff97e803002ae90740b90801096be8179f1a00010012fd7b41a9ff830091c0035fd6\
               ff8300d1fd7b01a9fd430091e00700f9e10700b9e80740f9690180d2000109cafefeff97e803002a\
               e90740b90901090be803092a007d4093fd7b41a9ff830091c0035fd6ff4300d1e00700f9e80740f9\
               690080d22900c0f2007d099bff430091c0035fd6ff8300d1fd7b01a9fd430091e00700f9e10700b9\
               e80740f9a90180d2000109caf0ffff97e803002ae90740b90801096be8179f1a00010012fd7b41a9\
               ff830091c0035fd6ff8300d1fd7b01a9fd430091e00700f9e10300f9e80740f9000d40d2d5feff97\
               e803002a08050011e90340f9080109ebe8179f1a00010012fd7b41a9ff830091c0035fd6",
        functions: &[
            ("z32m", 0x0, 0x20),
            ("zn", 0x20, 0x20),
            ("zw", 0x40, 0x20),
            ("ru", 0x60, 0x1c),
            ("si", 0x7c, 0x1c),
            ("lb", 0x98, 0x20),
            ("lq", 0xb8, 0x24),
            ("w", 0xdc, 0x28),
            ("cz", 0x104, 0x30),
            ("ci", 0x134, 0x38),
            ("cp", 0x16c, 0x34),
            ("cru", 0x1a0, 0x30),
            ("cs", 0x1d0, 0x34),
            ("cb", 0x204, 0x34),
            ("cq", 0x238, 0x34),
            ("cw", 0x26c, 0x30),
            ("z6", 0x29c, 0x20),
            ("z7", 0x2bc, 0x20),
            ("z8", 0x2dc, 0x20),
            ("sink", 0x2fc, 0x1c),
            ("rsh", 0x318, 0x2c),
            ("lsh", 0x344, 0x2c),
            ("arg", 0x370, 0x34),
            ("eqv", 0x3a4, 0x44),
            ("ar", 0x3e8, 0x44),
            ("w64", 0x42c, 0x20),
            ("eqw", 0x44c, 0x44),
            ("eqz", 0x490, 0x44),
        ],
        checked: &[
            "z32m", "zn", "lb", "cz", "ci", "cp", "cs", "lq", "cq", "cw", "z6", "z7", "z8", "rsh", "lsh", "arg", "eqv",
            "ar", "eqw", "eqz",
        ],
    },
];

fn bytes(hex: &str) -> Vec<u8> {
    let hex: String = hex.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
}

/// A relocatable object holding the image's `.text` and its function symbols.
fn object(image: &Image) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, image.arch, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &bytes(image.code), 16);
    for &(name, value, size) in image.functions {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    obj.write().unwrap()
}

fn decompile_all(image: &Image) -> String {
    let path = common::scratch_file(&format!("zextreturn-{}", image.name), "o");
    std::fs::write(&path, object(image)).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna")).args(["decompile-all", path.to_str().unwrap()]).output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "{}: {text}\n{}", image.name, String::from_utf8_lossy(&output.stderr));
    text
}

/// The printed function `name`, from its header comment to the next.
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{text}"));
    let rest = &text[start + 1..];
    &text[start..start + 1 + rest.find("// Function:").unwrap_or(rest.len())]
}

/// Compiles every printed function with the source and runs each checked one
/// against its source over a range of inputs, at -O0 and -O2 with each host
/// compiler.
fn round_trip(image: &Image, text: &str) -> Result<(), String> {
    let printed: String = image.functions.iter().map(|(n, _, _)| function(text, n)).collect();
    let checks: String = ARGS
        .iter()
        .filter(|(n, _)| image.checked.contains(n))
        .map(|(n, arg)| format!("    CHECK({n}({arg}), s_{n}({arg}));\n"))
        .collect();
    let program = format!(
        "#include <stdio.h>\n#include <stdbool.h>\n{SOURCE}\n{printed}\n\
         static const int V[] = {{0, 1, -1, 3, 0x2a, 0x7f, 0x80, 0x2aaaaaab, 0x2aaaaaae, 0x2aaaaaaa, 0x19999998,\n\
         0x55555554, 0x7fffffff, (int)0x80000000, -7, 0x12345678, (int)0xdeadbeef}};\n\
         #define CHECK(got, want) do {{ long long g = (long long)(got), w = (long long)(want); if (g != w) {{ \\\n\
         printf(\"%s v=%#x: %llx, want %llx\\n\", #got, v, g, w); bad++; }} }} while (0)\n\
         int main(void) {{\n  int bad = 0;\n  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {{\n\
         int v = V[i];\n{checks}  }}\n  return bad != 0;\n}}\n"
    );
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let src = common::scratch_file(&format!("zextreturn-{}-{cc}{level}", image.name), "c");
            let exe = src.with_extension("exe");
            std::fs::write(&src, &program).unwrap();
            let out = Command::new(cc)
                .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                .args(common::CC_GCC15_DEMOTE)
                .output()
                .expect("spawn the C compiler");
            if !out.status.success() {
                return Err(format!("{cc} {level} rejects the printed C:\n{}", String::from_utf8_lossy(&out.stderr)));
            }
            let run = Command::new(&exe).output().expect("run the round trip");
            if !run.status.success() {
                return Err(format!("{cc} {level}:\n{}", String::from_utf8_lossy(&run.stdout)));
            }
        }
    }
    Ok(())
}

/// The printed callers compute what the binary does, and so do the callees a
/// caller computes with wider: `z32m`, `zn`, `lq`, `zw` (through the wrapper
/// `w`, which returns its result as it is), and `z6`, `z7` and `z8`, whose
/// callers shift the result before returning it or passing it on, return the
/// whole register,
/// `lb` an `unsigned char`, while `ru` and `si`, whose callers extend the
/// result themselves, still return 32 bits. A caller that compares or adds the
/// low word of a result its callee returns whole (`eqv`, `ar`, and `eqw` over
/// the 64-bit `w64`) truncates the call explicitly, to an unsigned word where
/// it zero-extends a sum (`eqz`).
#[test]
fn a_caller_that_computes_with_the_whole_register_keeps_the_zero_extension() {
    let widened = [
        ("z32m", "unsigned long z32m("),
        ("zn", "unsigned long zn("),
        ("zw", "unsigned long zw("),
        ("lq", "unsigned long lq("),
        ("z6", "unsigned long z6("),
        ("z7", "unsigned long z7("),
        ("z8", "unsigned long z8("),
        ("lb", "unsigned char lb("),
    ];
    for image in IMAGES {
        let text = decompile_all(image);
        round_trip(image, &text).unwrap_or_else(|e| panic!("{}: {e}\n{text}", image.name));
        for (name, ret) in widened {
            assert!(function(&text, name).contains(ret), "{}: {name} is not `{ret}`:\n{text}", image.name);
        }
        for name in ["ru", "si"] {
            let header = function(&text, name).lines().nth(1).unwrap_or_default();
            assert!(header.starts_with("int "), "{}: {name} widened: {header}\n{text}", image.name);
        }
    }
}
