package pkg;

public class TestVarMergeLostDefinition {
   public void afterTry(TestVarMergeLostDefinition.Src var1) {
      String var2 = "a";

      TestVarMergeLostDefinition.Item var6;
      try {
         var6 = var1.item();
         var1.work();
      } finally {
         var1.work();
      }

      var1.use(var6);
   }

   public void afterLoop(TestVarMergeLostDefinition.Src var1) {
      String var2 = "a";

      TestVarMergeLostDefinition.Item var3;
      do {
         var3 = var1.item();
      } while (!var1.done());

      var1.use(var3);
   }

   interface Item {
   }

   interface Src {
      void use(Object var1);

      void work();

      boolean done();

      TestVarMergeLostDefinition.Item item();
   }
}
