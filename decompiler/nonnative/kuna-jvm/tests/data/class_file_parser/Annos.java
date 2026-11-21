import java.lang.annotation.*;
import java.util.*;

@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD, ElementType.PARAMETER})
@interface Everything {
  byte b() default 1;
  char c() default 'c';
  short s() default 2;
  int i() default 3;
  long l() default 4L;
  float f() default 5.5f;
  double d() default 6.5;
  boolean z() default true;
  String str() default "x";
  Class<?> cls() default String.class;
  Class<?> prim() default int.class;
  Class<?> vd() default void.class;
  Class<?> arr() default String[].class;
  ElementType en() default ElementType.FIELD;
  Retention ann() default @Retention(RetentionPolicy.CLASS);
  int[] ints() default {1, 2, 3};
  String[] none() default {};
}

@Retention(RetentionPolicy.CLASS)
@interface Invisible { String value(); }

@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE_USE, ElementType.TYPE_PARAMETER})
@interface TU { int value() default 0; }

@Retention(RetentionPolicy.CLASS)
@Target(ElementType.TYPE_USE)
@interface TUI {}

@Everything(i = 42, str = "y", ints = {}, en = ElementType.TYPE)
@Invisible("cls")
public class Annos<@TU(1) T extends @TU(2) Comparable<T>> extends @TU(3) Object implements @TU(4) Runnable {
  @Everything @Deprecated public @TU(5) String field;
  @TU(6) List<@TU(7) String @TU(8) []> nested;
  Map<? extends @TU(9) Number, @TUI ? super @TU Integer> wild;
  java.util.Map.@TU(10) Entry<String, String> qualified;

  @Invisible("m") @Everything(cls = Annos.class)
  public @TU(11) int method(@Everything @Invisible("p0") int a, @TU(12) String b, final @Deprecated Object c) throws @TU(13) RuntimeException {
    @TU(14) Object local = b;
    if (local instanceof @TU(15) String) a++;
    Object n = new @TU(16) ArrayList<@TU(17) String>();
    String cast = (@TU(18) String) local;
    try { a++; } catch (@TU(19) IllegalStateException e) { a--; }
    java.util.function.Supplier<List<String>> sup = @TU(20) ArrayList::new;
    return a + cast.length() + n.hashCode() + sup.hashCode();
  }

  public <@TU(21) M extends @TU(22) Number> void generic(M m) {}

  public void run() {}

  @Deprecated
  static class Old {}
}
