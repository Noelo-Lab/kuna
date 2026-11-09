package pkg;

public class TestLocalClassAnonymousSubclass {
   public static Object detect(Object[] values) {
      class LocalClass<T> {
      }

      Class<?> subclass = new LocalClass<String>() {}.getClass();
      for (Object value : values) {
         if (value == LocalClass.class) {
            return subclass;
         }
      }
      return null;
   }
}
