package pkg;

public class TestDeduplicatedLambdaCaptures {
   static int total;

   static void add(TestDeduplicatedLambdaCaptures.Job var0) {
      total = total + var0.run();
   }

   void first(int var1) {
      for (int var2 = 0; var2 < var1; var2++) {
         Object var3 = new Object();
         add(() -> var3.hashCode());
      }
   }

   void second(int var1, Object var2) {
      add(() -> var2.hashCode());
   }

   void third(Object var1, Object var2, Object var3) {
      String var4 = var3.toString();
      add(() -> var4.hashCode());
   }

   static void fourth(Object var0, long var1, Object var3) {
      String var4 = var3.toString();
      add(() -> var4.hashCode());
      String var5 = var0.toString();
      add(() -> var5.hashCode());
   }

   void nested(Object var1, Object var2) {
      String var3 = var2.toString();
      add(() -> {
         String var1x = var3.toString();
         add(() -> var1x.hashCode());
         return 1;
      });
   }

   void two(int var1, Object var2, Object var3) {
      String var4 = var3.toString();
      String var5 = var2.toString();
      add(() -> var4.hashCode() + var5.hashCode());
      add(() -> var5.hashCode() + var4.hashCode());
   }

   interface Job {
      int run();
   }
}
