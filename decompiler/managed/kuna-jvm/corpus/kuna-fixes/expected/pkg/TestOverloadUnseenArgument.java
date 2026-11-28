package pkg;

public class TestOverloadUnseenArgument implements OverloadVector<String> {
   double x;

   public double distance(OverloadVector<String> v) {
      return this.distance((OverloadPoint<String>)v);
   }

   public double distance(OverloadPoint<String> p) {
      return Math.abs(((TestOverloadUnseenArgument)p).x - this.x);
   }

   static int take(CharSequence c) {
      return 1;
   }

   static int take(String s) {
      return 2;
   }

   static int both(String s) {
      return take((CharSequence)s) + take(s);
   }
}
