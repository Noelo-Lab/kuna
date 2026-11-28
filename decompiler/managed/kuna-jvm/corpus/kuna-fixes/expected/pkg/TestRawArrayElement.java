package pkg;

public class TestRawArrayElement {
   int sites(int var1) {
      TestRawArrayElement.Bag var2 = new TestRawArrayElement.Bag();
      TestRawArrayElement.Bag[] var3 = new TestRawArrayElement.Bag[]{var2};
      var3[0].removeIf((TestRawArrayElement.Pred<String>)(var1x -> var1x.length() > var1));
      TestRawArrayElement.Bag[][] var4 = new TestRawArrayElement.Bag[][]{var3};
      var4[0][0].removeIf((TestRawArrayElement.Pred<String>)(var0 -> var0.isEmpty()));
      return ((String)var3[0].first).length();
   }

   static class Bag<T> {
      T first;

      boolean removeIf(TestRawArrayElement.Pred<? super T> var1) {
         return false;
      }
   }

   interface Pred<T> {
      boolean test(T var1);
   }
}
