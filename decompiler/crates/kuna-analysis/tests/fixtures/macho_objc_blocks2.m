// Self-contained Mach-O fixture for block recovery (tests/stages/kuna-objcblocks2.xml):
// `-[Chain build:]` builds three stack blocks in a row, each sent before the next
// is built, so the shared flags word stays in a callee-saved `d` register across
// the calls; `sum` (a C function) builds one whose invoke builds a nested block;
// `pick` references a global block (`_NSConcreteGlobalBlock`); `-[Chain call:with:]`
// calls its block parameter. The invokes are stripped local symbols.
// See fixtures/README.md ("Mach-O block recovery") for the build recipe.
__attribute__((objc_root_class))
@interface Chain { long base; }
- (long)call:(long)v with:(long (^)(long))b;
- (long)build:(long)k;
@end
extern long take(long (^)(long));
@implementation Chain
- (long)call:(long)v with:(long (^)(long))b { return b(v) + base; }
- (long)build:(long)k {
  long a = [self call:k with:^long(long x) { return x + k; }];
  long b = [self call:a with:^long(long x) { return x * k; }];
  return [self call:b with:^long(long x) { return x - (long)self; }];
}
@end
long sum(long n) {
  return take(^long(long x) { return take(^long(long y) { return x + y + n; }); });
}
long pick(void) { return take(^long(long x) { return x + 1; }); }
int main(void) { return 0; }
