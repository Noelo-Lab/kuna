package pkg;

public class TestFieldReadAfterCall {
   int count;
   long total;
   Sink sink = new Sink();

   static class Sink {
      Sink next;

      void accept(String s, float a, float b) {
      }

      void wide(long a, double b) {
      }
   }

   TestFieldReadAfterCall step() {
      count++;
      return this;
   }

   void forward(String s) {
      step().sink.accept(s, count, count);
   }

   void chain(String s) {
      step().sink.next.accept(s, count, total);
   }

   void widened() {
      step().sink.wide(count, total);
   }
}
