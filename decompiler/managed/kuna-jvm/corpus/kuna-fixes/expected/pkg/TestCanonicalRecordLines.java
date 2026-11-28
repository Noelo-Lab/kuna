package pkg;

public class TestCanonicalRecordLines {
   public record Canonical(String name, int size) {
      public Canonical(String name, int size) {
         if (size < 0) {
            throw new IllegalArgumentException("size");
         }

         this.name = name;
         this.size = size;
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
