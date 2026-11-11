package pkg;

public class TestVarMergeNarrowing {
   static final Object MARK = new Object();

   public int firstLength(TestVarMergeNarrowing.Source var1) {
      String var2 = (String)var1.next();
      if (var2 == MARK) {
         return -1;
      }

      var2 = (String)var1.next();
      return var2.length();
   }

   public int nextLength(TestVarMergeNarrowing.Source var1) {
      Object var2 = var1.next();
      if (var2 == MARK) {
         return -1;
      }

      String var3 = (String)var2;
      return var3.length();
   }

   public interface Source {
      Object next();
   }
}
