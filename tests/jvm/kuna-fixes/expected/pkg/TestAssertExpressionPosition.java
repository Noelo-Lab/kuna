package pkg;

public class TestAssertExpressionPosition {
   private boolean done;
   private int left;

   static void each(TestAssertExpressionPosition.Pairs pairs) {
      pairs.accept("k", "v");
   }

   static void put(Object value, TestAssertExpressionPosition.Sink sink) {
      sink.accept(value);
   }

   static void run(TestAssertExpressionPosition.Runnable r) {
      r.run();
   }

   void consistent() {
      each((key, value) -> {
         assert value.equals(key) : "mismatch key=[" + key + "]";
      });
   }

   void noFailure(Object item) {
      put(item, e -> {
         assert e == null : e;
      });
   }

   void unreachable(String name) {
      run(() -> {
         assert false : name;
      });
   }

   void plain() {
      run(() -> {
         assert this.left > 0;
      });
   }

   int drain(TestAssertExpressionPosition.Source s) {
      int n = 0;

      Object ref;
      while ((ref = s.ref()) != null) {
         n += ref.hashCode();
         boolean more = true;

         while (more) {
            n += s.next();
            if (s.empty()) {
               more = false;
            }

            if (this.done) {
               n -= this.left;
               more = false;
            }

            assert s.valid();
         }
      }

      return n;
   }

   interface Pairs {
      void accept(String var1, Object var2);
   }

   interface Runnable {
      void run();
   }

   interface Sink {
      void accept(Object var1);
   }

   interface Source {
      int next();

      boolean empty();

      boolean valid();

      Object ref();
   }
}
