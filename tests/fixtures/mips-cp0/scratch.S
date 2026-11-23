/* Big-endian MIPS GNU binutils 2.30.
   ${AS} -EB -64 -march=octeon2 scratch.S -o scratch.o
   ${LD} -EB -m elf64btsmip -e scratch -Ttext 0x1000 scratch.o -o scratch.elf
   ${OBJDUMP} -d scratch.elf
   GNU as SHA-256: 96b3d7b45a26a3b53aba77e56a372d30c8fa92f06092a2ca78a39d1e97ad465a */
.set noreorder
.text
.globl scratch
.type scratch,@function
scratch:
    mtc0    $4,$31,2
    mtc0    $5,$31,3
    ehb
    dmfc0   $8,$31,2
    sd      $8,0($6)
    mfc0    $8,$31,3
    sd      $8,8($6)
    dmfc0   $2,$31,3
    jr      $31
    nop
.size scratch,.-scratch

.globl previous
.type previous,@function
previous:
    dmfc0   $2,$31,2
    mtc0    $4,$31,2
    ehb
    jr      $31
    nop
.size previous,.-previous

.globl read_word
.type read_word,@function
read_word:
    mfc0    $2,$31,3
    jr      $31
    nop
.size read_word,.-read_word

.globl clear_first
.type clear_first,@function
clear_first:
    mtc0    $0,$31,2
    ehb
    jr      $31
    nop
.size clear_first,.-clear_first

.globl around, observe
.type around,@function
around:
    daddiu $sp,$sp,-32
    sd $ra,24($sp)
    mtc0 $4,$31,2
    ehb
    jal observe
    nop
    ehb
    dmfc0 $2,$31,2
    ld $ra,24($sp)
    jr $ra
    daddiu $sp,$sp,32
.size around,.-around

.type observe,@function
observe:
    dmfc0 $2,$31,2
    sd $2,0($6)
    mtc0 $5,$31,2
    ehb
    jr $ra
    nop
.size observe,.-observe

.globl write_twice
.type write_twice,@function
write_twice:
    mtc0 $4,$31,3
    ehb
    mtc0 $5,$31,3
    ehb
    jr $31
    nop
.size write_twice,.-write_twice
