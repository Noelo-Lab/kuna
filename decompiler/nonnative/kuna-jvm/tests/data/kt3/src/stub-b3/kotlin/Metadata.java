package kotlin;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

@Retention(RetentionPolicy.RUNTIME)
public @interface Metadata {
  int k() default 1;
  String[] d1() default {};
  int[] d2() default {};
  int xs() default 0;
}
