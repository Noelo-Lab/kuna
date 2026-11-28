package pkg;

public class TestUncontainedArguments<T> {
   TestUncontainedArguments<? extends TestUncontainedArguments<T>> top() {
      return (TestUncontainedArguments<? extends TestUncontainedArguments<T>>)this;
   }

   static class Node<N> {
      TestUncontainedArguments.Node<? extends TestUncontainedArguments.Node<N>> self() {
         return (TestUncontainedArguments.Node<? extends TestUncontainedArguments.Node<N>>)this;
      }

      void take(TestUncontainedArguments.Node<? extends TestUncontainedArguments.Node<N>> n) {
      }

      void relay() {
         this.take((TestUncontainedArguments.Node<? extends TestUncontainedArguments.Node<N>>)this);
      }
   }

   static class Self<S extends TestUncontainedArguments.Self<S>> {
      TestUncontainedArguments.Self<? extends TestUncontainedArguments.Self<S>> me() {
         return this;
      }
   }
}
