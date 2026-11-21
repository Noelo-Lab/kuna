package pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class TestTypeAnnotationArgumentScope {
   public TestTypeAnnotationArgumentScope.Fn<TestTypeAnnotationArgumentScope.Pair<String, @TestTypeAnnotationArgumentScope.A @TestTypeAnnotationArgumentScope.A Object>, TestTypeAnnotationArgumentScope.@TestTypeAnnotationArgumentScope.A Pair<String, Object>> finisher;

   public TestTypeAnnotationArgumentScope.Fn<TestTypeAnnotationArgumentScope.Pair<String, Object>, TestTypeAnnotationArgumentScope.Pair<@TestTypeAnnotationArgumentScope.A String, Object>> second(
      TestTypeAnnotationArgumentScope.Fn<TestTypeAnnotationArgumentScope.@TestTypeAnnotationArgumentScope.A Pair<String, Object>, TestTypeAnnotationArgumentScope.@TestTypeAnnotationArgumentScope.A Pair<String, Object>> f
   ) {
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
