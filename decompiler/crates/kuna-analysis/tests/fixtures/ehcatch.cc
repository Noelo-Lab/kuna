// Fixture source for the exception-table tests (kuna `ehcatch`).
//
//   clang++ -O2 -fPIC -fexceptions -c ehcatch.cc -o x.o
//   ld.lld -shared -o ehcatch_x86_64.so x.o
//   clang++ --target=armv7a-linux-gnueabihf -O2 -fPIC -fexceptions -c ehcatch.cc -o a.o
//   ld.lld -shared -o ehcatch_arm.so a.o
//   clang++ --target=armv7a-linux-gnueabihf -mthumb -O2 -fPIC -fexceptions -c ehcatch.cc -o t.o
//   ld.lld -shared -o t.so t.o && llvm-strip -o ehcatch_thumb_stripped.so t.so
void work(int);

struct __attribute__((visibility("hidden"))) Odd {
  int code;
};

__attribute__((noinline)) static int twice(int a) {
  work(a);
  return a * 2;
}

int guarded(int v) {
  try {
    work(v);
  } catch (int e) {
    return e;
  }
  return 0;
}

int sorted(int v) {
  try {
    work(v);
  } catch (const Odd &e) {
    return e.code;
  } catch (...) {
    return -1;
  }
  return twice(v);
}
