// Self-contained Mach-O fixture for stack blocks (tests/stages/kuna-objcblocks.xml):
// `-[Runner maker:]` builds a block literal on its frame (`_NSConcreteStackBlock`
// isa, the invoke and descriptor addresses stored after it) and sends it;
// `-[Runner tally:]` captures an object, a double, a `__block` cell and an int
// (copy/dispose helpers, `_Block_object_assign` flags 3 and 8, a `Block_byref`
// cell built on the frame). The invoke functions are stripped local symbols.
// See fixtures/README.md ("Mach-O stack blocks") for the build recipe.
__attribute__((objc_root_class))
@interface Runner { long base; }
- (long)run:(long)v with:(long (^)(long))b;
- (long)maker:(long)k;
- (long)tally:(long)n scale:(double)s;
- (long)base;
@end
@implementation Runner
- (long)run:(long)v with:(long (^)(long))b { return b(v) + base; }
- (long)maker:(long)k { long c = base + k; return [self run:k with:^long(long x) { return x * c; }]; }
- (long)base { return base; }
- (long)tally:(long)n scale:(double)s {
  __block long total = 0;
  int step = (int)n + 1;
  long r = [self run:n with:^long(long x) { total += x * step + (long)s + [self base]; return total; }];
  return r + total;
}
@end
int main(void) { return 0; }
