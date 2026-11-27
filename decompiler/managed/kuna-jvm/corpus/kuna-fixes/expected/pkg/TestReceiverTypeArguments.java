package pkg;

public class TestReceiverTypeArguments {
   static final TestReceiverTypeArguments.Setting<Integer> MODE = new TestReceiverTypeArguments.Setting<>();
   static final TestReceiverTypeArguments.Setting<String> NAME = new TestReceiverTypeArguments.Setting<>();

   static TestReceiverTypeArguments.Iter<TestReceiverTypeArguments.Setting<?>> settings() {
      return TestReceiverTypeArguments.Lst.<TestReceiverTypeArguments.Setting<?>>of(MODE).iterator();
   }

   static TestReceiverTypeArguments.Iter<TestReceiverTypeArguments.Setting<?>> both() {
      return TestReceiverTypeArguments.Lst.<TestReceiverTypeArguments.Setting<?>>of(MODE, NAME).iterator();
   }

   static <E> TestReceiverTypeArguments.Sorted<E> copyOf(TestReceiverTypeArguments.Cmp<? super E> cmp, E e) {
      TestReceiverTypeArguments.Sorted<E> sorted = new TestReceiverTypeArguments.Builder<E>(cmp).add(e).build();
      return sorted;
   }

   static TestReceiverTypeArguments.Iter<TestReceiverTypeArguments.Setting<Integer>> plain() {
      return TestReceiverTypeArguments.Lst.of(MODE).iterator();
   }

   public static void main(String[] args) {
      System.out.println(settings().next() == MODE && both().next() == MODE && plain().next() == MODE && copyOf((a, b) -> 0, "x") != null);
   }

   static class Builder<E> {
      final TestReceiverTypeArguments.Cmp<? super E> cmp;

      Builder(TestReceiverTypeArguments.Cmp<? super E> cmp) {
         this.cmp = cmp;
      }

      TestReceiverTypeArguments.Builder<E> add(E e) {
         return this;
      }

      TestReceiverTypeArguments.Sorted<E> build() {
         return new TestReceiverTypeArguments.Sorted<>();
      }
   }

   interface Cmp<T> {
      int compare(T var1, T var2);
   }

   interface Iter<T> {
      T next();
   }

   static class Lst<E> {
      final E first;

      Lst(E first) {
         this.first = first;
      }

      static <E> TestReceiverTypeArguments.Lst<E> of(E e) {
         return new TestReceiverTypeArguments.Lst<>(e);
      }

      static <E> TestReceiverTypeArguments.Lst<E> of(E a, E b) {
         return new TestReceiverTypeArguments.Lst<>(a);
      }

      TestReceiverTypeArguments.Iter<E> iterator() {
         return new TestReceiverTypeArguments.Iter<E>() {
            @Override
            public E next() {
               return Lst.this.first;
            }
         };
      }
   }

   static class Setting<T> {
   }

   static class Sorted<E> {
   }
}
