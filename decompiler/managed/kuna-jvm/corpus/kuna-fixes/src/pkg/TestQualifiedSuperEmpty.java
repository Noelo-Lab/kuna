package pkg;

public class TestQualifiedSuperEmpty {
   public class Inner {
   }

   public class Implicit extends Inner {
      public Implicit() {
      }
   }

   static class Nested extends TestQualifiedSuperEmpty.Inner {
      Nested(TestQualifiedSuperEmpty o) {
         o.super();
      }
   }

   public class Other extends Inner {
      public Other(TestQualifiedSuperEmpty o) {
         o.super();
      }
   }

   static class Unseen extends TestQualifiedSuperEmptyBase {
      class UnseenInner extends Inner {
         final int x;

         UnseenInner(int x) {
            this.x = x;
         }
      }
   }
}

class TestQualifiedSuperEmptyBase extends TestQualifiedSuperEmpty {
}
