package pkg;

import lib.Base;

public class TestNonPublicNestedImport extends Base {
   private Base.Helper last;

   public Object make(Object o) {
      Base.Helper h = new Base.Helper(o);
      this.last = h;
      return h.value;
   }

   public Object deep() {
      return new Base.Holder.Deep();
   }

   protected Base.Status accept(int i) {
      return i > 0 ? Base.Status.YES : Base.Status.NO;
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
            return o == null ? new Base.Marker() : o;
         }
      };
   }
}
