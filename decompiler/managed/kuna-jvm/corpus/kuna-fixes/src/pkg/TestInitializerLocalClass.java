package pkg;

public class TestInitializerLocalClass {
   static int w = 1;

   static {
      class Local {
         int get() {
            return w + 1;
         }
      }

      w = new Local().get();
   }

   static int z = 2;
   int count;

   {
      class Counter {
         int next() {
            return TestInitializerLocalClass.this.count + z;
         }
      }

      this.count = new Counter().next();
   }

   TestInitializerLocalClass() {
   }

   TestInitializerLocalClass(int extra) {
      this.count += extra;
   }

   static class Nested {
      static final Object MADE;

      static {
         class Maker {
            Object make() {
               return new Object();
            }
         }

         Maker m = new Maker();
         MADE = m.make();
      }
   }
}
