package pkg;

public class TestLocalClassAnonymousSubclass {
   public static Object detect(Object[] values) {
      Class<?> subclass = (new LocalClass<String>() {}).getClass();

      for (Object value : values) {
         class LocalClass<T> {
         }

         if (value == LocalClass.class) {
            return subclass;
         }
      }

      return null;
   }
}
