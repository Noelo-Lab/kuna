/* Switches that are their own loop header: every case but the default loops
 * back to the dispatch on a fresh call result, and `self_loop_exit` also has
 * cases that leave the loop to a shared tail.  The callees are external so
 * the compiler keeps one dispatch at the loop head.
 *
 *   gcc -O2 -fno-pie -fcf-protection=none -c switch_self_loop_x86_64.c -o switch_self_loop_gcc_x86_64.o
 *   clang -O2 -fno-pie -fcf-protection=none -c switch_self_loop_x86_64.c -o switch_self_loop_clang_x86_64.o
 */
extern int next_selector(int);
extern void t0(void), t1(void), t2(void), t3(void), t4(void), t5(void);

void self_loop(int x) {
  for (;;) {
    switch (next_selector(x)) {
    case 0: t0(); break;
    case 1: t1(); break;
    case 2: t2(); break;
    case 3: t3(); break;
    case 4: t4(); break;
    case 5: t5(); break;
    default: return;
    }
  }
}

void self_loop_exit(int x) {
  for (;;) {
    switch (next_selector(x)) {
    case 0: t0(); break;
    case 1: t1(); break;
    case 2: t2(); goto out;
    case 3: t3(); break;
    case 4: t4(); goto out;
    case 5: t5(); break;
    default: return;
    }
  }
out:
  t5();
  t0();
}
