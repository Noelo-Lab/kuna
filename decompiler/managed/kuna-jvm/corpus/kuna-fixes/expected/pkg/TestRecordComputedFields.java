package pkg;

import java.util.Objects;

public class TestRecordComputedFields {
   record Checked(Object node, String reason) {
      Checked(Object node, String reason) {
         this.node = Objects.requireNonNull(node);
         this.reason = reason;
      }
   }

   record Counted(int value) {
      static int created;
      static int last;

      Counted {
         if (value < 0) {
            throw new IllegalArgumentException();
         }

         created++;
         last = value;
      }
   }

   record Plain(int a, long b) {
   }

   record Trimmed(String name, int size) {
      Trimmed(String name, int size) {
         this.name = name.trim();
         this.size = size;
      }
   }
}
