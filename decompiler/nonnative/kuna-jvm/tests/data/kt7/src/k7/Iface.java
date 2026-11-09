package k7;

public interface Iface {
  int CONST = 3;
  String NAME = "iface";

  void ifaceM();

  default void def() {
  }

  static void ifStatic() {
  }
}
