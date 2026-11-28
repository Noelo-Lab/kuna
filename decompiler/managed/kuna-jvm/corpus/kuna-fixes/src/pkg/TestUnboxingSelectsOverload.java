// An unboxing call stays written where an inherited overload takes the box: javac -g --release 17
package pkg;

public class TestUnboxingSelectsOverload {
   abstract static class Schema {
      public final String validate(Object value) {
         return "object";
      }

      public final String validate(long value) {
         return "long";
      }

      public final String validate(double value) {
         return "double";
      }

      public final String validate(Double value) {
         return "Double";
      }

      public final String validate(Long value) {
         return "Long";
      }

      protected abstract String validateInternal(Object value);
   }

   static class NumberSchema extends Schema {
      protected String validateInternal(Object value) {
         return null;
      }

      protected String validateInternal(Double value) {
         if (value == null) {
            return null;
         }
         return validate(value.doubleValue());
      }

      protected String validateInternal(Long value) {
         return validate(value.longValue());
      }
   }
}
