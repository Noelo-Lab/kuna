package pkg;

public class TestCapturedConstantType {
   interface Source {
      Object get();
   }

   static Source anonymous() {
      int width = 4;
      return new Source() {
         @Override
         public Object get() {
            return width;
         }
      };
   }

   static Source lambda() {
      int count = 7;
      return () -> count;
   }

   static Object local() {
      int size = 9;

      class Holder {
         Object value() {
            return size;
         }
      }

      return new Holder().value();
   }

   static Source narrow() {
      short small = 5;
      return () -> small;
   }
}
