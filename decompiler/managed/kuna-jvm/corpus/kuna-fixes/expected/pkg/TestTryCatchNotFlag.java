package pkg;

public class TestTryCatchNotFlag {
   private int failures;

   static Object make() throws Exception {
      return new Object();
   }

   static TestTryCatchNotFlag.Res open(String name) {
      return name.isEmpty() ? null : new TestTryCatchNotFlag.Res();
   }

   Object create(boolean fallback) {
      Object r;
      try {
         r = make();
      } catch (Throwable ex) {
         if (!fallback) {
            throw new IllegalStateException(ex);
         }

         r = null;
      }

      return r;
   }

   void attempt(boolean quiet) {
      try {
         make();
      } catch (Throwable ex) {
         if (!quiet) {
            this.report(ex);
         }

         this.failures++;
      }
   }

   int read(String name) throws Exception {
      try (TestTryCatchNotFlag.Res res = open(name)) {
         return res.value();
      }
   }

   void report(Throwable t) {
   }

   static class Res implements AutoCloseable {
      int value() {
         return 1;
      }

      public void close() {
      }
   }
}
