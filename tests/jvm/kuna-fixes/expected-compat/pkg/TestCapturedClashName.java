package pkg;

public class TestCapturedClashName {
   static int apply(TestCapturedClashName.Task task, int base) {
      return task.run(base);
   }

   public static TestCapturedClashName.Opener opener(int[] data) {
      int first = data[0];
      TestCapturedClashName.Opener opener = new TestCapturedClashName.Opener() {
         @Override
         public TestCapturedClashName.Reader open(int[] other) {
            final int firstx = other[0];
            return new TestCapturedClashName.Reader() {
               @Override
               public int read() {
                  return first;
               }
            };
         }
      };
      return first > 0 ? opener : null;
   }

   public static TestCapturedClashName.Opener lambda(int[] data) {
      int total = data.length;
      TestCapturedClashName.Opener opener = new TestCapturedClashName.Opener() {
         @Override
         public TestCapturedClashName.Reader open(int[] other) {
            int totalx = other.length * 2;
            int sum = TestCapturedClashName.apply(base -> base + total, 1);
            return () -> sum;
         }
      };
      return total > 0 ? opener : null;
   }

   public interface Opener {
      TestCapturedClashName.Reader open(int[] var1);
   }

   public interface Reader {
      int read();
   }

   interface Task {
      int run(int var1);
   }
}
