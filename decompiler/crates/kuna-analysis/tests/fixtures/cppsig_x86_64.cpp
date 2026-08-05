// Fixture source for the `cppsig` gate (demangled C++ signature application).
//
// Built as a STRIPPED shared library, so there is no DWARF and no `.symtab` —
// only the exported `.dynsym` mangled names survive, which is exactly the
// situation the feature exists for (a stripped C++ shared library).
//
//   g++ -O0 -shared -fPIC -fno-inline -o cppsig_x86_64.so cppsig_x86_64.cpp
//   strip --strip-all cppsig_x86_64.so
//
// Every function below is one shape of the `this` decision:
//
//   sig::Account::Account   constructor          -> PROVEN this
//   sig::Account::~Account  destructor           -> PROVEN this
//   sig::Account::balance   const member         -> PROVEN this  (`_ZNK`)
//   sig::Account::deposit   plain member         -> AMBIGUOUS (inferred only)
//   sig::Account::rate      STATIC member        -> AMBIGUOUS, and adding a
//                                                   `this` here would be WRONG
//   sig::combine            namespaced free fn   -> AMBIGUOUS, no `this`
//   sig_global              unqualified free fn  -> PROVEN there is no `this`
//
// `-fno-inline` and the `volatile` sink keep every body reachable and distinct so
// the decompiler cannot fold them together.

namespace sig {

class Ledger;

class Account {
 public:
  Account(int opening);
  ~Account();
  int balance() const;
  int deposit(Ledger *book, int amount);
  static int rate(int basis);

 private:
  int cents_;
  int count_;
};

volatile int sink;

Account::Account(int opening) : cents_(opening), count_(0) { sink = opening; }

Account::~Account() { sink = cents_; }

int Account::balance() const { return cents_; }

int Account::deposit(Ledger *book, int amount) {
  sink = (int)(long)book;
  cents_ += amount;
  count_ += 1;
  return cents_;
}

int Account::rate(int basis) { return basis * 3 + 1; }

// Deliberately calls no member function: an intra-library call to an exported
// member would emit a PLT stub carrying the SAME mangled name, and `load
// function <name>` would then resolve to the stub instead of the body.
int combine(Account *a, int extra) {
  sink = (int)(long)a;
  return extra * 7;
}

}  // namespace sig

int sig_global(char *text, int n) {
  sig::sink = n;
  return (int)text[0] + n;
}
