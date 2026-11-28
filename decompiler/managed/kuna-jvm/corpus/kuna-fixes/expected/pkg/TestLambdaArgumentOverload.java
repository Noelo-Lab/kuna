package pkg;

public class TestLambdaArgumentOverload {
   private int total;

   static <T> int select(Class<?> type, TestLambdaArgumentOverload.Lookup<T> lookup) {
      return 0;
   }

   static int select(Class<?> type, TestLambdaArgumentOverload.Filter filter) {
      return 1;
   }

   void add(int value) {
      this.total += value;
   }

   void add(int[] values) {
      this.total += values.length;
   }

   public void drain(TestLambdaArgumentOverload.Source source) {
      source.forEach((TestLambdaArgumentOverload.IntSink)(this::add));
      source.forEach((TestLambdaArgumentOverload.IntSink)(v -> this.total -= v));
   }

   public int lookup() {
      return select(String.class, (TestLambdaArgumentOverload.Lookup<Boolean>)(name -> name.isEmpty() ? Boolean.TRUE : null));
   }

   public int filter() {
      return select(String.class, (TestLambdaArgumentOverload.Filter)(name -> name.isEmpty()));
   }

   public int single(TestLambdaArgumentOverload.Lookup<String> lookup) {
      return lookup.inspect("x").length();
   }

   interface Filter {
      boolean matches(String var1);
   }

   interface IntSink {
      void accept(int var1);
   }

   interface Lookup<T> {
      T inspect(String var1);
   }

   interface Sink<T> {
      void accept(T var1);
   }

   interface Source {
      void forEach(TestLambdaArgumentOverload.IntSink var1);

      void forEach(TestLambdaArgumentOverload.Sink<? super Integer> var1);
   }
}
