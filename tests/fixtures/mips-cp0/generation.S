/* GNU binutils 2.30, big-endian MIPS.
   ${AS} -EB -64 -march=octeon3 generation.S -o generation.o
   ${OBJDUMP} -d generation.o
   Both stage profiles use the same instruction bytes. */
.set noreorder
.text
.globl transfer
.type transfer,@function
transfer:
    mtc0    $4,$31,4
    mtc0    $5,$31,5
    ehb
    mfc0    $8,$31,4
    sd      $8,0($6)
    dmfc0   $2,$31,5
    jr      $31
    nop
.size transfer,.-transfer

.globl replace
.type replace,@function
replace:
    dmtc0   $4,$31,5
    ehb
    dmtc0   $5,$31,5
    ehb
    dmfc0   $2,$31,5
    jr      $31
    nop
.size replace,.-replace
