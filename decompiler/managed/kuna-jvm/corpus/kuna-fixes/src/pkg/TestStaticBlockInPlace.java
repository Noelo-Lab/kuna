package pkg;

import java.util.ArrayList;
import java.util.List;

public class TestStaticBlockInPlace {
   static final List<String> LOG = new ArrayList<>();

   static {
      LOG.add("registered");
   }

   public static final TestStaticBlockInPlace INSTANCE = new TestStaticBlockInPlace();
   static String label;

   static {
      int size = LOG.size();
      label = "n" + size;
      total = size;
   }

   static final int LENGTH = label.length();

   static {
      TestStaticBlockInPlace.total += LENGTH;
      Runnable report = () -> LOG.add("total " + TestStaticBlockInPlace.total);
      report.run();
   }

   static final String LAST = LOG.get(LOG.size() - 1);
   static int total;

   enum Kind {
      A,
      B;

      static {
         LOG.add("kinds");
      }

      static final int COUNT = values().length;
   }
}
