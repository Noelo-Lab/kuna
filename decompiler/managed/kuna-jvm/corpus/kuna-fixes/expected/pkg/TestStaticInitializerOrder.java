package pkg;

public class TestStaticInitializerOrder {
   static int base;
   static int next = base + 1;
   static int[] sizes;
   static String[] names = new String[]{"a", "b"};
   static int first = 3;
   static int second = first + 1;

   static {
      base = 5;
      sizes = new int[names.length];
   }
}
