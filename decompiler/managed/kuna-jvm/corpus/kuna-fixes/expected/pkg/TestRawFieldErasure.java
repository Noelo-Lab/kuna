package pkg;

public class TestRawFieldErasure {
   static <M extends TestRawFieldErasure.Unit<M>> void fill(M[] target, M[] source) {
   }

   abstract static class Base<E extends Comparable<E>> {
   }

   static class Holder<N extends TestRawFieldErasure.Unit<N>> {
      void copy(TestRawFieldErasure.UnitStore<N> into, TestRawFieldErasure.Base<N> from) {
         TestRawFieldErasure.fill(into.data, (N[])((TestRawFieldErasure.UnitStore)from).data);
      }
   }

   abstract static class Store<E extends Comparable<E>> extends TestRawFieldErasure.Base<E> {
      E[] data;
   }

   interface Unit<U extends TestRawFieldErasure.Unit<U>> extends Comparable<U> {
   }

   abstract static class UnitStore<N extends TestRawFieldErasure.Unit<N>> extends TestRawFieldErasure.Store<N> {
   }
}
