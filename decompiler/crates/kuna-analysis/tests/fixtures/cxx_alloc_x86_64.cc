// A C++ program whose `new`, `new[]`, `delete` and `delete[]` are imports, as
// they are from libstdc++. `make_array` returns `new[]`'s result and
// `drop_array` passes its pointer to `delete[]`.
// Build: zig cc -target x86_64-linux-gnu.2.38 -shared -fPIC -o libcxx_alloc_x86_64_stub.so cxx_alloc_x86_64_stub.c
//        zig c++ -target x86_64-linux-gnu.2.38 -O1 -g0 -fno-sanitize=undefined -fno-exceptions -nostdlib++ -o cxx_alloc_x86_64 cxx_alloc_x86_64.cc -L. -lcxx_alloc_x86_64_stub
//        llvm-objcopy --strip-debug cxx_alloc_x86_64
struct Node { long value; Node *next; };
__attribute__((noinline)) Node *make(long v) { Node *n = new Node; n->value = v; n->next = nullptr; return n; }
__attribute__((noinline)) void drop(Node *n) { delete n; }
__attribute__((noinline)) long *make_array(unsigned long count) { return new long[count]; }
__attribute__((noinline)) void drop_array(long *p) { delete[] p; }
int main(int argc, char **) {
    Node *n = make(argc);
    long *a = make_array(argc + 4);
    a[0] = n->value;
    long r = a[0];
    drop_array(a);
    drop(n);
    return (int)r;
}
