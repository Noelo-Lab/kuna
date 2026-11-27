import java.lang.annotation.*;
import java.util.*;

public record Records<T extends Comparable<T>>(@Records.Mark String name, int age, T value, List<String>... tags) implements Comparable<Records<T>> {
  @Retention(RetentionPolicy.RUNTIME)
  @Target({ElementType.FIELD, ElementType.RECORD_COMPONENT, ElementType.METHOD, ElementType.PARAMETER})
  @interface Mark {}

  public Records {
    if (age < 0) throw new IllegalArgumentException();
  }

  public int compareTo(Records<T> o) { return value.compareTo(o.value); }

  record Empty() {}

  static String describe(Object o) {
    if (o instanceof Records<?> r && r.age() > 3) return r.name();
    return switch (o) {
      case String s when s.length() > 2 -> "long";
      case String s -> "short";
      case Integer i -> "int" + i;
      default -> "other";
    };
  }
}
