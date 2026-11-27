package pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class TestAnnotatedInnerType<V> {
   private TestAnnotatedInnerType<V>.@TestAnnotatedInnerType.Nullable Task<?> task;

   void start() {
      this.task = new TestAnnotatedInnerType.Call();
   }

   Object finish() {
      TestAnnotatedInnerType<V>.Task<?> local = this.task;
      this.task = null;
      return local != null ? local.run() : null;
   }

   private final class Call extends TestAnnotatedInnerType<V>.Task<V> {
      @Override
      V run() {
         return null;
      }
   }

   @Target(ElementType.TYPE_USE)
   @Retention(RetentionPolicy.CLASS)
   public @interface Nullable {
   }

   private abstract class Task<T> {
      abstract T run();
   }
}
