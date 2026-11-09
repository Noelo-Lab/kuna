package pkg;

import lib.Base;
import lib.Base.Helper;
import lib.Base.Marker;
import lib.Base.Status;
import lib.Base.Holder.Deep;

public class TestNonPublicNestedImport extends Base {
   private Helper last;

   public Object make(Object o) {
      Helper h = new Helper(o);
      this.last = h;
      return h.value;
   }

   public Object deep() {
      return new Deep();
   }

   protected Status accept(int i) {
      return i > 0 ? Status.YES : Status.NO;
   }

   public int code(int i) {
      switch (this.accept(i)) {
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
         public Object make(Object o) {
            return o == null ? new Marker() : o;
         }
      };
   }
}
