package pkg;

public class TestRawTargetLambda<E extends CharSequence> {
   interface Pred<T> {
      boolean test(T t);
   }

   interface Fn<A, R> {
      R apply(A a);
   }

   interface Gen<A> {
      A make(int n);
   }

   interface Bi<A, B> {
      void accept(A a, B b);
   }

   static class Box<T> {
      T value;

      Box(T value, Fn<T, String> namer) {
         this.value = value;
      }

      boolean removeIf(Pred<? super T> p) {
         return false;
      }

      <R> Box<R> map(Fn<? super T, ? extends R> f) {
         return null;
      }

      <A> A[] toArray(Gen<A[]> g) {
         return g.make(0);
      }
   }

   static <T> Box<T> boxOf(T t) {
      return new Box<>(t, x -> "b");
   }

   static void takeRaw(Fn f) {
   }

   public String receiver(String x, int k) {
      Box<String> box = boxOf(x);
      box.removeIf(s -> s.length() > k);
      box.removeIf(s -> s.hashCode() > k);
      return box.value;
   }

   public E typeVariable(E e) {
      Box<E> box = boxOf(e);
      box.removeIf(s -> s.length() > 0);
      return box.value;
   }

   public int[] arrays(int[] a) {
      Box<int[]> box = boxOf(a);
      box.removeIf(v -> v.length > 0);
      return box.value;
   }

   public String constructor(String x) {
      Box<String> box = new Box<>(x, s -> s.trim());
      return box.value;
   }

   public int reference(String x) {
      Box<String> box = boxOf(x);
      Object m = box.map(String::length);
      return box.value.length() + m.hashCode();
   }

   public int chain(String x) {
      Box<String> box = boxOf(x);
      Integer[] lengths = box.map(s -> s.length()).toArray(n -> new Integer[n]);
      return lengths.length + box.value.length();
   }

   public void declared() {
      takeRaw((Fn<String, Integer>) s -> s.length());
      takeRaw(s -> s.hashCode());
   }

   public void lambdaParameter() {
      Bi<Box<String>, Integer> f = (b, i) -> {
         Integer[] a = b.map(s -> s.length() + i).toArray(n -> new Integer[n]);
      };
   }
}
