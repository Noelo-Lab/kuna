package pkg;

public class TestCapturedClashName {
   public interface Reader {
      int read();
   }

   public interface Opener {
      Reader open(int[] data);
   }

   interface Task {
      int run(int base);
   }

   static int apply(Task task, int base) {
      return task.run(base);
   }

   public static Opener opener(int[] data) {
      int first = data[0];
      Opener opener = new Opener() {
         public Reader open(int[] other) {
            int first = other[0];
            return new Reader() {
               public int read() {
                  return first;
               }
            };
         }
      };
      return first > 0 ? opener : null;
   }

   public static Opener lambda(int[] data) {
      int total = data.length;
      Opener opener = new Opener() {
         public Reader open(int[] other) {
            int total = other.length * 2;
            int sum = apply(base -> base + total, 1);
            return () -> sum;
         }
      };
      return total > 0 ? opener : null;
   }
}
