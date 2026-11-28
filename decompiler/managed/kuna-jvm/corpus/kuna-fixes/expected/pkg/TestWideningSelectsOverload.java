package pkg;

public class TestWideningSelectsOverload {
   interface Sink<N extends Number> {
      void invoke(N var1);

      void invoke(double var1);

      default void invoke(float value) {
         this.invoke((double)value);
      }
   }

   abstract static class Total implements TestWideningSelectsOverload.Sink<Double> {
      public final void invoke(Double value) {
         this.invoke(value.doubleValue());
      }

      @Override
      public final void invoke(float value) {
         this.invoke((double)value);
      }
   }
}
