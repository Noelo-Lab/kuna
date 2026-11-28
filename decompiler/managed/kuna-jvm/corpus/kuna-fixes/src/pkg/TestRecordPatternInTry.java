package pkg;

public class TestRecordPatternInTry {
  sealed interface Shape permits Circle, Label {}

  record Circle(int radius) implements Shape {}

  record Label(String text) implements Shape {}

  static class Missing extends Exception {}

  Shape load(String key) throws Missing {
    return null;
  }

  public Object describe(String key) {
    try {
      Shape s = this.load(key);
      if (s instanceof Circle(int r)) {
        return r * 2;
      } else {
        return s instanceof Label(String t) ? t : null;
      }
    } catch (Missing e) {
      return e;
    }
  }

  public int measure(String key) {
    int size = 0;

    try {
      Shape s = this.load(key);
      if (s instanceof Circle(int r)) {
        size = r;
      } else if (s instanceof Label(String t)) {
        size = t.length();
      }
    } catch (Missing e) {
      size = -1;
    }

    return size;
  }
}
