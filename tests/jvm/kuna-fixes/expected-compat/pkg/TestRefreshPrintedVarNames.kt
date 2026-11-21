package pkg

public class TestRefreshPrintedVarNames(step: Long) {
   private final val step: Long
   public final val countdown: RefreshCountdown

   init {
      this.step = step
      this.countdown = object : RefreshCountdown {
         public override fun drain(count: Long): Long {
            var countx: Long = countx
            var steps: Long = 0L

            while (countx > 0L) {
               countx -= TestRefreshPrintedVarNames.this.step
               steps += 1L
            }

            return steps
         }
      }
   }
}
