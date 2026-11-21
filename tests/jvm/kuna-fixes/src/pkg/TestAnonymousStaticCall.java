package pkg;

public class TestAnonymousStaticCall {
   public interface Counter {
      int next();
   }

   public Counter counter(final int start) {
      return new Counter() {
         private int value = start;

         public int next() {
            Counter inner = new Counter() {
               public int next() {
                  return twice(value) + Counter2.offset();
               }
            };
            this.value = inner.next();
            return this.value;
         }

         private static int twice(int x) {
            return x * 2;
         }
      };
   }

   static class Counter2 {
      static int offset() {
         return 1;
      }
   }
}
