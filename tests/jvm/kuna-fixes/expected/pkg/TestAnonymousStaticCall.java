package pkg;

public class TestAnonymousStaticCall {
   public TestAnonymousStaticCall.Counter counter(final int start) {
      return new TestAnonymousStaticCall.Counter() {
         private int value = start;

         @Override
         public int next() {
            TestAnonymousStaticCall.Counter inner = new TestAnonymousStaticCall.Counter() {
               @Override
               public int next() {
                  return twice(value) + TestAnonymousStaticCall.Counter2.offset();
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

   public interface Counter {
      int next();
   }

   static class Counter2 {
      static int offset() {
         return 1;
      }
   }
}
