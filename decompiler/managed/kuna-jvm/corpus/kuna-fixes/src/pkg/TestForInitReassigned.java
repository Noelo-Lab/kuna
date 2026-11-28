package pkg;

public class TestForInitReassigned {
   Object[] last;

   void copy(Object[] src, long hits, long size) {
      long var4 = Math.min(hits, size);
      var4 = Math.min((long)src.length, var4); for (int var6 = 0; (long)var6 < var4; var6++) {
         this.last[var6] = src[var6];
      }
   }
}
