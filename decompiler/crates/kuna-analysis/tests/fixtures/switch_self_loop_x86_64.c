/* Switches that are their own loop header: every case but the default loops
 * back to the dispatch on a fresh call result, and `self_loop_exit` also has
 * cases that leave the loop to a shared tail.  `mixed_loop` and
 * `direct_mixed` also have a case that restarts an enclosing loop, beside a
 * case tail shared by two cases.  `direct_mixed` and `direct_loop` dispatch
 * on a masked selector with no range check, so Clang's empty `case 0` jumps
 * straight back to the dispatch block.  `nested_switch` puts such a dispatch
 * loop in case 0 of another switch.  The callees are external so the
 * compiler keeps one dispatch at the loop head.
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

void mixed_loop(int x) {
outer:
  t5();
  for (;;) {
    switch (next_selector(x)) {
    case 0: t0(); break;
    case 1: t1(); break;
    case 2: t2(); break;
    case 3: t3(); break;
    case 4: t4(); goto outer;
    case 5: t0(); t1(); break;
    default: return;
    }
  }
}

void direct_mixed(int x) {
outer:
  t5();
  for (;;) {
    switch (next_selector(x) & 7) {
    case 0: break;
    case 1: t1(); break;
    case 2: t2(); break;
    case 3: t3(); break;
    case 4: t4(); goto outer;
    case 5: t0(); t1(); break;
    case 6: t2(); t3(); break;
    default: return;
    }
  }
}

void direct_loop(int x) {
  for (;;) {
    switch (next_selector(x) & 7) {
    case 0: break;
    case 1: t1(); break;
    case 2: t2(); break;
    case 3: t3(); break;
    case 4: t4(); break;
    case 5: t0(); t1(); break;
    case 6: t2(); t3(); break;
    default: return;
    }
  }
}

void nested_switch(int x) {
  t5();
  switch (next_selector(x)) {
  case 0:
    for (;;) {
      switch (next_selector(x)) {
      case 0: t0(); break;
      case 1: t1(); break;
      case 2: t2(); break;
      case 3: t3(); break;
      case 4: t4(); return;
      default: return;
      }
    }
  case 1: t1(); return;
  case 2: t2(); break;
  case 3: t3(); break;
  case 4: t4(); break;
  default: return;
  }
  t5();
}
