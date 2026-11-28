package pkg;

public class TestCapturedVariableMerge {
   static int total;

   static {
      Object o = new Object();
      add(() -> o.hashCode() + 1);
   }

   static int first = total;

   static {
      Object ox = "second";
      add(() -> ox.hashCode() + 2);
   }

   static int second = total;

   static void add(TestCapturedVariableMerge.Job job) {
      total = total + job.run();
   }

   void blocks(int k) {
      Object o = new Object();
      add(() -> o.hashCode() + k);
      Object ox = "second" + k;
      add(() -> ox.hashCode() - k);
   }

   int reused(int k) {
      int n = 0;
      int m = k + 1;
      n += m;
      m = k + 2;
      n += m;
      return n;
   }

   interface Job {
      int run();
   }
}
