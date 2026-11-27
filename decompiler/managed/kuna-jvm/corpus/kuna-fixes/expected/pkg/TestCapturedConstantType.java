package pkg;

public class TestCapturedConstantType {
   static TestCapturedConstantType.Source anonymous() {
      int var0 = 4;
      return new TestCapturedConstantType.Source() {
         @Override
         public Object get() {
            return var0;
         }
      };
   }

   static TestCapturedConstantType.Source lambda() {
      int var0 = 7;
      return () -> var0;
   }

   static Object local() {
      int var0 = 9;

      class Holder {
         Object value() {
            return var0;
         }
      }

      return new Holder().value();
   }

   static TestCapturedConstantType.Source narrow() {
      short var0 = 5;
      return () -> var0;
   }

   interface Source {
      Object get();
   }
}
