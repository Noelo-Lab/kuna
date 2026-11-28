package pkg;

public class TestLambdaLocalReads<E extends CharSequence> {
   interface Fn<A, R> {
      R apply(A a);
   }

   interface Bi<A, B, R> {
      R apply(A a, B b);
   }

   interface Ov<A> {
      int m(A a);

      default int m(String s) {
         return 0;
      }
   }

   static class Box<T> {
      T value;

      T get() {
         return this.value;
      }
   }

   E item;

   static int use(Fn<String, Integer> f) {
      return 0;
   }

   public int fits(String x, CharSequence cs, Object o) {
      Fn<String, Integer> f = s -> s.length();
      Fn<CharSequence, Integer> g = s -> s.length();
      return f.apply(x) + f.apply((String) o) + f.apply(null) + g.apply("lit") + g.apply(cs);
   }

   public int arrays(String x) {
      Bi<String, int[], Integer> b = (s, a) -> s.length() + a[0];
      return b.apply(x, new int[1]);
   }

   public int typeVariable(E e) {
      Fn<E, Character> h = s -> s.charAt(0);
      return h.apply(e) + h.apply(this.item);
   }

   public int boxed(int n) {
      Fn<Integer, Integer> i = v -> v + 1;
      return i.apply(n);
   }

   public int passed(String x) {
      Fn<String, Integer> f = s -> s.hashCode();
      return use(f) + f.apply(x);
   }

   public int checked(Box<String> box) {
      Fn<String, Integer> f = s -> s.hashCode();
      return f.apply(box.get());
   }

   public int overloaded(CharSequence cs) {
      Ov<CharSequence> o = s -> s.hashCode();
      return o.m(cs);
   }
}
