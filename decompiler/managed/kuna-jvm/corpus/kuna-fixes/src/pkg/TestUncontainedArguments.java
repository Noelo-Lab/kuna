package pkg;

public class TestUncontainedArguments<T> {
   static class Node<N> {
      Node<? extends Node<N>> self() {
         return (Node<? extends Node<N>>) this;
      }

      void take(Node<? extends Node<N>> n) {
      }

      void relay() {
         take((Node<? extends Node<N>>) this);
      }
   }

   static class Self<S extends Self<S>> {
      Self<? extends Self<S>> me() {
         return this;
      }
   }

   TestUncontainedArguments<? extends TestUncontainedArguments<T>> top() {
      return (TestUncontainedArguments<? extends TestUncontainedArguments<T>>) this;
   }
}
