package pkg;

public class TestAssertThrowAfterIf {
   // $VF: Could not resugar all assert statements!
   public static void loopEnd(TestAssertThrowAfterIf.Ctx c) {
      while (c.more()) {
         if (!c.exec()) {
            return;
         }

         if (!$assertionsDisabled && !c.initial()) {
            throw new AssertionError();
         }
      }
   }

   // $VF: Could not resugar all assert statements!
   public static void beforeReturn(TestAssertThrowAfterIf.Ctx c, Object current) {
      while (c.more()) {
         Object request = c.next();
         if (request == null) {
            if ($assertionsDisabled || current == null && !c.more()) {
               if (c.initial()) {
                  c.done();
               }

               return;
            }

            throw new AssertionError("sources left");
         }

         c.exec();
      }
   }

   interface Ctx {
      boolean more();

      boolean exec();

      boolean initial();

      Object next();

      void done();
   }
}
