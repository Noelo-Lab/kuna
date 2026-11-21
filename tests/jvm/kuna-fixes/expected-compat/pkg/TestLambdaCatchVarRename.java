package pkg;

public class TestLambdaCatchVarRename {
   static void log(Exception e) {
   }

   static void log(String s) {
   }

   static String describe(TestLambdaCatchVarRename.Task[] tasks) {
      return "tasks";
   }

   static void forEach(TestLambdaCatchVarRename.Task[] tasks, TestLambdaCatchVarRename.Each each) {
      for (TestLambdaCatchVarRename.Task t : tasks) {
         each.each(t);
      }
   }

   public static void test(TestLambdaCatchVarRename.Task[] tasks) {
      forEach(tasks, t -> {
         try {
            t.run();
         } catch (Exception e) {
            log(ex);
         }
      });
      String e = describe(tasks);
      log(e);
   }

   public static int count(TestLambdaCatchVarRename.Task[] tasks) {
      int[] n = new int[1];
      forEach(tasks, t -> {
         try {
            t.run();
            n[0]++;
         } catch (RuntimeException ex) {
            log(exxx);
         } catch (Exception ex) {
            throw new IllegalStateException(exx);
         }
      });

      for (TestLambdaCatchVarRename.Task ex : tasks) {
         log(describe(new TestLambdaCatchVarRename.Task[]{ex}));
      }

      return n[0];
   }

   interface Each {
      void each(TestLambdaCatchVarRename.Task var1);
   }

   interface Task {
      void run() throws Exception;
   }
}
