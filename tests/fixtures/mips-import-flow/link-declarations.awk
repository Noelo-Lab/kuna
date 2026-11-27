# Keep LLVM's call-origin DIEs while assembling its single-call witnesses as
# O32 PIC. The assembler expands jal to a CALL16 load and a JALR relocation.
{ sub(/\.option[[:space:]]+pic0/, ".option pic2"); print }
/^# %bb\.0:/ { print "\t.set push\n\t.set noreorder\n\t.cpload $25\n\t.set pop" }
