package pkg;

public class TestFieldReadAfterCall {
   int count;
   long total;
   TestFieldReadAfterCall.Sink sink = new TestFieldReadAfterCall.Sink();

   TestFieldReadAfterCall step() {
      this.count++;
      return this;
   }

   void forward(String s) {
      this.step().sink.accept(s, this.count, this.count);
   }

   void chain(String s) {
      this.step().sink.next.accept(s, this.count, (float)this.total);
   }

   void widened() {
      this.step().sink.wide(this.count, this.total);
   }

   static class Sink {
      TestFieldReadAfterCall.Sink next;

      void accept(String s, float a, float b) {
      }

      void wide(long a, double b) {
      }
   }
}
