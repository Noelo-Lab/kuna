package pkg;

public class TestMixedWidening {
   boolean compare(int i, long l, float f, double d) {
      return (double)((float)i) == i
         || (double)((float)l) == l
         || (long)((float)i) == i
         || (double)f == i
         || f == i
         || (double)f == l
         || f < (float)l
         || (double)i < f
         || d == f
         || i != (float)l;
   }

   double arithmetic(int i, long l, float f, double d) {
      return (double)((float)i) + i + ((double)f * l - f * (float)l) + ((double)((float)l) + l) + ((double)f + i) + (f + i) + (d + f + i + l) + i / (float)l;
   }

   float floats(int i, long l, float f, short s, char c, byte b) {
      return i + (float)l + (f + i) + f * (float)l + (s + f) + c * f + (b - f) + (float)(i + l) + i * (float)l;
   }

   long longs(int i, long l, short s, char c) {
      return i + l + (long)i * i + i * i + l + s + ((long)c << 2) + ((long)i << i) + (long)i + s;
   }

   double ternary(boolean b, int i, long l, float f) {
      double x = b ? (double)((float)i) : i;
      double y = b ? (double)f : l;
      float z = b ? i : f;
      return x + y + z + (b ? (double)((float)l) : l);
   }

   double compound(int i, long l, float f, double d) {
      d += (float)i;
      d *= (double)f * i;
      f += (float)l;
      f -= i * (float)l;
      l += i;
      return d + f + l;
   }
}
