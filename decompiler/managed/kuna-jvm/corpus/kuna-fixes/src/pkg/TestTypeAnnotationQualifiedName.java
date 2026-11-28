package pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import lib.AnnotatedHolder;

public class TestTypeAnnotationQualifiedName {
   private AnnotatedHolder.@A Entry<String> annotated;
   private AnnotatedHolder.Entry<String> plain;

   public AnnotatedHolder.@A Entry<String> pick(AnnotatedHolder.@A Entry<String> other) {
      return other != null ? other : this.annotated;
   }

   public AnnotatedHolder.Entry<String> plain() {
      return this.plain;
   }

   public Entry own() {
      return new Entry();
   }

   @Retention(RetentionPolicy.RUNTIME)
   @Target(ElementType.TYPE_USE)
   @interface A {
   }

   static class Entry {
   }
}
