package pkg;

// TestListedInnerMemberLib stands for a library class that is not given to the decompiler: its
// class files are not part of the case.
public class TestListedInnerMember extends TestListedInnerMemberLib {
   Object inherited() {
      return new Entry(1);
   }

   static Object qualified(TestListedInnerMemberLib lib) {
      return lib.new Entry(2);
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

class TestListedInnerMemberLib {
   class Entry {
      Entry(int value) {
      }
   }
}
