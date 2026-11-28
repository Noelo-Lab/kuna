package pkg;

import java.util.function.Supplier;

public class TestSyntheticReceiverCapture {
   static int total;

   static void add(TestSyntheticReceiverCapture.Job var0) {
      total = total + var0.run();
   }

   void only(Object var1, Object var2, Object var3) {
      String var4 = var3.toString();
      add(() -> var4.hashCode());
   }

   void sup(Object var1, Object var2, Object var3) {
      String var4 = var3.toString();
      Supplier var5 = () -> var4.toString();
      add(() -> ((String)var5.get()).length());
   }

   int twoArgs(String var1, String var2) {
      String var3 = var1.trim();
      add(() -> var3.indexOf(var2));
      return total;
   }

   void parameter(Object var1, Object var2) {
      add(() -> var2.hashCode());
   }

   void reassigned(Object var1, Object var2) {
      Object var3 = var2;
      add(() -> var3.hashCode() + 1);
   }

   void self() {
      TestSyntheticReceiverCapture var1 = this;
      add(() -> var1.hashCode() + 2);
   }

   interface Job {
      int run();
   }
}
