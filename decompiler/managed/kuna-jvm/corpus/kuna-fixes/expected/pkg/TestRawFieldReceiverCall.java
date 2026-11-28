package pkg;

public class TestRawFieldReceiverCall {
   static TestRawFieldReceiverCall.Holder<String> make() {
      return new TestRawFieldReceiverCall.Holder<>();
   }

   int sites(TestRawFieldReceiverCall.Holder<String> var1) {
      TestRawFieldReceiverCall.Holder var2 = new TestRawFieldReceiverCall.Holder();
      var2.next = make();
      int var3 = (Integer)var2.f.apply("x");
      int var4 = (Integer)var2.next.f.apply("y");
      String var5 = (String)var2.next.value;
      return var3 + var4 + var5.length() + var2.next.size() + var1.f.apply("z") + var1.next.value.length();
   }

   interface Fn<A, B> {
      B apply(A var1);
   }

   static class Holder<T> {
      TestRawFieldReceiverCall.Fn<T, Integer> f;
      TestRawFieldReceiverCall.Holder<T> next;
      T value;

      int size() {
         return 0;
      }
   }
}
