package pkg;

public class TestAnonymousVarReceiver {
   static int counter() {
      var var0 = new Object() {
         int count;

         void bump() {
            this.count++;
         }
      };
      var0.bump();
      var0.bump();
      return var0.count;
   }

   static int square(final int var0) {
      var var1 = new TestAnonymousVarReceiver.Shape() {
         @Override
         public int area() {
            return var0 * var0;
         }

         int perimeter() {
            return 4 * var0;
         }
      };
      return var1.area() + var1.perimeter();
   }

   interface Shape {
      int area();
   }
}
