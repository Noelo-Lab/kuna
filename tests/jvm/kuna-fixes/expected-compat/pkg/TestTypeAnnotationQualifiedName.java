package pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class TestTypeAnnotationQualifiedName {
   private @TestTypeAnnotationQualifiedName.A Entry<String> annotated;
   private lib.AnnotatedHolder.Entry<String> plain;

   public @TestTypeAnnotationQualifiedName.A Entry<String> pick(@TestTypeAnnotationQualifiedName.A Entry<String> other) {
      return other != null ? other : this.annotated;
   }

   public lib.AnnotatedHolder.Entry<String> plain() {
      return this.plain;
   }

   public TestTypeAnnotationQualifiedName.Entry own() {
      return new TestTypeAnnotationQualifiedName.Entry();
   }

   @Retention(RetentionPolicy.RUNTIME)
   @Target(ElementType.TYPE_USE)
   @interface A {
   }

   static class Entry {
   }
}
