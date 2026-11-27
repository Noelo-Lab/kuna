package pkg;

import java.util.HashMap;
import java.util.Map;

public class TestStaticInitExit {
   static final Map<String, Integer> RANK = new HashMap<>();
   static int last;
   static int found;

   static {
      Object[] groups = new Object[]{null, "a", "b", null, "c"};
      int rank = 0;
      int i = 0;

      outer:
      while (true) {
         i++;
         rank++;

         while (true) {
            if (i == groups.length) {
               break outer;
            }

            if (!(groups[i] instanceof String)) {
               break;
            }

            RANK.put((String)groups[i++], rank);
         }
      }

      for (int j = 0; j < 3; j++) {
         if (RANK.containsKey("x" + j)) {
            last = j;
            break;
         }
      }
   }

   static class Searched {
      static final int[] VALUES = new int[]{3, 5, 8};
      static int index = -1;

      static {
         search:
         for (int k = 0; k < VALUES.length; k++) {
            for (int v = 0; v < 10; v++) {
               if (VALUES[k] == v * v) {
                  index = k;
                  break search;
               }
            }
            found++;
         }
      }
   }
}
