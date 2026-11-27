package pkg;

public class TestNestedLambdaCapturedGeneric<T> {
   interface Sink<A> {
      void accept(A a);
   }

   interface Pair<A, B> {
      void accept(A a, B b);
   }

   interface Mapper<A, R> {
      R apply(A a);
   }

   <N, V> TestNestedLambdaCapturedGeneric<T> usingPairs(Pair<T, Pair<N, V>> pairs) {
      return this;
   }

   public <E, N, V> TestNestedLambdaCapturedGeneric<T> extracted(
      Pair<T, Sink<E>> elements, Mapper<E, N> names, Mapper<E, V> values
   ) {
      return this.usingPairs((instance, pairs) -> elements.accept(instance, e -> pairs.accept(names.apply(e), values.apply(e))));
   }
}
