# kuna dev container — reproducible build + cross-arch fixture environment.
#
# This image is ENV-ONLY: it does NOT COPY the kuna repo. The workspace is mounted
# at runtime (/workspaces/kuna), so the same image serves every branch/worktree.
#
# Base is ubuntu:22.04 deliberately: it matches the host toolchain era so the
# vendored cross-arch ELF test fixtures stay byte-reproducible.
FROM ubuntu:22.04

# Non-interactive apt + sane locale for the build tools.
ENV DEBIAN_FRONTEND=noninteractive \
    LANG=C.UTF-8 \
    LC_ALL=C.UTF-8

# ---------------------------------------------------------------------------
# Single apt layer: core build chain, every cross toolchain WITH its linker
# (each gcc-<triple> pulls binutils-<triple> = the assembler + ld), binary
# inspection tools, and the language/dev tooling. apt lists cleaned at the end.
#
# NOTE: each `gcc-<triple>` package depends on `binutils-<triple>`, so the
# cross *linkers* (e.g. arm-linux-gnueabihf-ld) come in for free — that is the
# key unblock: this image can LINK cross-arch ELFs, not just assemble objects.
#
# Non-ELF toolchains for the multi-format loader work:
#   - `gcc-mingw-w64` => x86_64-/i686-w64-mingw32-gcc: builds idiomatic PE/PE32+
#     executables that IMPORT from DLLs (msvcrt puts/printf) and carry DWARF +
#     Itanium-mangled C++ — needed for the PE import-naming + PE DWARF fixtures.
#   - `lld` (below) ships ld64.lld (Mach-O) and lld-link (PE), so a Mach-O
#     executable can be linked in-container later.
# (osxcross / a full Apple SDK for cross-building Mach-O from C is still out of
# scope — not packaged in apt; build from source if/when needed.)
# ---------------------------------------------------------------------------
RUN apt-get update && apt-get install -y --no-install-recommends \
        # core build
        build-essential \
        make \
        cmake \
        pkg-config \
        gcc \
        g++ \
        clang \
        lld \
        # cross toolchains (each pulls its binutils => assembler + linker)
        gcc-arm-linux-gnueabihf \
        g++-arm-linux-gnueabihf \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu \
        gcc-riscv64-linux-gnu \
        gcc-mips-linux-gnu \
        gcc-mipsel-linux-gnu \
        gcc-mips64el-linux-gnuabi64 \
        gcc-powerpc64le-linux-gnu \
        gcc-sparc64-linux-gnu \
        # Windows/PE cross toolchain (x86_64-/i686-w64-mingw32-gcc + binutils:
        # links importing PE/PE32+ exes for the multi-format loader fixtures)
        gcc-mingw-w64 \
        # binary inspection
        binutils-multiarch \
        file \
        # languages / tooling
        golang-go \
        python3 \
        python3-pip \
        git \
        curl \
        wget \
        ca-certificates \
        gnupg \
        unzip \
        xz-utils \
        gdb \
        sudo \
        less \
        vim \
    && rm -rf /var/lib/apt/lists/*

# ---------------------------------------------------------------------------
# Non-root user `dev` (uid 1000) with passwordless sudo.
# ---------------------------------------------------------------------------
RUN useradd --create-home --shell /bin/bash --uid 1000 dev \
    && echo 'dev ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/dev \
    && chmod 0440 /etc/sudoers.d/dev

# ---------------------------------------------------------------------------
# Rust toolchain, installed AS the dev user and pinned to the host's 1.90.0
# (matches decompiler/Cargo.toml `rust-version = "1.90"`). CARGO_HOME /
# RUSTUP_HOME live under the dev home so the toolchain is owned by `dev`.
# ---------------------------------------------------------------------------
ENV RUSTUP_HOME=/home/dev/.rustup \
    CARGO_HOME=/home/dev/.cargo \
    PATH=/home/dev/.cargo/bin:$PATH

USER dev
WORKDIR /home/dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --no-modify-path --default-toolchain 1.90.0 --profile minimal \
    && rustup toolchain install 1.90.0 \
    && rustup default 1.90.0 \
    && rustup component add rustfmt clippy

# ---------------------------------------------------------------------------
# Image sanity: fail the build if the pinned rustc or the ARM cross-compiler
# (the headline unblock) are not present/working.
# ---------------------------------------------------------------------------
RUN rustc --version | grep -q '1\.90\.0' \
    && cargo --version \
    && arm-linux-gnueabihf-gcc --version

WORKDIR /workspaces/kuna
CMD ["/bin/bash", "-l"]
