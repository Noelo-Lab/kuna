package pkg;

public class TestCapturedListAdd {
   static <T> T check(T t) {
      return t;
   }

   static <E extends Exception> TestCapturedListAdd.Bag<TestCapturedListAdd.Task<? extends E>> collect(
      TestCapturedListAdd.Source<? extends TestCapturedListAdd.Task<? extends E>> tasks
   ) {
      TestCapturedListAdd.Bag<TestCapturedListAdd.Task<? extends E>> bag = new TestCapturedListAdd.Bag<>();
      tasks.each(t -> bag.add(check(t)));
      return bag;
   }

   static class Bag<E> {
      void add(E e) {
      }
   }

   interface Sink<T> {
      void put(T var1);
   }

   interface Source<T> {
      void each(TestCapturedListAdd.Sink<? super T> var1);
   }

   interface Task<E extends Exception> {
      void run() throws E;
   }
}
