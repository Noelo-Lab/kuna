// Fixture for the anonymous-namespace name-only demangling fix.
//
// Every definition below gets Itanium `_GLOBAL__N_` mangling, whose name-only
// demangling used to reduce `(anonymous namespace)` to an EMPTY `::` component
// and abort the whole load with "Non-global scope has empty name".  The three
// shapes are: an anonymous namespace nested in a named one, a class inside that
// anonymous namespace, and a top-level anonymous namespace (the leading-`::`
// case).
//
//   g++ -O1 -o anon_namespace_x86_64 anon_namespace_x86_64.cpp
namespace outer {
namespace {
__attribute__((noinline)) int nested_helper(int a) { return a * 3; }
struct Widget {
    __attribute__((noinline)) int emit(int v) { return nested_helper(v); }
};
}  // namespace
int entry(int v) {
    Widget w;
    return w.emit(v);
}
}  // namespace outer

namespace {
__attribute__((noinline)) int top_helper(int a) { return a + 1; }
}  // namespace

int main(void) { return outer::entry(2) + top_helper(1); }
