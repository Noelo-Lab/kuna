package pkg

public class TestCatchBlockJump {
   private final var failures: Int

   public fun runAll(tasks: Array<pkg.TestCatchBlockJump.Task>): Int {
      var done: Int = 0

      for (t in tasks) {
         var var7: Int
         try {
            var7 = t.run()
         } catch (var10: TestCatchBlockJump.Failure) {
            val var9: Int = this.failures++
            continue
         }

         done += var7
      }

      return done
   }

   public fun firstWorking(tasks: Array<pkg.TestCatchBlockJump.Task>): Int {
      var result: Int = -1

      for (t in tasks) {
         var var7: Int
         try {
            var7 = t.run()
         } catch (var10: TestCatchBlockJump.Failure) {
            val var9: Int = this.failures++
            break
         }

         result = var7
      }

      return result + this.failures
   }

   public class Failure : RuntimeException()

   public interface Task {
      public abstract fun run(): Int {
      }
   }
}
