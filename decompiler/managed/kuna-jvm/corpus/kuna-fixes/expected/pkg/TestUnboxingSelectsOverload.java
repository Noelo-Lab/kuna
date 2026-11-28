package pkg;

public class TestUnboxingSelectsOverload {
   static class NumberSchema extends TestUnboxingSelectsOverload.Schema {
      @Override
      protected String validateInternal(Object value) {
         return null;
      }

      protected String validateInternal(Double value) {
         if (value == null) {
            return null;
         }

         return this.validate(value.doubleValue());
      }

      protected String validateInternal(Long value) {
         return this.validate(value.longValue());
      }
   }

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

      protected abstract String validateInternal(Object var1);
   }
}
