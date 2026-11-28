package pkg;

public class TestStaticInitAfterCall {
   static int a;
   static int b;
   static final StringBuilder LOG = new StringBuilder();
   static String c;
   static int d = 3;
   static int e;

   static {
      init();
      b = a + 1;
      c = LOG.toString();
   }

   static int f = next();

   static void init() {
      a = 5;
      LOG.append("init");
   }

   static int next() {
      return ++d;
   }

   static {
      e = d * 2;
   }
}
