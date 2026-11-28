package pkg;

public class TestCapturedSlotReuse {
   static int total;

   static void add(TestCapturedSlotReuse.Job var0) {
      total = total + var0.run();
   }

   void nested(Object var1, int var2) {
      String var3 = var1.toString();
      if (var2 > 0) {
         add(() -> 1);
         return;
      }

      add(() -> var3.hashCode());
      if (var2 < -5) {
         String var4 = "second" + var2;
         add(() -> var4.hashCode() - var2);
      }
   }

   void elseBranch(Object var1, int var2) {
      String var3 = var1.toString();
      total = total + var3.hashCode();
      if (var2 < -5) {
         total--;
      } else {
         try {
            String var5 = "second" + var2;
            add(() -> var5.hashCode() - var2);
         } catch (RuntimeException var4) {
            total++;
         }
      }
   }

   void loop(Object var1, int var2) {
      String var3 = var1.toString();
      total = total + var3.hashCode();

      for (int var5 = 0; var5 < var2; var5++) {
         String var4 = "second" + var5;
         add(() -> var4.hashCode() - var2);
      }
   }

   void switched(Object var1, int var2) {
      String var3 = var1.toString();
      total = total + var3.hashCode();
      switch (var2) {
         case 1:
            String var4 = "second" + var2;
            add(() -> var4.hashCode() - var2);
            break;
         default:
            total++;
      }
   }

   interface Job {
      int run();
   }
}
