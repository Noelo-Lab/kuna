package pkg;

import lib.Base;

public class TestNonPublicNestedImport extends Base {
   private Helper last;

   @Override
   public Object make(Object o) {
      Helper h = new Helper(o);
      this.last = h;
      return h.value;
   }

   public Object deep() {
      return new Holder.Deep();
   }

   protected Status accept(int i) {
      return i > 0 ? Status.YES : Status.NO;
   }

   public int code(int i) {
      switch (accept(i)) {
         case YES:
            return 1;
         case NO:
            return 2;
         default:
            return 3;
      }
   }

   public Base anonymous() {
      return new Base() {
         @Override
         public Object make(Object o) {
            return o == null ? new Marker() : o;
         }
      };
   }
}
