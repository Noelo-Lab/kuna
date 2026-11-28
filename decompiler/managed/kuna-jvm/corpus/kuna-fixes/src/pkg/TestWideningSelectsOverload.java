// A primitive widening selects an inherited overload beside a generic one: javac -g --release 17
package pkg;

public class TestWideningSelectsOverload {
   interface Sink<N extends Number> {
      void invoke(N value);

      void invoke(double value);

      default void invoke(float value) {
         this.invoke((double) value);
      }
   }

   abstract static class Total implements Sink<Double> {
      public final void invoke(Double value) {
         this.invoke(value.doubleValue());
      }

      public final void invoke(float value) {
         this.invoke((double) value);
      }
   }
}
