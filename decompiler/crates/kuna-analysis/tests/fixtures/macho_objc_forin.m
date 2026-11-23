// Self-contained Objective-C Mach-O fixture for an `NSFastEnumerationState`
// passed by address: `Bag` is declared but never implemented here, so the
// for-in loop's `countByEnumeratingWithState:objects:count:` send goes through
// an `objc_msgSend$<sel>` stub whose selector only the headers describe. Root
// class so it needs NO macOS SDK. See fixtures/README.md.
__attribute__((objc_root_class)) @interface Bag
- (void)visit:(id)item;
@end
void visit_all(Bag *bag, Bag *sink) {
  for (id item in bag)
    [sink visit:item];
}
int main(void) { return 0; }
