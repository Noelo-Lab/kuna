package pkg;

public class TestSwitchCaseOrder {
   static int sink;

   static String style(char c) {
      return switch (c) {
         case '-' -> null;
         case 'F' -> "full";
         case 'L' -> "long";
         case 'M' -> "medium";
         default -> throw new IllegalArgumentException("bad " + c);
      };
   }

   static int query(TestSwitchCaseOrder.Opt o, long v) {
      return switch (o) {
         case POINTS -> (int)(v + 1L);
         case DOC_VALUES -> (int)v;
      };
   }

   static void arrows(int x) {
      switch (x) {
         case 1:
            sink = 1;
            break;
         case 4:
            sink = 4;
            break;
         case 7:
            sink = 7;
            break;
         default:
            throw new IllegalStateException();
      }

      sink++;
   }

   static int fall(int x) {
      int r = 0;
      switch (x) {
         case 5:
            r = 7;
            break;
         case 9:
            r = 1;
         case 2:
            r += 2;
      }

      return r;
   }

   static int middle(int x) {
      switch (x) {
         case 10:
            return 1;
         case 30:
            return 3;
         default:
            return 0;
      }
   }

   enum Opt {
      POINTS,
      DOC_VALUES;
   }
}
