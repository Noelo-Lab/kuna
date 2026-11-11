record B16Rec(int a, String b, long... rest) {
  static int s = 1;

  @Deprecated
  public int a() {
    return a;
  }
}
