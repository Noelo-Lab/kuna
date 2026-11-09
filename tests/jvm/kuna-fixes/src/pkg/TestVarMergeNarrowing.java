package pkg;

public class TestVarMergeNarrowing {
   static final Object MARK = new Object();

   public interface Source {
      Object next();
   }

   public int firstLength(Source src) {
      {
         Object o = src.next();
         if (o == MARK) {
            return -1;
         }
      }
      {
         String s = (String)src.next();
         return s.length();
      }
   }

   public int nextLength(Source src) {
      Object o = src.next();
      if (o == MARK) {
         return -1;
      }
      String s = (String)o;
      return s.length();
   }
}
