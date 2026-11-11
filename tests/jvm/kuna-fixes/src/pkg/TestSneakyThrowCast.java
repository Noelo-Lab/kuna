package pkg;

public class TestSneakyThrowCast<T extends Throwable> {
   static Error sneakyThrow(Throwable t) {
      throw new TestSneakyThrowCast<Error>().throwIt(t);
   }

   @SuppressWarnings("unchecked")
   private Error throwIt(Throwable t) throws T {
      throw (T) t;
   }

   @SuppressWarnings("unchecked")
   static <E extends Exception> void rethrow(Exception e) throws E {
      throw (E) e;
   }
}
