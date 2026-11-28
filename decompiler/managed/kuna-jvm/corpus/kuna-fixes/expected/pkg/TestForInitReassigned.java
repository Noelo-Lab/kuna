package pkg;

public class TestForInitReassigned {
   Object[] last;

   void copy(Object[] var1, long var2, long var4) {
      long var6 = Math.min(var2, var4);
      var6 = Math.min(var1.length, var6);

      for (int var8 = 0; var8 < var6; var8++) {
         this.last[var8] = var1[var8];
      }
   }
}
