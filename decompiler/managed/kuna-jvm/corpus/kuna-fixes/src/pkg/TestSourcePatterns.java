package pkg;

public class TestSourcePatterns {
   public static class Shape {
      int sides() {
         return 0;
      }
   }

   public static class Square extends Shape {
      int side;
   }

   public static class Circle extends Shape {
      int radius;
   }

   public int castStatement(Shape shape) {
      if (shape instanceof Square) {
         Square square = (Square) shape;
         return square.side * square.side;
      }
      return 0;
   }

   public int castChain(Shape shape) {
      int area = 0;
      if (shape instanceof Square) {
         Square square = (Square) shape;
         area = square.side * square.side;
      } else if (shape instanceof Circle) {
         Circle circle = (Circle) shape;
         area = 3 * circle.radius * circle.radius;
      }
      return area + shape.sides();
   }

   public int castReassigned(Shape shape) {
      if (shape instanceof Square) {
         Square square = (Square) shape;
         if (square.side == 0) {
            square = new Square();
         }
         return square.sides();
      }
      return -1;
   }

   public int pattern(Shape shape) {
      if (shape instanceof Square square) {
         return square.side;
      }
      return 0;
   }

   public int patternAndTest(Shape shape, boolean round) {
      if (round
            && shape instanceof Circle circle && circle.radius > 0) {
         return circle.radius;
      }
      return 0;
   }
}
