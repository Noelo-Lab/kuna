package pkg;

public class TestVarargsSpreadResolution {
   static String build(String... args) {
      return "varargs:" + args.length;
   }

   static String build() {
      return build();
   }

   static int count(Object... args) {
      return args == null ? -1 : args.length;
   }

   static int wrapArray(String[] callbacks) {
      return count(callbacks);
   }

   static int castArray(String[] addresses) {
      return count(addresses);
   }

   static int castNull() {
      return count(null);
   }

   static int castTable(String[][] table) {
      return count(table);
   }

   static int castObjects(Object[] values) {
      return count((Object)values);
   }

   static int ints(int[] values) {
      return count(values);
   }

   static int two(String[] a, String[] b) {
      return count(a, b);
   }

   static String pick(Object o) {
      return "object";
   }

   static String pick(String... s) {
      return "strings:" + s.length;
   }

   static String pickOne() {
      return pick("a");
   }

   static String pickTwo() {
      return pick("a", "b");
   }

   static String name(String n, Object... rest) {
      return n + rest.length;
   }

   static String name(String n, Integer i) {
      return n + "integer";
   }

   static String nameString() {
      return name("x", "y");
   }

   static String nameBoxed() {
      return name("x", 4);
   }
}
