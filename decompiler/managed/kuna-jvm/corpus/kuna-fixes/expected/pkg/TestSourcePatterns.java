package pkg;

public class TestSourcePatterns {
   public int castStatement(TestSourcePatterns.Shape shape) {
      if (shape instanceof TestSourcePatterns.Square) {
         TestSourcePatterns.Square square = (TestSourcePatterns.Square)shape;
         return square.side * square.side;
      }

      return 0;
   }

   public int castChain(TestSourcePatterns.Shape shape) {
      int area = 0;
      if (shape instanceof TestSourcePatterns.Square) {
         TestSourcePatterns.Square square = (TestSourcePatterns.Square)shape;
         area = square.side * square.side;
      } else if (shape instanceof TestSourcePatterns.Circle) {
         TestSourcePatterns.Circle circle = (TestSourcePatterns.Circle)shape;
         area = 3 * circle.radius * circle.radius;
      }

      return area + shape.sides();
   }

   public int castReassigned(TestSourcePatterns.Shape shape) {
      if (shape instanceof TestSourcePatterns.Square) {
         TestSourcePatterns.Square square = (TestSourcePatterns.Square)shape;
         if (square.side == 0) {
            square = new TestSourcePatterns.Square();
         }

         return square.sides();
      }

      return -1;
   }

   public int pattern(TestSourcePatterns.Shape shape) {
      if (shape instanceof TestSourcePatterns.Square square) {
         return square.side;
      }

      return 0;
   }

   public int patternAndTest(TestSourcePatterns.Shape shape, boolean round) {
      if (round && shape instanceof TestSourcePatterns.Circle circle && circle.radius > 0) {
         return circle.radius;
      }

      return 0;
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
