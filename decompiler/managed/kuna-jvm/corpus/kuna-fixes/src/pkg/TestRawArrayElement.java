package pkg;

public class TestRawArrayElement {
   interface Pred<T> {
      boolean test(T t);
   }

   static class Bag<T> {
      T first;

      boolean removeIf(Pred<? super T> p) {
         return false;
      }
   }

   int sites(int k) {
      Bag<String> bag = new Bag<>();
      Bag<String>[] bags = new Bag[]{bag};
      bags[0].removeIf(t -> t.length() > k);
      Bag<String>[][] grid = new Bag[][]{bags};
      grid[0][0].removeIf(t -> t.isEmpty());
      return bags[0].first.length();
   }
}
