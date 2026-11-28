package pkg;

public class TestCapturedVariableMerge {
   interface Job {
      int run();
   }

   static int total;

   static {
      Object o = new Object();
      add(() -> o.hashCode() + 1);
   }

   static int first = total;

   static {
      Object o = "second";
      add(() -> o.hashCode() + 2);
   }

   static int second = total;

   static void add(Job job) {
      total += job.run();
   }

   void blocks(int k) {
      {
         Object o = new Object();
         add(() -> o.hashCode() + k);
      }

      {
         Object o = "second" + k;
         add(() -> o.hashCode() - k);
      }
   }

   int reused(int k) {
      int n = 0;
      {
         int m = k + 1;
         n += m;
      }

      {
         int m = k + 2;
         n += m;
      }

      return n;
   }

   void exclusive(Object a, boolean c) {
      Object picked;
      if (a != null) {
         if (c) {
            picked = a.toString();
            add(picked::hashCode);
            return;
         }

         picked = a;
      } else {
         picked = "none";
      }

      add(() -> picked.hashCode() + 3);
   }
}
