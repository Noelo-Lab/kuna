package pkg;

public class TestQualifiedSuperEmpty {
   public class Implicit extends TestQualifiedSuperEmpty.Inner {
   }

   public class Inner {
   }

   static class Nested extends TestQualifiedSuperEmpty.Inner {
      Nested(TestQualifiedSuperEmpty o) {
         o.super();
      }
   }

   public class Other extends TestQualifiedSuperEmpty.Inner {
      public Other(TestQualifiedSuperEmpty o) {
         o.super();
      }
   }

   static class Unseen extends TestQualifiedSuperEmptyBase {
      class UnseenInner extends TestQualifiedSuperEmpty.Inner {
         final int x;

         UnseenInner(int x) {
            this.x = x;
         }
      }
   }
}
