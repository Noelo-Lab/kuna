package pkg;

public class TestFinallyRethrowOutside {
   static void work() {
   }

   // $VF: Could not inline inconsistent finally blocks
   // Please report this to the kuna issue tracker, at https://github.com/Noelo-Lab/kuna/issues with a copy of the class file (if you have the rights to distribute it!)
   public int breakInFinally(int a) {
      while (true) {
         label59: {
            label60: {
               try {
                  try {
                     work();
                     break label60;
                  } catch (RuntimeException e) {
                     a--;
                  }
               } catch (Throwable var7) {
                  if (a <= 3) {
                     throw var7;
                  }

                  return a;
               }

               if (a <= 3) {
                  break label59;
               }

               return a;
            }

            if (a > 3) {
               break;
            }
         }

         a += 2;
      }

      return a;
   }
}
