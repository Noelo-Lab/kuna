package pkg;

import pkg.TestListedInnerMemberLib.Entry;

public class TestListedInnerMember extends TestListedInnerMemberLib {
   Object inherited() {
      return new Entry(1);
   }

   static Object qualified(TestListedInnerMemberLib var0) {
      return var0.new Entry(2);
   }

   class Values {
      Object outer() {
         return TestListedInnerMember.this.new Entry(3);
      }

      Object implicit() {
         return new Entry(4);
      }
   }
}
