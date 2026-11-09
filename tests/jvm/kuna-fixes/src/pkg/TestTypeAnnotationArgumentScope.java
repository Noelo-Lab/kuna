package pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class TestTypeAnnotationArgumentScope {
   public Fn<Pair<String, @A Object>, @A Pair<String, @A Object>> finisher;

   public Fn<Pair<String, Object>, Pair<@A String, Object>> second(Fn<@A Pair<String, Object>, Pair<String, @A Object>> f) {
      return null;
   }

   @Retention(RetentionPolicy.RUNTIME)
   @Target(ElementType.TYPE_USE)
   @interface A {
   }

   interface Fn<T, R> {
   }

   interface Pair<K, V> {
   }
}
