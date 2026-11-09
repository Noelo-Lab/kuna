package kotlin;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

@Retention(RetentionPolicy.RUNTIME)
public @interface Metadata {
  String k() default "";
  String[] d1() default {};
  String[] d2() default {};
}
