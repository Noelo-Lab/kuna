package pkg;

public class TestSwitchCaseOrder {
   static int sink;

   static String style(char c) {
      return switch (c) {
         case 'M' -> "medium";
         case 'L' -> "long";
         case 'F' -> "full";
         case '-' -> null;
         default -> throw new IllegalArgumentException("bad " + c);
      };
   }

   static int query(TestSwitchCaseOrder.Opt o, long v) {
      return switch (o) {
         case DOC_VALUES -> (int)v;
         case POINTS -> (int)(v + 1L);
      };
   }

   static void arrows(int x) {
      switch (x) {
         case 7:
            sink = 7;
            break;
         case 1:
            sink = 1;
            break;
         case 4:
            sink = 4;
            break;
         default:
            throw new IllegalStateException();
      }

      sink++;
   }

   static int fall(int x) {
      int r = 0;
      switch (x) {
         case 9:
            r = 1;
         case 2:
            r += 2;
            break;
         case 5:
            r = 7;
      }

      return r;
   }

   static int middle(int x) {
      switch (x) {
         case 30:
            return 3;
         default:
            return 0;
         case 10:
            return 1;
      }
   }

   enum Opt {
      POINTS,
      DOC_VALUES;
   }
}
