package pkg;

public class TestAssertThrowAfterIf {
   interface Ctx {
      boolean more();

      boolean exec();

      boolean initial();

      Object next();

      void done();
   }

   public static void loopEnd(Ctx c) {
      while (c.more()) {
         if (!c.exec()) {
            return;
         }
         assert c.initial();
      }
   }

   public static void beforeReturn(Ctx c, Object current) {
      while (c.more()) {
         Object request = c.next();
         if (request == null) {
            assert current == null && !c.more() : "sources left";
            if (c.initial()) {
               c.done();
            }
            return;
         }
         c.exec();
      }
   }
}
