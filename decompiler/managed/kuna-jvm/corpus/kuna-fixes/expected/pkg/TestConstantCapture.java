package pkg;

import java.util.function.Supplier;

public class TestConstantCapture {
   static TestConstantCapture.Text anonymous() {
      String prefix = "p:";
      int width = 4;
      return new TestConstantCapture.Text() {
         @Override
         public String text() {
            return prefix + width;
         }
      };
   }

   static String local(String name) {
      String tag = "#";
      final String trimmed = name.trim();

      class Label {
         String render() {
            return tag + trimmed;
         }
      }

      return new Label().render();
   }

   static Supplier<String> lambda() {
      String fixed = "x";
      return () -> fixed;
   }

   interface Text {
      String text();
   }
}
