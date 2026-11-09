public interface Iface<T> extends Comparable<T>, java.util.function.Supplier<T> {
  int CONST = 7;
  String NAME = "iface";
  void abstractOne(T t);
  default int defaulted(T t) { return CONST + t.hashCode(); }
  static Iface<String> make() { return null; }
}
