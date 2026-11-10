package pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class TestAnnotatedInnerType<V> {
   @Target(ElementType.TYPE_USE)
   @Retention(RetentionPolicy.CLASS)
   public @interface Nullable {
   }

   private @Nullable Task<?> task;

   private abstract class Task<T> {
      abstract T run();
   }

   private final class Call extends Task<V> {
      V run() {
         return null;
      }
   }

   void start() {
      task = new Call();
   }

   Object finish() {
      Task<?> local = task;
      task = null;
      return local != null ? local.run() : null;
   }
}
