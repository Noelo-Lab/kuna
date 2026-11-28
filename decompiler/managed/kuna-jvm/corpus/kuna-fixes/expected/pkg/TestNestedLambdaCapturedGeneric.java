package pkg;

public class TestNestedLambdaCapturedGeneric<T> {
   <N, V> TestNestedLambdaCapturedGeneric<T> usingPairs(TestNestedLambdaCapturedGeneric.Pair<T, TestNestedLambdaCapturedGeneric.Pair<N, V>> var1) {
      return this;
   }

   public <E, N, V> TestNestedLambdaCapturedGeneric<T> extracted(
      TestNestedLambdaCapturedGeneric.Pair<T, TestNestedLambdaCapturedGeneric.Sink<E>> var1,
      TestNestedLambdaCapturedGeneric.Mapper<E, N> var2,
      TestNestedLambdaCapturedGeneric.Mapper<E, V> var3
   ) {
      return this.usingPairs((var3x, var4) -> var1.accept(var3x, var3xx -> var4.accept(var2.apply(var3xx), var3.apply(var3xx))));
   }

   interface Mapper<A, R> {
      R apply(A var1);
   }

   interface Pair<A, B> {
      void accept(A var1, B var2);
   }

   interface Sink<A> {
      void accept(A var1);
   }
}
