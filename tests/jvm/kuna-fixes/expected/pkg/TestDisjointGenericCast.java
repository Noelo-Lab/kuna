package pkg;

public class TestDisjointGenericCast {
   static final TestDisjointGenericCast.Coll<Object, ?, TestDisjointGenericCast.Lst<Object>> TO_LIST = new TestDisjointGenericCast.Coll<Object, Object, TestDisjointGenericCast.Lst<Object>>() {};
   static final TestDisjointGenericCast.Span<TestDisjointGenericCast.Key> ALL = new TestDisjointGenericCast.Span<>();

   static <E> TestDisjointGenericCast.Coll<E, ?, TestDisjointGenericCast.Lst<E>> toList() {
      return (TestDisjointGenericCast.Coll)TO_LIST;
   }

   static <C extends TestDisjointGenericCast.Key<?>> TestDisjointGenericCast.Span<C> all() {
      return (TestDisjointGenericCast.Span)ALL;
   }

   static TestDisjointGenericCast.Lst<TestDisjointGenericCast.Lst<?>> collect() {
      return new TestDisjointGenericCast.Lst<>();
   }

   static <T> TestDisjointGenericCast.Lst<TestDisjointGenericCast.Lst<? super T>> rawTypes() {
      return (TestDisjointGenericCast.Lst)collect();
   }

   static <E> TestDisjointGenericCast.Lst<E> same(TestDisjointGenericCast.Lst<Object> list) {
      return (TestDisjointGenericCast.Lst<E>)list;
   }

   interface Coll<T, A, R> {
   }

   interface Key<T> {
   }

   static class Lst<E> {
   }

   static class Span<C extends TestDisjointGenericCast.Key> {
   }
}
