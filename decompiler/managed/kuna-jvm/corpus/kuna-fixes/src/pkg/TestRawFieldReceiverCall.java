package pkg;

public class TestRawFieldReceiverCall {
   interface Fn<A, B> {
      B apply(A a);
   }

   static class Holder<T> {
      Fn<T, Integer> f;
      Holder<T> next;
      T value;

      int size() {
         return 0;
      }
   }

   static Holder<String> make() {
      return new Holder<>();
   }

   int sites(Holder<String> param) {
      Holder<String> h = new Holder<>();
      h.next = make();
      int a = h.f.apply("x");
      int b = h.next.f.apply("y");
      String v = h.next.value;
      return a + b + v.length() + h.next.size() + param.f.apply("z") + param.next.value.length();
   }
}
