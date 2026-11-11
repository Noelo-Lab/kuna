package kotlin;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

@Retention(RetentionPolicy.RUNTIME)
public @interface Metadata {
  int k() default 1;
  int d1() default 0;
  int[] d2() default {};
}
