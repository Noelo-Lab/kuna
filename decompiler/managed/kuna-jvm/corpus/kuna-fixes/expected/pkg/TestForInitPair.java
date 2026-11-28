package pkg;

public class TestForInitPair {
   int[] data = new int[8];
   double total;

   int sum(int[] values) {
      int total = 0;

      for (int i = 0, n = values.length; i < n; i++) {
         total += values[i];
      }

      int after = total * 2;
      return after;
   }

   void fill(int count) {
      for (int i = 0, max = count / 2; i < max; i++) {
         this.data[i] = i;
      }

      int k = 0;

      for (int j = k, max = this.data.length; j < max; j++) {
         this.data[j]++;
      }
   }

   int outside(int[] values) {
      int i = 0;

      for (int n = values.length; i < n; i++) {
         this.data[i % 8] = values[i];
      }

      return i;
   }

   void bare(int[] input, int[] out) {
      for (int a = 0, b, c, size = input.length; a < size; a += 3) {
         b = a + 1;
         c = a + 2;
         out[c % 8] = input[a] + b;
      }
   }

   void pivot(double[][] rows, int col) {
      double value;
      for (int i = 0, limit = rows.length; i < limit; i++) {
         value = -rows[i][col];
         if (value != 0.0) {
            this.total += value;
         }
      }

      value = rows.length * 2.0;
      if (value > 1.0) {
         this.total -= value;
      }
   }
}
