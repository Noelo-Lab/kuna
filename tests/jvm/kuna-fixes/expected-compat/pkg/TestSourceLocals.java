package pkg;

import java.util.List;

public class TestSourceLocals {
   private final String[] names;
   private final int id;
   private final Object lock = new Object();

   TestSourceLocals(String[] names, int id) {
      this.names = names;
      this.id = id;
   }

   public int hashCode() {
      int result = this.id;
      return 31 * result + this.names.length;
   }

   static <T> List<T> cast(List<?> list) {
      return (List<T>)list;
   }

   static String pick(boolean c, String a, String b) {
      return c ? a : b;
   }

   static String[] pair(String a, String b) {
      return new String[]{a, b};
   }

   static int sum(List<String> src) {
      int total = 0;

      for (String s : src.subList(1, 3)) {
         total += s.length();
      }

      return total;
   }

   static int count(List<String> src) {
      int n = 0;

      for (String s : src) {
         n += s.length();
      }

      return n;
   }

   int guarded() {
      try {
         return this.names.length;
      } finally {
         log("done");
      }
   }

   int locked() {
      synchronized (this.lock) {
         return this.id;
      }
   }

   static long copy(TestSourceLocals.Source from, TestSourceLocals.Sink to) {
      char[] buf = new char[16];
      long total = 0L;

      int nRead;
      while ((nRead = from.read(buf)) != -1) {
         to.append(buf, nRead);
         total += nRead;
      }

      return total;
   }

   static void log(String message) {
   }

   interface Sink {
      void append(char[] var1, int var2);
   }

   interface Source {
      int read(char[] var1);
   }
}
