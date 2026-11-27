package pkg;

public class TestAnonymousVarReceiver {
   interface Shape {
      int area();
   }

   static int counter() {
      var counter = new Object() {
         int count;

         void bump() {
            count++;
         }
      };
      counter.bump();
      counter.bump();
      return counter.count;
   }

   static int square(int side) {
      var square = new Shape() {
         public int area() {
            return side * side;
         }

         int perimeter() {
            return 4 * side;
         }
      };
      return square.area() + square.perimeter();
   }

   static int inherited() {
      var shape = new Shape() {
         public int area() {
            return 2;
         }
      };
      return shape.area() + shape.hashCode();
   }
}
