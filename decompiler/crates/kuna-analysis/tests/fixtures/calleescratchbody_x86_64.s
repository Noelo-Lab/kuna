# (kuna) `calleescratchbody` fixture -- RE-friction round 12, need
# `rax-preserving-stack-probe` (challenge 69d7f15a8afd9d6c48b48871, PE x86-64
# `crackme_shroud.exe`, sub_140001000 -> __chkstk @0x1408d0000), reduced to one
# caller/callee pair.
#
# Every compiler frame larger than a page opens "mov eax,SIZE; call __chkstk;
# sub rsp,rax", so the allocation only stays a constant if RAX survives the
# call.  The convention says otherwise -- RAX is the return register and so sits
# in <killedbycall> -- and the probe contradicts it: it READS RAX and never
# writes it.
#
# `chkstk` is MSVC's stack probe verbatim, spill slot and probe loop included.
# It writes RSP, R10 and R11, which the convention names in neither
# <unaffected> nor <killedbycall>: they are scratch, and clobbering scratch is
# not a departure from the convention, so calleepreserves' positive-evidence
# gate never fired here.  What makes its silence about RAX evidence rather than
# a stub's is the pair of saves at 0x1024/0x1028 -- it wrote memory.
#
# With `option calleescratchbody off`, f indexes its frame off the call's
# phantom result:
#     *(unsigned long *)&(&Stack0000000000000008)[-chkstk()] = a0;
# with it on (the default), the allocation is constant and the slot is a local:
#     chkstk(); return a0;
#
# Built with:  as -o f.o calleescratchbody_x86_64.s \
#           && ld -o calleescratchbody_x86_64 f.o
        .text
        .globl f
        .type f, @function
f:
        .byte 0xb8,0x00,0x20,0x00,0x00          # MOV   EAX,0x2000
        call  chkstk                            # CALL  chkstk
        .byte 0x48,0x29,0xc4                    # SUB   RSP,RAX
        .byte 0x48,0x89,0x7c,0x24,0x08          # MOV   [RSP+0x8],RDI
        .byte 0x48,0x8b,0x44,0x24,0x08          # MOV   RAX,[RSP+0x8]
        .byte 0x48,0x81,0xc4,0x00,0x20,0x00,0x00 # ADD  RSP,0x2000
        .byte 0xc3                              # RET
        .size f, .-f
        .balign 16, 0x90

        .globl chkstk
        .type chkstk, @function
chkstk:
        .byte 0x48,0x83,0xec,0x10               # SUB   RSP,0x10
        .byte 0x4c,0x89,0x14,0x24               # MOV   [RSP],R10
        .byte 0x4c,0x89,0x5c,0x24,0x08          # MOV   [RSP+0x8],R11
        .byte 0x4d,0x33,0xdb                    # XOR   R11,R11
        .byte 0x4c,0x8d,0x54,0x24,0x18          # LEA   R10,[RSP+0x18]
        .byte 0x4c,0x2b,0xd0                    # SUB   R10,RAX
        .byte 0x4d,0x0f,0x42,0xd3               # CMOVC R10,R11
        .byte 0x65,0x4c,0x8b,0x1c,0x25,0x10,0x00,0x00,0x00 # MOV R11,GS:[0x10]
        .byte 0x4d,0x3b,0xd3                    # CMP   R10,R11
        .byte 0x73,0x16                         # JNC   +0x16
        .byte 0x66,0x41,0x81,0xe2,0x00,0xf0     # AND   R10W,0xf000
        .byte 0x4d,0x8d,0x9b,0x00,0xf0,0xff,0xff # LEA  R11,[R11-0x1000]
        .byte 0x41,0xc6,0x03,0x00               # MOV   byte ptr [R11],0x0
        .byte 0x4d,0x3b,0xd3                    # CMP   R10,R11
        .byte 0x75,0xf0                         # JNZ   -0x10
        .byte 0x4c,0x8b,0x14,0x24               # MOV   R10,[RSP]
        .byte 0x4c,0x8b,0x5c,0x24,0x08          # MOV   R11,[RSP+0x8]
        .byte 0x48,0x83,0xc4,0x10               # ADD   RSP,0x10
        .byte 0xc3                              # RET
        .size chkstk, .-chkstk
