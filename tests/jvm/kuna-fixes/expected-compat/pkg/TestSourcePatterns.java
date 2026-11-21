package pkg;

public class TestSourcePatterns {
   public int castStatement(TestSourcePatterns.Shape shape) {
      return shape instanceof TestSourcePatterns.Square square ? square.side * square.side : 0;
   }

   public int castChain(TestSourcePatterns.Shape shape) {
      int area = 0;
      if (shape instanceof TestSourcePatterns.Square square) {
         area = square.side * square.side;
      } else if (shape instanceof TestSourcePatterns.Circle circle) {
         area = 3 * circle.radius * circle.radius;
      }

      return area + shape.sides();
   }

   public int castReassigned(TestSourcePatterns.Shape shape) {
      if (shape instanceof TestSourcePatterns.Square square) {
         if (square.side == 0) {
            square = new TestSourcePatterns.Square();
         }

         return square.sides();
      } else {
         return -1;
      }
   }

   public int pattern(TestSourcePatterns.Shape shape) {
      return shape instanceof TestSourcePatterns.Square square ? square.side : 0;
   }

   public int patternAndTest(TestSourcePatterns.Shape shape, boolean round) {
      return round && shape instanceof TestSourcePatterns.Circle circle && circle.radius > 0 ? circle.radius : 0;
   }

   public static class Circle extends TestSourcePatterns.Shape {
      int radius;
   }

   public static class Shape {
      int sides() {
         return 0;
      }
   }

   public static class Square extends TestSourcePatterns.Shape {
      int side;
   }
}
