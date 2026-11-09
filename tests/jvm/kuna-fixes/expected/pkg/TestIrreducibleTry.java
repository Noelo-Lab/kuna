package pkg;

public class TestIrreducibleTry {
   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Irreducible bytecode was duplicated to produce valid code
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   public static int run(int var0, TestIrreducibleTry.Src var1) {
      boolean var10;
      if (var0 == 0) {
         boolean var28 = false /* VF: Semaphore variable */;

         try {
            var28 = true;
            var1.open();
            var28 = false;
         } finally {
            if (var28) {
               var1.close();
            }
         }
      } else {
         boolean var22 = false /* VF: Semaphore variable */;

         try {
            var22 = true;
            var1.resume();
            var22 = false;
         } finally {
            if (var22) {
               var1.close();
            }
         }

         var10 = false;

         try {
            var10 = true;
            var1.tick();
            var10 = false;
         } finally {
            if (var10) {
               var1.close();
            }
         }
      }

      while (true) {
         boolean var16 = false /* VF: Semaphore variable */;

         int var2;
         int var3;
         try {
            var16 = true;
            var2 = var1.next();
            if (var2 == 0) {
               var16 = false;
               break;
            }

            var3 = var1.step();
            var16 = false;
         } finally {
            if (var16) {
               var1.close();
            }
         }

         if (var3 == 0) {
            return var2;
         }

         var10 = false;

         try {
            var10 = true;
            var1.tick();
            var10 = false;
         } finally {
            if (var10) {
               var1.close();
            }
         }
      }

      var1.close();
      return -1;
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Irreducible bytecode was duplicated to produce valid code
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   public static int poll(int var0, TestIrreducibleTry.Src var1) {
      boolean var34;
      boolean var49;
      boolean var94;
      switch (var0) {
         case 0:
            boolean var124 = false /* VF: Semaphore variable */;

            try {
               var124 = true;
               var1.open();
               var124 = false;
               break;
            } finally {
               if (var124) {
                  var1.close();
               }
            }
         case 1:
            var1.resume();
            boolean var64 = false /* VF: Semaphore variable */;

            try {
               var64 = true;
               var1.tick();
               var64 = false;
            } finally {
               if (var64) {
                  var1.close();
               }
            }

            var49 = false;

            int var139;
            try {
               var49 = true;
               int var3 = var1.step();
               var139 = var1.next();
               var49 = false;
            } finally {
               if (var49) {
                  var1.close();
               }
            }

            if (var139 == 0) {
               return 20;
            }

            var34 = false;

            try {
               var34 = true;
               if (var1.step() != 0) {
                  var34 = false;
                  break;
               }

               var139 = var1.next();
               var34 = false;
            } finally {
               if (var34) {
                  var1.close();
               }
            }

            if (var139 == 0) {
               return 30;
            }

            var94 = false;

            try {
               var94 = true;
               var1.tick();
               var94 = false;
               break;
            } finally {
               if (var94) {
                  var1.close();
               }
            }
         case 2:
            var1.resume();
            boolean var79 = false /* VF: Semaphore variable */;

            try {
               var79 = true;
               var1.tick();
               var79 = false;
            } finally {
               if (var79) {
                  var1.close();
               }
            }

            var34 = false;

            int var2;
            try {
               var34 = true;
               if (var1.step() != 0) {
                  var34 = false;
                  break;
               }

               var2 = var1.next();
               var34 = false;
            } finally {
               if (var34) {
                  var1.close();
               }
            }

            if (var2 == 0) {
               return 30;
            }

            var94 = false;

            try {
               var94 = true;
               var1.tick();
               var94 = false;
               break;
            } finally {
               if (var94) {
                  var1.close();
               }
            }
         case 3:
            var1.resume();
            boolean var109 = false /* VF: Semaphore variable */;

            try {
               var109 = true;
               var1.tick();
               var109 = false;
            } finally {
               if (var109) {
                  var1.close();
               }
            }

            var94 = false;

            try {
               var94 = true;
               var1.tick();
               var94 = false;
               break;
            } finally {
               if (var94) {
                  var1.close();
               }
            }
         default:
            return -1;
      }

      while (true) {
         boolean var19 = false /* VF: Semaphore variable */;

         int var141;
         try {
            var19 = true;
            var141 = var1.next();
            var19 = false;
         } finally {
            if (var19) {
               var1.close();
            }
         }

         if (var141 == 0) {
            return 10;
         }

         var49 = false;

         try {
            var49 = true;
            int var144 = var1.step();
            var141 = var1.next();
            var49 = false;
         } finally {
            if (var49) {
               var1.close();
            }
         }

         if (var141 == 0) {
            return 20;
         }

         var34 = false;

         try {
            var34 = true;
            if (var1.step() != 0) {
               var34 = false;
               continue;
            }

            var141 = var1.next();
            var34 = false;
         } finally {
            if (var34) {
               var1.close();
            }
         }

         if (var141 == 0) {
            return 30;
         }

         var94 = false;

         try {
            var94 = true;
            var1.tick();
            var94 = false;
         } finally {
            if (var94) {
               var1.close();
            }
         }
      }
   }

   public interface Src {
      void open();

      int next();

      int step();

      void resume();

      void close();

      void tick();
   }
}
