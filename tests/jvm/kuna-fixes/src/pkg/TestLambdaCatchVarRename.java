package pkg;

public class TestLambdaCatchVarRename {
   interface Task {
      void run() throws Exception;
   }

   interface Each {
      void each(Task t);
   }

   static void log(Exception e) {
   }

   static void log(String s) {
   }

   static String describe(Task[] tasks) {
      return "tasks";
   }

   static void forEach(Task[] tasks, Each each) {
      for (Task t : tasks) {
         each.each(t);
      }
   }

   public static void test(Task[] tasks) {
      forEach(tasks, t -> {
         try {
            t.run();
         } catch (Exception e) {
            log(e);
         }
      });
      String e = describe(tasks);
      log(e);
   }

   public static int count(Task[] tasks) {
      int[] n = new int[1];
      forEach(tasks, t -> {
         try {
            t.run();
            n[0]++;
         } catch (RuntimeException ex) {
            log(ex);
         } catch (Exception ex) {
            throw new IllegalStateException(ex);
         }
      });
      for (Task ex : tasks) {
         log(describe(new Task[]{ex}));
      }
      return n[0];
   }
}
