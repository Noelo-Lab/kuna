public sealed interface Sealed permits Sealed.A, Sealed.B, Sealed.C {
  final class A implements Sealed {}
  non-sealed class B implements Sealed {}
  record C(int x) implements Sealed {}

  static int which(Sealed s) {
    return switch (s) {
      case A a -> 1;
      case B b -> 2;
      case C c -> c.x();
    };
  }

  static String text(int day) {
    String r = switch (day) {
      case 1, 7 -> "weekend";
      case 2, 3, 4, 5, 6 -> {
        String t = "week";
        yield t + "day";
      }
      default -> throw new IllegalArgumentException("bad " + day);
    };
    return """
      text block %s
      """.formatted(r);
  }
}
