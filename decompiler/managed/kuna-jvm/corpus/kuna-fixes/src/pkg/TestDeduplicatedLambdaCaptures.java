package pkg;

public class TestDeduplicatedLambdaCaptures {
   interface Job {
      int run();
   }

   static int total;

   static void add(Job job) {
      total += job.run();
   }

   void first(int k) {
      for (int i = 0; i < k; i++) {
         Object o = new Object();
         add(() -> o.hashCode());
      }
   }

   void second(int k, Object first) {
      Object o = first;
      add(() -> o.hashCode());
   }

   void third(Object a, Object b, Object c) {
      Object o = c.toString();
      add(() -> o.hashCode());
   }

   static void fourth(Object a, long wide, Object b) {
      Object o = b.toString();
      add(() -> o.hashCode());
      Object p = a.toString();
      add(() -> p.hashCode());
   }

   void nested(Object a, Object b) {
      Object o = b.toString();
      add(() -> {
         Object inner = o.toString();
         add(() -> inner.hashCode());
         return 1;
      });
   }

   void two(int k, Object x, Object y) {
      Object p = y.toString();
      Object q = x.toString();
      add(() -> p.hashCode() + q.hashCode());
      add(() -> q.hashCode() + p.hashCode());
   }
}
