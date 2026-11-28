package pkg;

public class TestAssertExpressionPosition {
   private boolean done;
   private int left;

   interface Pairs {
      void accept(String key, Object value);
   }

   interface Sink {
      void accept(Object value);
   }

   interface Source {
      int next();

      boolean empty();

      boolean valid();

      Object ref();
   }

   static void each(Pairs pairs) {
      pairs.accept("k", "v");
   }

   static void put(Object value, Sink sink) {
      sink.accept(value);
   }

   static void run(Runnable r) {
      r.run();
   }

   interface Runnable {
      void run();
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

   int drain(Source s) {
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
}
