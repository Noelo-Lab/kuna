package pkg;

public class TestRawLambdaSelection {
   interface Pred<T> {
      boolean test(T t);
   }

   interface Fn<A, B> {
      B apply(A a);
   }

   static class Runner<T> {
      void run(Pred<T> p) {
      }

      void run(Fn<T, String> f) {
      }
   }

   static void over(Fn f) {
   }

   static void over(Pred p) {
   }

   static void takeRaw(Fn f) {
   }

   Fn kept;

   void selected(int k) {
      Runner<String> r = new Runner<>();
      r.run((Pred<String>)s -> s.isEmpty());
      r.run((Fn<String, String>)s -> s.trim() + k);
      over((Fn<String, Integer>)s -> s.length() + k);
      over((Pred<int[]>)a -> a.length > k);
      over((Pred)o -> o.hashCode() > k);
   }

   void assigned(int k) {
      Fn<String, Integer> passed = s -> s.length() + k;
      takeRaw(passed);
      Fn<String, Integer> stored = String::length;
      this.kept = stored;
      Fn<Object, Integer> plain = o -> o.hashCode();
      takeRaw(plain);
   }
}
