package pkg;

public class TestLoopCarriedLocal {
   private int seed = 17;

   int step(int k) {
      this.seed = this.seed * 31 + k;
      return this.seed;
   }

   long draw(long bound) {
      long acc;
      long rem;
      do {
         acc = (long)this.step(3) << 20;
         acc ^= this.step(5) & 4095L;
         rem = acc % bound;
      } while (acc - rem < 0L);

      return rem;
   }

   double settle(double x) {
      int n = (int)x;

      double head;
      double tail;
      while (true) {
         double t = n * 0.25;
         head = x - t;
         tail = head * t;
         t = n * 0.5;
         double u = head;
         head = t + u;
         tail += head - u;
         if (head > 0.0) {
            break;
         }

         n--;
      }

      return head + tail;
   }
}
