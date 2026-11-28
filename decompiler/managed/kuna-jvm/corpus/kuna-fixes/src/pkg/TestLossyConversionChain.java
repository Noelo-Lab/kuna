// A widening conversion that rounds stays written under another conversion: javac -g --release 17
package pkg;

public class TestLossyConversionChain {
   static double[] doubles = new double[1];

   static int i() {
      return 16777217;
   }

   static long l() {
      return 9007199254740993L;
   }

   static double asDouble(double d) {
      return d;
   }

   static long viaFloat() {
      return (long) (float) i();
   }

   static long viaDouble() {
      return (long) (double) l();
   }

   static int intViaFloat() {
      return (int) (float) i();
   }

   static float doubleRounded() {
      return (float) (double) l();
   }

   static double floatRounded() {
      return (double) (float) i();
   }

   static double contexts(int i, boolean flag, double d) {
      double x = asDouble((float) i);
      double y = (float) i;
      double z = (float) i * d;
      double t = flag ? (float) i : d;
      doubles[0] = (float) i;
      short narrow = (short) (float) i;
      return x + y + z + t + narrow;
   }

   static long exact(byte b, int i) {
      return (long) (float) b + (long) (double) i;
   }
}
