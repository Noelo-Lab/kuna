package pkg;

public class TestRecordComputedFields {
   record Checked(Object node, String reason) {
   }

   record Counted(int value) {
      static int created;
      static int last;

      Counted {
         if (value < 0) {
            throw new IllegalArgumentException();
         }

         created++;
      }
   }

   record Plain(int a, long b) {
   }

   record Trimmed(String name, int size) {
   }
}
