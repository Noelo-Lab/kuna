/* Stage bytes from GNU binutils 2.46 for powerpc-linux-gnu.
   ${AS} -me500 -o select.o select.S
   ${LD} -Ttext 0x100000 -e choose -o select.elf select.o
   ${OBJDUMP} -d -Me500 select.elf */
.text
.globl choose, sum5, choose_zero
.type choose,@function
choose:
    cmpwi 3,0
    isel 7,5,6,2
    b sum5
.size choose,.-choose

.type sum5,@function
sum5:
    add 3,3,4
    add 3,3,5
    add 3,3,6
    add 3,3,7
    blr
.size sum5,.-sum5

.type choose_zero,@function
choose_zero:
    cmpwi 3,0
    isel 3,0,4,2
    blr
.size choose_zero,.-choose_zero
