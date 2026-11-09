package k7;

import java.lang.annotation.*;

@Retention(RetentionPolicy.CLASS)
@Target(ElementType.TYPE_USE)
public @interface TU2 {
  int n() default 1;
}
