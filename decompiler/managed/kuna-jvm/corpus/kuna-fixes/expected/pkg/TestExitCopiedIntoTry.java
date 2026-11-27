package pkg;

public class TestExitCopiedIntoTry {
   static int wrap(int v) {
      return v * 2;
   }

   public static int breakToReturn(String[] args) {
      int ret = 0;

      label21:
      for (String arg : args) {
         try {
            switch (arg.length()) {
               case 1:
                  ret = 1;
               default:
                  break label21;
            }
         } catch (RuntimeException var7) {
         }
      }

      return wrap(ret);
   }

   public static int breakToReturnFinally(int[] a) {
      int n = 0;

      label50:
      for (int x : a) {
         try {
            switch (x) {
               case 0:
                  break;
               case 1:
                  n = x;
               default:
                  break label50;
            }
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
                  break;
               case 1:
                  n = x;
                  return n;
               default:
                  return n;
            }
         } finally {
            System.out.println(n);
         }
      }

      return n;
   }
}
