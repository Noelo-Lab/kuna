typedef struct A { unsigned lo:3, hi:5, wide:16, top:8; } A;
typedef struct W { unsigned short w:16; } W;
typedef struct M { unsigned short w:16; double d; } M;

__attribute__((noinline)) void change(A *p) { p->wide = 19; }
unsigned a_save(A *p, unsigned v) { unsigned old = p->wide; p->wide = v; return old + 1; }
unsigned hi_save(A *p, unsigned v) { unsigned old = p->hi; p->hi = v; return old + 1; }
unsigned a_alias(A *p, unsigned char *q) { unsigned old = p->wide; *q = 7; return old + 1; }
unsigned a_call(A *p) { unsigned old = p->wide; change(p); return old + 1; }
unsigned a_phi(A *p, int k) { unsigned r = p->wide; if (k > 3) r = p->top; else if (k < 0) r = p->lo; return r + p->wide; }
unsigned a_swap(A *p) { unsigned old = p->wide; p->wide = p->top; p->top = old; return p->wide + p->top; }
unsigned w_byval(W w) { return w.w + 1; }
unsigned a_byval(A a) { return a.lo + a.hi + a.wide + a.top; }
double m_byval(M m) { return m.w + m.d; }
