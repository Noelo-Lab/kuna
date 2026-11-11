/* kuna xrefs fixture: an address a fixed-width ISA forms across two
 * instructions (AArch64 adrp + add/ldr, RISC-V lui/auipc + addi/lw/sw).
 * Return, read, index and pass a static label's address; load and store a
 * global. audit_pad keeps the label off a page boundary, so the page an
 * adrp/lui forms is not itself the label. */
const char audit_pad[] = "PAD_SO_THE_LABEL_IS_NOT_PAGE_ALIGNED";
const char audit_label[] = "AUDIT_STATIC_LABEL";
volatile const char audit_vlabel[] = "AUDIT_VOLATILE_LABEL";
volatile unsigned int audit_global = 7;
const char *volatile audit_label_ptr = audit_label;

__attribute__((noinline)) int audit_target(int value) { return value + 11; }
int (*volatile audit_func_ptr)(int) = audit_target;
__attribute__((noinline)) const char *audit_direct_label(void) { return audit_label; }
__attribute__((noinline)) int audit_direct_call(int value) { return audit_target(value); }
__attribute__((noinline)) int audit_indirect_call(int value) { return audit_func_ptr(value); }
__attribute__((noinline)) int audit_first_byte(void) { return audit_vlabel[3]; }
__attribute__((noinline)) int audit_indexed_byte(int i) { return audit_vlabel[i]; }
__attribute__((noinline)) int audit_consume(const char *p) { return p[0]; }
__attribute__((noinline)) int audit_pass_label(void) { return audit_consume(audit_label); }
__attribute__((noinline)) unsigned int audit_leaf(unsigned int value) { return value * 3u + audit_global; }
__attribute__((noinline)) void audit_store(unsigned int value) { audit_global = value; }

int main(void) {
  audit_store(5);
  return audit_direct_call(1) + audit_indirect_call(2) + audit_direct_label()[0]
       + audit_first_byte() + audit_indexed_byte(4) + audit_pass_label() + (int)audit_leaf(3);
}
