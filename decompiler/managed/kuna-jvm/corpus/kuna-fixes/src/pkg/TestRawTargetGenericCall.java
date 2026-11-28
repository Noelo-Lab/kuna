package pkg;

public class TestRawTargetGenericCall {
   interface Fn<A, B> {
      B apply(A a);
   }

   interface Cmp<T> {
      int compare(T a, T b);

      static <T, U extends Comparable<? super U>> Cmp<T> comparing(Fn<? super T, ? extends U> key) {
         return (a, b) -> 0;
      }
   }

   static class Box<T> {
      T held;

      void sort(Cmp<? super T> c) {
      }
   }

   static void takeRaw(Cmp c) {
   }

   int sites(int k) {
      Box<String> b = new Box<>();
      b.sort(Cmp.comparing(s -> s.length() + k));
      takeRaw(Cmp.comparing((String s) -> s.trim()));
      Cmp<String> trimmed = Cmp.comparing(s -> s.trim());
      b.sort(trimmed);
      Box<int[]> arrays = new Box<>();
      arrays.sort(Cmp.comparing(a -> a.length));
      return b.held.length();
   }
}
