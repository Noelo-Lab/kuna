package pkg;

public class TestAssertDeadThrow {
   private int total;

   public int[] spans(int[][] starts, int[][] ends, int limit) {
      int[] out = new int[starts.length];

      for (int j = 0; j < starts.length; j++) {
         int pos = 0;

         for (int i = 0; i < starts[j].length; i++) {
            int start = starts[j][i];
            assert start >= pos && start < limit;
            this.add(pos, start);
            int end = ends[j][i];
            assert end > start : "bad span " + start + " " + end;

            while (i + 1 < starts[j].length && starts[j][i + 1] < end) {
               end = max(ends[j][++i], end);
            }

            pos = end;
         }

         out[j] = pos;
      }

      return out;
   }

   public int widths(int[][] rows, int limit) {
      int sum = 0;

      for (int[] row : rows) {
         int pos = 0;

         for (int i = 0; i < row.length; i++) {
            int width = row[i];
            assert width >= pos && width < limit : "row " + row + " at " + pos;
            this.add(pos, width);
            int next = width + 1;
            assert next > width;

            while (i + 1 < row.length && row[i + 1] < width) {
               width = max(row[++i], width);
            }

            pos = width;
         }

         sum += pos;
      }

      return sum;
   }

   private static int max(int a, int b) {
      return a > b ? a : b;
   }

   private void add(int from, int to) {
      this.total += to - from;
   }
}
