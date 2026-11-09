public enum Enums {
  RED("r") { int code() { return 1; } },
  GREEN("g") { int code() { return 2; } },
  BLUE("b") { int code() { return 3; } };

  private final String tag;
  Enums(String tag) { this.tag = tag; }
  abstract int code();

  static String name(Enums e) {
    switch (e) {
      case RED: return "red";
      case GREEN: return "green";
      default: return e.tag;
    }
  }
}
