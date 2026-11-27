package pkg;

import pkglib.Ext;

public class TestLambdaReturnTypeArgument {
   public interface Coll<E> {
      int size();
   }

   public interface Lst<E> extends Coll<E> {
   }

   public interface Fn<A, B> {
      B apply(A a);
   }

   public static final class Opt<T> {
      final T value;

      Opt(T value) {
         this.value = value;
      }

      public <U> Opt<U> map(Fn<? super T, ? extends U> f) {
         return new Opt<>(value == null ? null : f.apply(value));
      }

      public T orElse(T other) {
         return value != null ? value : other;
      }
   }

   static <E> Lst<E> empty() {
      return () -> 0;
   }

   static int count(Opt<Ext> ext) {
      Coll<String> names = ext.map(Ext::names).orElse(empty());
      return names.size();
   }

   static int lengths(Opt<String> s) {
      return s.map(x -> x.length()).orElse(0);
   }
}
