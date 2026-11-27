package pkg;

import pkglib.Ext;

public class TestLambdaReturnTypeArgument {
   static <E> TestLambdaReturnTypeArgument.Lst<E> empty() {
      return () -> 0;
   }

   static int count(TestLambdaReturnTypeArgument.Opt<Ext> ext) {
      TestLambdaReturnTypeArgument.Coll<String> names = ext.map(Ext::names).orElse(empty());
      return names.size();
   }

   static int lengths(TestLambdaReturnTypeArgument.Opt<String> s) {
      return s.<Integer>map(x -> x.length()).orElse(0);
   }

   public interface Coll<E> {
      int size();
   }

   public interface Fn<A, B> {
      B apply(A var1);
   }

   public interface Lst<E> extends TestLambdaReturnTypeArgument.Coll<E> {
   }

   public static final class Opt<T> {
      final T value;

      Opt(T value) {
         this.value = value;
      }

      public <U> TestLambdaReturnTypeArgument.Opt<U> map(TestLambdaReturnTypeArgument.Fn<? super T, ? extends U> f) {
         return (TestLambdaReturnTypeArgument.Opt<U>)(new TestLambdaReturnTypeArgument.Opt<>(this.value == null ? null : f.apply(this.value)));
      }

      public T orElse(T other) {
         return this.value != null ? this.value : other;
      }
   }
}
