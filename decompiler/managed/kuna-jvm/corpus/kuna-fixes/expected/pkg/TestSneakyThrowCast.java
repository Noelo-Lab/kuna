package pkg;

public class TestSneakyThrowCast<T extends Throwable> {
   static Error sneakyThrow(Throwable t) {
      throw new TestSneakyThrowCast<RuntimeException>().throwIt(t);
   }

   private Error throwIt(Throwable t) throws T {
      throw (T)t;
   }

   static <E extends Exception> void rethrow(Exception e) throws E {
      throw (E)e;
   }
}
