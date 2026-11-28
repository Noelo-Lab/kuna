package pkg;

public class TestExitCopiedIntoTry {
   static int wrap(int v) {
      return v * 2;
   }

   public static int breakToReturn(String[] args) {
      int ret = 0;
      for (String arg : args) {
         try {
            switch (arg.length()) {
               case 1:
                  ret = 1;
            }
            break;
         } catch (RuntimeException ex) {
         }
      }
      return wrap(ret);
   }

   public static int breakToReturnFinally(int[] a) {
      int n = 0;
      for (int x : a) {
         try {
            switch (x) {
               case 0:
                  continue;
               case 1:
                  n = x;
            }
            break;
         } finally {
            n += 10;
         }
      }
      return n;
   }

   public static int breakToReturnFinallyReads(int[] a) {
      int n = 0;
      for (int x : a) {
         try {
            switch (x) {
               case 0:
                  continue;
               case 1:
                  n = x;
            }
            break;
         } finally {
            System.out.println(n);
         }
      }
      return n;
   }
}
