package pkg;

public class TestCapturedSlotReuse {
   interface Job {
      int run();
   }

   static int total;

   static void add(Job job) {
      total += job.run();
   }

   void nested(Object a, int k) {
      {
         Object o = a.toString();
         if (k > 0) {
            add(() -> 1);
            return;
         }

         add(() -> o.hashCode());
      }

      if (k < -5) {
         Object p = "second" + k;
         add(() -> p.hashCode() - k);
      }
   }

   void elseBranch(Object a, int k) {
      {
         Object o = a.toString();
         total += o.hashCode();
      }

      if (k < -5) {
         total--;
      } else {
         try {
            Object p = "second" + k;
            add(() -> p.hashCode() - k);
         } catch (RuntimeException e) {
            total++;
         }
      }
   }

   void loop(Object a, int k) {
      {
         Object o = a.toString();
         total += o.hashCode();
      }

      for (int i = 0; i < k; i++) {
         Object p = "second" + i;
         add(() -> p.hashCode() - k);
      }
   }

   void switched(Object a, int k) {
      {
         Object o = a.toString();
         total += o.hashCode();
      }

      switch (k) {
         case 1:
            Object p = "second" + k;
            add(() -> p.hashCode() - k);
            break;
         default:
            total++;
      }
   }
}
