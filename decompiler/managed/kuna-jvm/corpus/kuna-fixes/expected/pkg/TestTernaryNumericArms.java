package pkg;

public class TestTernaryNumericArms {
   public Number intOrLong(boolean c) {
      return (Number)(c ? (Object)1 : 2L);
   }

   public Object intOrLongObject(boolean c) {
      return c ? (Object)1 : 2L;
   }

   public Integer boxedOrNull(boolean c, Integer a) {
      return c ? (Integer)1 : a;
   }

   public Boolean flagOrNull(boolean c, Boolean b) {
      return c ? (Boolean)true : b;
   }

   public Object doubleOrBoxed(boolean c, Double d) {
      return c ? d : (Double)0.5;
   }

   public long numeric(boolean c, int i, long l) {
      return c ? i : l;
   }
}
