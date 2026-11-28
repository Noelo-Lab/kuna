package pkg;

public class TestMixedWidening {
   boolean compare(int i, long l, float f, double d) {
      return (float)i == (double)i || (double)(float)l == l || (long)(float)i == i || (double)f == i || f == i || (double)f == l || f < l
         || (double)i < f || d == f || (float)i != (float)l;
   }

   double arithmetic(int i, long l, float f, double d) {
      return (float)i + (double)i + ((double)f * l - (f * l)) + ((float)l + (double)l) + ((double)f + i) + (f + i) + (d + f + i + l) + (float)i / l;
   }

   float floats(int i, long l, float f, short s, char c, byte b) {
      return (float)i + l + (f + i) + (f * l) + (s + f) + (c * f) + (b - f) + (float)(i + l) + (float)i * (float)l;
   }

   long longs(int i, long l, short s, char c) {
      return (long)i + l + ((long)i * i) + (i * i) + (l + s) + ((long)c << 2) + ((long)i << i) + (i + (long)s);
   }

   double ternary(boolean b, int i, long l, float f) {
      double x = b ? (double)(float)i : (double)i;
      double y = b ? f : l;
      float z = b ? (float)i : f;
      return x + y + z + (b ? (float)l : (double)l);
   }

   double compound(int i, long l, float f, double d) {
      d += (float)i;
      d *= (double)f * i;
      f += l;
      f -= (float)i * l;
      l += i;
      return d + f + l;
   }
}
