package pkg;

public class TestDisjointGenericCast {
   interface Coll<T, A, R> {
   }

   interface Key<T> {
   }

   static class Lst<E> {
   }

   @SuppressWarnings("rawtypes")
   static class Span<C extends Key> {
   }

   static final Coll<Object, ?, Lst<Object>> TO_LIST = new Coll<Object, Object, Lst<Object>>() {
   };
   @SuppressWarnings("rawtypes")
   static final Span<Key> ALL = new Span<>();

   @SuppressWarnings("unchecked")
   static <E> Coll<E, ?, Lst<E>> toList() {
      return (Coll) TO_LIST;
   }

   @SuppressWarnings("unchecked")
   static <C extends Key<?>> Span<C> all() {
      return (Span) ALL;
   }

   static Lst<Lst<?>> collect() {
      return new Lst<>();
   }

   @SuppressWarnings("unchecked")
   static <T> Lst<Lst<? super T>> rawTypes() {
      return (Lst) collect();
   }

   @SuppressWarnings("unchecked")
   static <E> Lst<E> same(Lst<Object> list) {
      return (Lst<E>) list;
   }
}
