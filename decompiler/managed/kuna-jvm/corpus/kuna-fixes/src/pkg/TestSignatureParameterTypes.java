package pkg;

public class TestSignatureParameterTypes<K, V extends CharSequence> {
   interface BiFn<A, B, R> {
      R apply(A a, B b);
   }

   static class Ref<X, Y> {
      X key;
      Y value;
   }

   static class Holder<T> {
      T value;
   }

   static class Seq<T> {
      T get(int i) {
         return null;
      }
   }

   static class Bag<E> {
      void addAll(Bag<? extends E> b) {
      }
   }

   interface Fn<A, R> {
      R apply(A a);
   }

   static class Chain<X> {
      static <Z> Chain<Z> concat(Chain<? extends Z> a, Chain<? extends Z> b) {
         return null;
      }

      Seq<X> collect() {
         return null;
      }
   }

   static <F, R> Chain<Holder<R>> mapAll(Chain<F> c, Fn<F, R> f) {
      return null;
   }

   static class Store {
      Object get() {
         return null;
      }
   }

   class Inner<T> {
      Holder<? super T> get() {
         return null;
      }

      Seq<T> all() {
         return null;
      }
   }

   Ref rawRoot;

   Ref rawRef() {
      return this.rawRoot;
   }

   <R> R useSnapshot(BiFn<Ref<K, V>, long[], R> f) {
      Ref<K, V> r = rawRef();
      return f.apply(r, new long[1]);
   }

   public <T> Holder<? extends T> superCast(Inner<T> inner) {
      return (Holder<? extends T>) inner.get();
   }

   public <T> T element(Inner<T> inner) {
      return inner.all().get(0);
   }

   static <T> T first(Seq<T> s) {
      return s.get(0);
   }

   static String rawFirst(Seq raw) {
      return (String) first(raw);
   }

   static int typedFirst(Seq<String> s) {
      return first(s).length() + s.get(0).length();
   }

   public V value(Ref<K, V> ref) {
      V v = ref.value;
      return v.length() > 0 ? v : ref.value;
   }

   public void merge(Bag<String> into, Holder<Bag<String>> h, Store s) {
      into.addAll(h.value);
      into.addAll((Bag<String>) s.get());
   }

   <T1 extends V, T2 extends V> Seq<Holder<V>> mergeValues(Chain<T1> a, Chain<T2> b) {
      return Chain.concat(mapAll(a, v -> (V) v), mapAll(b, v -> (V) v)).collect();
   }

   @SafeVarargs
   static <X> X firstOf(X... xs) {
      return xs.length > 1 ? xs[1] : xs[0];
   }
}
