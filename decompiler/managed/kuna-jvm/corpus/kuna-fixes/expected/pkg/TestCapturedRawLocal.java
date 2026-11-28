package pkg;

public class TestCapturedRawLocal {
   int sites(final int var1) {
      final TestCapturedRawLocal.Bag var2 = new TestCapturedRawLocal.Bag();
      TestCapturedRawLocal.Runner var3 = new TestCapturedRawLocal.Runner();
      var3.one(
         (TestCapturedRawLocal.Pred<String>)(var2x -> var2.removeIf((TestCapturedRawLocal.Pred<String>)(var2xx -> var2xx.length() > var1 + var2x.length())))
      );
      Runnable var4 = () -> var2.copy().removeIf((TestCapturedRawLocal.Pred<String>)(var0x -> var0x.isEmpty()));
      var4.run();
      Runnable var5 = new Runnable() {
         public void run() {
            var2.removeIf((TestCapturedRawLocal.Pred<String>)(var1xx -> var1xx.length() > var1));
         }
      };
      var5.run();
      return var1;
   }

   int fromParam(TestCapturedRawLocal.Bag<String> var1) {
      Runnable var2 = () -> var1.copy().removeIf(var0x -> var0x.isEmpty());
      var2.run();
      return 0;
   }

   static class Bag<T> {
      boolean removeIf(TestCapturedRawLocal.Pred<? super T> var1) {
         return false;
      }

      TestCapturedRawLocal.Bag<T> copy() {
         return this;
      }
   }

   interface Pred<T> {
      boolean test(T var1);
   }

   static class Runner<T> {
      void one(TestCapturedRawLocal.Pred<T> var1) {
      }
   }
}
