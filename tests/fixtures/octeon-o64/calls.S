/* Stage bytes from big-endian MIPS GNU binutils 2.30.
   ${AS} -EB -mabi=o64 -march=octeon -o calls.o calls.S
   ${LD} -m elf32btsmip -Ttext 0x100000 -e forward -o calls.elf calls.o
   ${OBJDUMP} -d calls.elf */
.set noreorder
.text
.globl forward, combine, next_slot, high_word
.type forward,@function
forward:
    daddiu $sp,$sp,-48
    sd     $ra,40($sp)
    sw     $4,36($sp)
    li     $5,2
    li     $6,3
    li     $7,4
    jal    combine
    li     $8,91
    ld     $ra,40($sp)
    jr     $ra
    daddiu $sp,$sp,48
.size forward,.-forward

.type combine,@function
combine:
    lw     $2,36($sp)
    addu   $2,$2,$7
    addu   $2,$2,$6
    addu   $2,$2,$5
    bbit0  $4,0,1f
    nop
    addiu  $2,$2,11
1:  jr     $ra
    nop
.size combine,.-combine

.type next_slot,@function
next_slot:
    jr     $ra
    addiu  $2,$4,4
.size next_slot,.-next_slot

.type high_word,@function
high_word:
    jr     $ra
    dsrl32 $2,$4,0
.size high_word,.-high_word
