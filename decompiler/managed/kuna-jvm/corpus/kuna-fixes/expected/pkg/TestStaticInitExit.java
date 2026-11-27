package pkg;

import java.util.HashMap;
import java.util.Map;

public class TestStaticInitExit {
   static final Map<String, Integer> RANK = new HashMap();
   static int last;
   static int found;

   static {
      Object[] groups = new Object[]{null, "a", "b", null, "c"};
      int rank = 0;
      int i = 0;

      label32:
      while (true) {
         i++;
         rank++;

         while (i != groups.length) {
            if (!(groups[i] instanceof String)) {
               continue label32;
            }

            RANK.put((String)groups[i++], rank);
         }

         for (int j = 0; j < 3; j++) {
            if (RANK.containsKey("x" + j)) {
               last = j;
               break;
            }
         }

         break;
      }
   }

   static class Searched {
      static final int[] VALUES = new int[]{3, 5, 8};
      static int index = -1;

      static {
         label22:
         for (int k = 0; k < VALUES.length; k++) {
            for (int v = 0; v < 10; v++) {
               if (VALUES[k] == v * v) {
                  index = k;
                  break label22;
               }
            }

            TestStaticInitExit.found++;
         }
      }
   }
}
