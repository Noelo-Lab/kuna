package pkg;

import ext.TestOverrideInaccessibleBase;

public class TestOverrideInaccessible extends TestOverrideInaccessibleBase {
   @Override
   Object parser() {
      return this;
   }

   @Override
   public int secret() {
      return 2;
   }

   @Override
   protected int size() {
      return 3;
   }

   @Override
   public String name() {
      return "sub";
   }

   static class Inner {
      private int hidden() {
         return 4;
      }
   }

   static class InnerSub extends TestOverrideInaccessible.Inner {
      @Override
      int hidden() {
         return 5;
      }
   }
}
