package pkg;

public class TestStaticInitializerOrder {
   static int base;
   static int next;
   static int[] sizes;
   static String[] names = {"a", "b"};
   static int first = 3;
   static int second = first + 1;

   static {
      next = base + 1;
      base = 5;
      sizes = new int[names.length];
   }
}
