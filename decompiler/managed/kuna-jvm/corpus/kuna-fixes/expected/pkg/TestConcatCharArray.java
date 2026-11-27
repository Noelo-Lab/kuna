package pkg;

public class TestConcatCharArray {
   static String valueOf(char[] cs) {
      return "x" + String.valueOf(cs) + "y";
   }

   static String builder(char[] cs) {
      return "x" + String.valueOf(cs) + "y";
   }
}
