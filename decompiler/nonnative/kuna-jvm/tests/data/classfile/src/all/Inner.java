public class Inner {
  private int secret = 1;
  private static int sSecret() { return 2; }

  class Member { int peek() { return secret + sSecret(); } }
  static class Nested { static class Deeper { } }
  interface Callback { void call(); }

  Object local() {
    class Local implements Callback { public void call() { secret++; } }
    return new Local();
  }

  Callback anon(final int k) {
    return new Callback() {
      int count = k;
      public void call() { count += secret; }
    };
  }

  static Object staticAnon() {
    return new Object() { @Override public String toString() { return "anon"; } };
  }

  Object nestedAnon() {
    return new Callback() {
      public void call() {
        Object o = new Object() { };
      }
    };
  }
}
