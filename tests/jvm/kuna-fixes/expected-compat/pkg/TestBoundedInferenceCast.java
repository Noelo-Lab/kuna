package pkg;

public class TestBoundedInferenceCast {
   static <E> TestBoundedInferenceCast.Cmp<E> orNatural(TestBoundedInferenceCast.Cmp<E> c) {
      return c != null ? c : TestBoundedInferenceCast.Ord.natural();
   }

   static <E> int copy(E a, E b) {
      TestBoundedInferenceCast.Ord<E> o = TestBoundedInferenceCast.Ord.natural();
      return o.compare(a, b);
   }

   static <E> TestBoundedInferenceCast.Cmp<? super E> reversed() {
      return TestBoundedInferenceCast.Ord.natural().reverse();
   }

   static TestBoundedInferenceCast.Cmp<Object> objects() {
      return TestBoundedInferenceCast.Ord.natural();
   }

   static <E extends TestBoundedInferenceCast.Rank<E>> TestBoundedInferenceCast.Cmp<E> ranked() {
      return TestBoundedInferenceCast.Ord.natural();
   }

   public static void main(String[] args) {
      System.out
         .println(orNatural(null).compare("a", "b") + copy(1, 2) + reversed().compare(null, null) + objects().compare(1, 2) + (ranked() == null ? 1 : 0));
   }

   interface Cmp<T> {
      int compare(T var1, T var2);
   }

   static class Ord<T> implements TestBoundedInferenceCast.Cmp<T> {
      static <C extends TestBoundedInferenceCast.Rank> TestBoundedInferenceCast.Ord<C> natural() {
         return new TestBoundedInferenceCast.Ord();
      }

      @Override
      public int compare(T a, T b) {
         return 0;
      }

      <S extends T> TestBoundedInferenceCast.Ord<S> reverse() {
         return new TestBoundedInferenceCast.Ord<>();
      }
   }

   interface Rank<T> {
      int rank(T var1);
   }
}
