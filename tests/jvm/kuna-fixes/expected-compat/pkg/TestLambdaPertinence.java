package pkg;

public class TestLambdaPertinence {
   static final TestLambdaPertinence.Order<TestLambdaPertinence.Box<?>> BY_SIZE = TestLambdaPertinence.Order.comparingInt(TestLambdaPertinence.Box::size)
      .reversed();

   static <T> TestLambdaPertinence.It<T> single(T element) {
      return () -> element;
   }

   TestLambdaPertinence.It<TestLambdaPertinence.Task<Integer>> tasks(TestLambdaPertinence.Flow<TestLambdaPertinence.Name> names) {
      return names.map(n -> () -> n.length()).iterator();
   }

   TestLambdaPertinence.It<TestLambdaPertinence.Action> actions(TestLambdaPertinence.Flow<TestLambdaPertinence.Name> names) {
      return names.map(n -> n::clear).iterator();
   }

   TestLambdaPertinence.Order<TestLambdaPertinence.Box<String>> byValue() {
      return TestLambdaPertinence.Order.<TestLambdaPertinence.Box<String>>comparingInt(b -> b.value.length()).reversed();
   }

   int first() {
      return single(() -> 1).next().call();
   }

   TestLambdaPertinence.Order<TestLambdaPertinence.Name> byLength() {
      return TestLambdaPertinence.Order.comparingInt(TestLambdaPertinence.Name::length).reversed();
   }

   TestLambdaPertinence.It<Integer> lengths(TestLambdaPertinence.Flow<TestLambdaPertinence.Name> names) {
      return names.map(TestLambdaPertinence.Name::length).iterator();
   }

   TestLambdaPertinence.It<String> texts(TestLambdaPertinence.Flow<TestLambdaPertinence.Name> names) {
      return names.map(n -> n.text).iterator();
   }

   interface Action {
      void run();
   }

   static class Box<V> {
      V value;

      int size() {
         return 1;
      }
   }

   interface Flow<T> {
      <R> TestLambdaPertinence.Flow<R> map(TestLambdaPertinence.Fn<? super T, ? extends R> var1);

      TestLambdaPertinence.It<T> iterator();
   }

   interface Fn<A, R> {
      R apply(A var1);
   }

   interface It<T> {
      T next();
   }

   static class Name {
      String text;

      int length() {
         return this.text.length();
      }

      void clear() {
         this.text = "";
      }
   }

   interface Order<T> {
      int compare(T var1, T var2);

      default TestLambdaPertinence.Order<T> reversed() {
         return (a, b) -> this.compare(b, a);
      }

      static <T> TestLambdaPertinence.Order<T> comparingInt(TestLambdaPertinence.ToInt<? super T> key) {
         return (a, b) -> key.applyAsInt(a) - key.applyAsInt(b);
      }
   }

   interface Task<V> {
      V call();
   }

   interface ToInt<T> {
      int applyAsInt(T var1);
   }
}
