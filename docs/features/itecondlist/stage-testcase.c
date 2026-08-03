/* Three structurally identical constant-assignment diamonds in a row.
   Indirect calls through a function-pointer parameter keep the object
   relocation-free; the explicit if/else makes both arms COPY a constant into
   the same stack slot, which is the shape iteregion matches. */
int t(int c, void (*g)(void), void (*f)(int))
{
  int x;
  g(); if (c) x = 7; else x = 9; f(x);
  g(); if (c) x = 7; else x = 9; f(x);
  g(); if (c) x = 7; else x = 9; f(x);
  return x;
}
