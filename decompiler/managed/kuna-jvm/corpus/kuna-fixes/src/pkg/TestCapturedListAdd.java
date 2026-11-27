package pkg;

public class TestCapturedListAdd {
   interface Task<E extends Exception> {
      void run() throws E;
   }

   interface Sink<T> {
      void put(T t);
   }

   interface Source<T> {
      void each(Sink<? super T> sink);
   }

   static class Bag<E> {
      void add(E e) {
      }
   }

   static <T> T check(T t) {
      return t;
   }

   static <E extends Exception> Bag<Task<? extends E>> collect(Source<? extends Task<? extends E>> tasks) {
      Bag<Task<? extends E>> bag = new Bag<>();
      tasks.each(t -> bag.add(check(t)));
      return bag;
   }
}
