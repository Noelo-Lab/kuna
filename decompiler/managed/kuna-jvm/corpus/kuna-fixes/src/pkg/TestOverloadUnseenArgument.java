// A cast selects the overload when the argument's type is not in the class context: javac -g --release 17
package pkg;

interface OverloadPoint<S> {
   double distance(OverloadPoint<S> p);
}

interface OverloadVector<S> extends OverloadPoint<S> {
   double distance(OverloadVector<S> v);
}

public class TestOverloadUnseenArgument implements OverloadVector<String> {
   double x;

   public double distance(OverloadVector<String> v) {
      return distance((OverloadPoint<String>) v);
   }

   public double distance(OverloadPoint<String> p) {
      return Math.abs(((TestOverloadUnseenArgument) p).x - x);
   }

   static int take(CharSequence c) {
      return 1;
   }

   static int take(String s) {
      return 2;
   }

   static int both(String s) {
      return take((CharSequence) s) + take(s);
   }
}
