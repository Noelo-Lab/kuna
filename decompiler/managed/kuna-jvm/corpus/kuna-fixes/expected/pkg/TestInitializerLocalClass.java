package pkg;

public class TestInitializerLocalClass {
   static int w = 1;
   static int z;
   int count;

   TestInitializerLocalClass() {
      class Counter {
         int next() {
            return TestInitializerLocalClass.this.count + TestInitializerLocalClass.z;
         }
      }

      this.count = new Counter().next();
   }

   TestInitializerLocalClass(int extra) {
      class Counter {
         int next() {
            return TestInitializerLocalClass.this.count + TestInitializerLocalClass.z;
         }
      }

      this.count = new Counter().next();
      this.count += extra;
   }

   static {
      class Local {
         int get() {
            return TestInitializerLocalClass.w + 1;
         }
      }

      w = new Local().get();
      z = 2;
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
