package pkg;

import java.util.Iterator;
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
      result = 31 * result + this.names.length;
      return result;
   }

   static <T> List<T> cast(List<?> list) {
      List<T> typed = (List<T>)list;
      return typed;
   }

   static String pick(boolean c, String a, String b) {
      String res = c ? a : b;
      return res;
   }

   static String[] pair(String a, String b) {
      String[] arr = new String[2];
      arr[0] = a;
      arr[1] = b;
      return arr;
   }

   static int sum(List<String> src) {
      int total = 0;
      List<String> part = src.subList(1, 3);

      for (String s : part) {
         total += s.length();
      }

      return total;
   }

   static int count(List<String> src) {
      int n = 0;
      Iterator<String> it = src.iterator();

      while (it.hasNext()) {
         String s = (String)it.next();
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
      int nRead;
      long total = 0L;

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
