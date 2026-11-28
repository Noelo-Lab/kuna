package pkg;

public class TestTwrAssignedLocal {
   static String assigned(TestTwrAssignedLocal.Source source) {
      String line;
      try (TestTwrAssignedLocal.Source.Handle handle = source.open()) {
         line = handle.read();
      } catch (IllegalStateException e) {
         throw new RuntimeException(e);
      }

      return line;
   }

   static String direct(TestTwrAssignedLocal.Source source) {
      try (TestTwrAssignedLocal.Source.Handle handle = source.open()) {
         return handle.read();
      } catch (IllegalStateException e) {
         throw new RuntimeException(e);
      }
   }

   static int noCatch(TestTwrAssignedLocal.Source source) {
      int n;
      try (TestTwrAssignedLocal.Source.Handle handle = source.open()) {
         n = handle.read().length();
      }

      return n;
   }

   interface Source {
      TestTwrAssignedLocal.Source.Handle open();

      interface Handle extends AutoCloseable {
         String read();

         @Override
         void close();
      }
   }
}
