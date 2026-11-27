package pkg;

public class TestSelfReferencingLambda {
   TestSelfReferencingLambda.Task task = this::body;

   public void plain() {
      this.task = () -> this.task = this::body;
   }

   interface Task {
      void run();
   }
}
