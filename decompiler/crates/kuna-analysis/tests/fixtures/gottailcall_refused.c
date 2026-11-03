/* GH-904: a tail call through a GOT slot that the post-check refuses. */
int five(int a, int b, int c, int d, int e);
int fwd5(int a, int b, int c, int d, int e) { return five(a, b, c, d, e + 1); }
int five(int a, int b, int c, int d, int e) { return a + b * c + d * e; }
