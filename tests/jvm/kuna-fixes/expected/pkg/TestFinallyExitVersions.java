package pkg;

public class TestFinallyExitVersions {
   public void typed(TestFinallyExitVersions.Src var1) {
      String var2 = "a";

      Object var6;
      try {
         var1.work();
      } finally {
         var6 = var1.obj();
      }

      var1.use(var6);
   }

   public void conditional(TestFinallyExitVersions.Src var1, Object var2) {
      Object var3 = var2;

      try {
         var1.work();
      } finally {
         if (var1.g() > 0) {
            var3 = "s";
         }
      }

      var1.use(var3);
   }

   public void withCatch(TestFinallyExitVersions.Src var1) {
      String var2 = "a";

      Object var9;
      try {
         var1.work();
      } catch (IllegalStateException var7) {
         String var10 = "c";
      } finally {
         var9 = var1.obj();
      }

      var1.use(var9);
   }

   public void nested(TestFinallyExitVersions.Src var1) {
      String var2 = "a";

      try {
         Object var12;
         try {
            var1.work();
         } finally {
            var12 = var1.obj();
         }

         var1.use(var12);
      } finally {
         var2 = "b";
      }

      var1.use(var2);
   }

   public void loop(TestFinallyExitVersions.Src var1) {
      Object var2 = "a";

      while (var1.g() > 0) {
         try {
            if (var1.g() > 5) {
               break;
            }

            var1.work();
         } finally {
            var2 = var1.obj();
         }
      }

      var1.use(var2);
   }

   public void reset(TestFinallyExitVersions.Src var1) {
      Object var2 = var1.next();
      TestFinallyExitVersions.Handle var3 = null;

      while (var2 != null) {
         try {
            var3 = var1.open(var2);
            var1.use(var3);
         } finally {
            if (var3 != null) {
               var3.close();
               var1.use(var3);
               var3 = null;
            }
         }

         var2 = var1.next();
      }
   }

   public Object local(TestFinallyExitVersions.Src var1) {
      boolean var2 = false;

      try {
         Object var3 = var1.next();
         var2 = true;
         return var3;
      } finally {
         int var7 = var1.g();
         if (var7 > 0 && var2) {
            var1.use(var1.obj());
         }
      }
   }

   static class Handle {
      private int closed;

      void close() {
         this.closed++;
      }

      public String toString() {
         return "Handle" + this.closed;
      }
   }

   interface Src {
      void work();

      void use(Object var1);

      int g();

      Object obj();

      TestFinallyExitVersions.Handle open(Object var1);

      Object next();
   }
}
