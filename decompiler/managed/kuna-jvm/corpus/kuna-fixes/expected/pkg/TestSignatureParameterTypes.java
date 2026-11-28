package pkg;

public class TestSignatureParameterTypes<K, V extends CharSequence> {
   TestSignatureParameterTypes.Ref rawRoot;

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

   @SafeVarargs
   static <X> X firstOf(X... var0) {
      return var0.length > 1 ? var0[1] : var0[0];
   }

   interface BiFn<A, B, R> {
      R apply(A var1, B var2);
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
}
