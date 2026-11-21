package pkg;

public class Hello {
  public static void main(String[] args) {
    Runnable r = new Runnable() {
      public void run() {}
    };
    r.run();
  }
}
