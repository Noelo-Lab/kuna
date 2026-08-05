// DWARF C++ prototype fixture (`--option cppproto`).
//
// Every construct here produces a `DW_TAG_subprogram` DEFINITION whose name does
// NOT live on the definition DIE, so the name-only DWARF walk drops it whole:
//
//   * `db::inner::scaled_add` — a namespace function defined out of line: the
//     definition carries only `DW_AT_specification`.
//   * `Account::deposit` / `Account::available` — out-of-line member definitions;
//     `available` is `const`, so its `this` is `const Account *const`, four DIEs
//     deep (the qualifier chain that used to blow the type-mapper depth cap and
//     degrade `Account *this` to `void *`).
//   * `Account::bump` — a `const` member with a C++ REFERENCE parameter
//     (`DW_TAG_reference_type`).
//   * `Account::make_id` — a `static` member (no artificial `this`), proving the
//     drop was never about the `this` DIE.
//   * `maxof<int>` / `maxof<double>` — template instantiations. These DO carry
//     their own `DW_AT_name`, but the name is `maxof<int>` while kuna's symbol
//     table normalizes the demangled name to `maxof`, so the by-NAME prototype
//     park misses them; only the address-keyed park binds.
//   * `probe_virtual_call` — a plain free function whose parameter is a pointer
//     to a `DW_TAG_class_type` (`void *` before the class arm).
//
// Built with `g++ -O0 -g -no-pie -fno-pic` (see fixtures/README.md).
#include <cstdio>

namespace db {
namespace inner {
int scaled_add(int a, int b);
}
}

int db::inner::scaled_add(int a, int b) {
    return a * 3 + b * 5;
}

class Shape {
public:
    int tag;
    Shape(int t) : tag(t) {}
    virtual int area(int scale) const { return scale * tag; }
};

class Account {
public:
    int id;
    int balance;
    int limit;

    Account(int i, int b);
    int deposit(int amount);
    int available() const;
    void bump(int &out) const;
    static int make_id(int seed);
};

Account::Account(int i, int b) {
    id = i;
    balance = b;
    limit = b * 2;
}

int Account::deposit(int amount) {
    balance += amount;
    if (balance > limit)
        balance = limit;
    id += 1;
    return balance;
}

int Account::available() const {
    return limit - balance + id;
}

void Account::bump(int &out) const {
    out = out + balance + limit;
}

int Account::make_id(int seed) {
    return seed * 7 + 13;
}

int probe_virtual_call(Shape *s, int scale) {
    return s->area(scale) + s->tag;
}

template <typename T>
T maxof(T a, T b) {
    return a > b ? a : b;
}

int probe_template_int(int a, int b) {
    return maxof<int>(a, b) + maxof<int>(b, a);
}

double probe_template_double(double a, double b) {
    return maxof<double>(a, b) * 2.0;
}

int main(int argc, char **argv) {
    Shape s(argc);
    Account a(3, 4);
    int o = 5;
    a.bump(o);
    int t = db::inner::scaled_add(argc, 2);
    t += probe_virtual_call(&s, argc);
    t += a.deposit(argc) + a.available() + Account::make_id(argc) + o;
    t += probe_template_int(argc, 4);
    t += (int)probe_template_double((double)argc, 2.5);
    printf("%d %s\n", t, argv[0]);
    return t & 0xff;
}
