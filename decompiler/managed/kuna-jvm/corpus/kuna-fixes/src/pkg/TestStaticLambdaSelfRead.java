package pkg;

public class TestStaticLambdaSelfRead {
   static final Runnable SELF = () -> {
      if (TestStaticLambdaSelfRead.SELF == null) {
         System.out.println("early");
      }
   };
   static final Runnable NEXT = () -> TestStaticLambdaSelfRead.LAST.run();
   static final Runnable LAST = () -> SELF.run();
}
