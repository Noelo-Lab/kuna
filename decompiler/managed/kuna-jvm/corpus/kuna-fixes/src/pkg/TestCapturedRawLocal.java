package pkg;

public class TestCapturedRawLocal {
   interface Pred<T> {
      boolean test(T t);
   }

   static class Bag<T> {
      boolean removeIf(Pred<? super T> p) {
         return false;
      }

      Bag<T> copy() {
         return this;
      }
   }

   static class Runner<T> {
      void one(Pred<T> p) {
      }
   }

   int sites(int k) {
      Bag<String> bag = new Bag<>();
      Runner<String> r = new Runner<>();
      r.one(s -> bag.removeIf(t -> t.length() > k + s.length()));
      Runnable run = () -> bag.copy().removeIf(t -> t.isEmpty());
      run.run();
      Runnable anon = new Runnable() {
         @Override
         public void run() {
            bag.removeIf(t -> t.length() > k);
         }
      };
      anon.run();
      return k;
   }

   int fromParam(Bag<String> bag) {
      Runnable run = () -> bag.copy().removeIf(t -> t.isEmpty());
      run.run();
      return 0;
   }
}
