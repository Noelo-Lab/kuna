package pkg;

public class TestStaticInitAfterCall {
   static int a;
   static int b;
   static final StringBuilder LOG = new StringBuilder();
   static String c;
   static int d = 3;
   static int e;
   static int f;

   static void init() {
      a = 5;
      LOG.append("init");
   }

   static int next() {
      return ++d;
   }

   static {
      init();
      b = a + 1;
      c = LOG.toString();
      f = next();
      e = d * 2;
   }
}
