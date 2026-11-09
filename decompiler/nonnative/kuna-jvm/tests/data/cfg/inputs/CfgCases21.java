// Inputs for the CFG oracle that need Java 21: switch expressions, pattern switches (whose
// MatchException handlers exercise the multiple-entry range handling), record patterns.

public class CfgCases21 {
  sealed interface Shape permits Circle, Square, Rect { }
  record Circle(double r) implements Shape { }
  record Square(double side) implements Shape { }
  record Rect(double w, double h) implements Shape { }
  record Pair(Object a, Object b) { }

  static int switchExpr(int x) {
    return switch (x) {
      case 1, 2 -> 10;
      case 3 -> {
        int y = x * 3;
        yield y + 1;
      }
      default -> -1;
    };
  }

  static String stringSwitchExpr(String s) {
    return switch (s) {
      case "a" -> "A";
      case "b", "c" -> "BC";
      default -> s.toUpperCase();
    };
  }

  static double area(Shape s) {
    return switch (s) {
      case Circle c -> Math.PI * c.r() * c.r();
      case Square q -> q.side() * q.side();
      case Rect(double w, double h) -> w * h;
    };
  }

  static String describe(Object o) {
    return switch (o) {
      case null -> "null";
      case Integer i when i > 10 -> "big " + i;
      case Integer i -> "int " + i;
      case String s -> "str " + s.length();
      case Pair(String a, Integer b) -> a + b;
      case Pair(Object a, Object b) -> "pair";
      default -> "other";
    };
  }

  static int instanceofPatterns(Object o) {
    if (o instanceof String s && !s.isEmpty()) {
      return s.length();
    }
    if (o instanceof Pair(Integer a, Integer b)) {
      return a + b;
    }
    return 0;
  }

  static int switchPatternInLoop(Object[] xs) {
    int n = 0;
    for (Object o : xs) {
      n += switch (o) {
        case Integer i -> {
          if (i < 0) {
            yield 0;
          }
          yield i;
        }
        case String s -> s.length();
        default -> 1;
      };
    }
    return n;
  }

  static int textBlockAndVar(String in) {
    var text = """
        line one
        line two
        """;
    var count = 0;
    for (var line : text.split("\n")) {
      if (line.contains(in)) {
        count++;
      }
    }
    return count;
  }
}
