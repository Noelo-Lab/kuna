package pkg;

public class TestCanonicalRecordLines {
   public record Canonical(String name, int size) {
      public Canonical {
         if (size < 0) {
            throw new IllegalArgumentException("size");
         }
      }
   }

   public record Compact(String name, int size) {
      public Compact {
         if (size < 0) {
            throw new IllegalArgumentException("size");
         }
      }
   }
}
