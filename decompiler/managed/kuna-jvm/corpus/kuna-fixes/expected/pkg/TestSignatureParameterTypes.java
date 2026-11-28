package pkg;

public class TestSignatureParameterTypes<K, V extends CharSequence> {
   TestSignatureParameterTypes.Ref rawRoot;

   static <F, R> TestSignatureParameterTypes.Chain<TestSignatureParameterTypes.Holder<R>> mapAll(
      TestSignatureParameterTypes.Chain<F> var0, TestSignatureParameterTypes.Fn<F, R> var1
   ) {
      return null;
   }

   TestSignatureParameterTypes.Ref rawRef() {
      return this.rawRoot;
   }

   <R> R useSnapshot(TestSignatureParameterTypes.BiFn<TestSignatureParameterTypes.Ref<K, V>, long[], R> var1) {
      TestSignatureParameterTypes.Ref var2 = this.rawRef();
      return (R)var1.apply(var2, new long[1]);
   }

   public <T> TestSignatureParameterTypes.Holder<? extends T> superCast(TestSignatureParameterTypes<K, V>.Inner<T> var1) {
      return (TestSignatureParameterTypes.Holder<? extends T>)var1.get();
   }

   public <T> T element(TestSignatureParameterTypes<K, V>.Inner<T> var1) {
      return var1.all().get(0);
   }

   static <T> T first(TestSignatureParameterTypes.Seq<T> var0) {
      return var0.get(0);
   }

   static String rawFirst(TestSignatureParameterTypes.Seq var0) {
      return (String)first(var0);
   }

   static int typedFirst(TestSignatureParameterTypes.Seq<String> var0) {
      return first(var0).length() + var0.get(0).length();
   }

   public V value(TestSignatureParameterTypes.Ref<K, V> var1) {
      V var2 = var1.value;
      return var2.length() > 0 ? var2 : var1.value;
   }

   public void merge(
      TestSignatureParameterTypes.Bag<String> var1,
      TestSignatureParameterTypes.Holder<TestSignatureParameterTypes.Bag<String>> var2,
      TestSignatureParameterTypes.Store var3
   ) {
      var1.addAll(var2.value);
      var1.addAll((TestSignatureParameterTypes.Bag<? extends String>)var3.get());
   }

   <T1 extends V, T2 extends V> TestSignatureParameterTypes.Seq<TestSignatureParameterTypes.Holder<V>> mergeValues(
      TestSignatureParameterTypes.Chain<T1> var1, TestSignatureParameterTypes.Chain<T2> var2
   ) {
      return TestSignatureParameterTypes.Chain.concat(mapAll(var1, var0 -> (V)var0), mapAll(var2, var0 -> (V)var0)).collect();
   }

   @SafeVarargs
   static <X> X firstOf(X... var0) {
      return var0.length > 1 ? var0[1] : var0[0];
   }

   static class Bag<E> {
      void addAll(TestSignatureParameterTypes.Bag<? extends E> var1) {
      }
   }

   interface BiFn<A, B, R> {
      R apply(A var1, B var2);
   }

   static class Chain<X> {
      static <Z> TestSignatureParameterTypes.Chain<Z> concat(
         TestSignatureParameterTypes.Chain<? extends Z> var0, TestSignatureParameterTypes.Chain<? extends Z> var1
      ) {
         return null;
      }

      TestSignatureParameterTypes.Seq<X> collect() {
         return null;
      }
   }

   interface Fn<A, R> {
      R apply(A var1);
   }

   static class Holder<T> {
      T value;
   }

   class Inner<T> {
      TestSignatureParameterTypes.Holder<? super T> get() {
         return null;
      }

      TestSignatureParameterTypes.Seq<T> all() {
         return null;
      }
   }

   static class Ref<X, Y> {
      X key;
      Y value;
   }

   static class Seq<T> {
      T get(int var1) {
         return null;
      }
   }

   static class Store {
      Object get() {
         return null;
      }
   }
}
