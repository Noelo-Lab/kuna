package pkg;

public class TestVarargArrayLength {
   static int count(Object... values) {
      return values.length;
   }

   static int countNames(String prefix, String... names) {
      return names.length;
   }

   static int nulls(int n) {
      return count(new Object[n]);
   }

   static int threeNulls() {
      return countNames("p", new String[3]);
   }

   static int rows(int n) {
      return count(new int[n][]);
   }

   static int none() {
      return count();
   }

   static int emptyArray() {
      return countNames("p", new String[0]);
   }

   static int two(String a) {
      return countNames("p", a, "b");
   }
}
