package pkg;

public class TestJavacLoops {
   private int total;
   private int skipped;
   private int offset;

   int lengths(int[][] rows) {
      for (int[] row : rows) {
         if (row == null) {
            return -1;
         }

         this.total += row.length;
      }

      return this.total;
   }

   int prefix(int[] values, int n) {
      int sum = 0;

      for (int i = 0; i < n; i++) {
         if (values[i] < 0) {
            return -1;
         }

         sum += values[i];
      }

      return sum;
   }

   void classify(TestJavacLoops.Source source) {
      while (source.more()) {
         int v = source.next();
         if (v > 10) {
            this.total += v;
         } else {
            if (v <= 0) {
               throw new IllegalStateException("negative " + v);
            }

            this.skipped += v;
         }
      }
   }

   int scan(int[] seq) {
      while (this.offset < seq.length) {
         int v = seq[this.offset++];
         if (v == 0) {
            this.skipped++;
         } else {
            this.total += v;
         }
      }

      return this.total;
   }

   int count(TestJavacLoops.Source source) {
      while (source.more()) {
         int v = source.next();
         if (v < 0) {
            this.skipped++;
         } else {
            String text = String.valueOf(v);
            this.total = this.total + text.length();
         }
      }

      return this.total;
   }

   int sum(int[][] rows) {
      int n = 0;

      for (int[] row : rows) {
         if (row != null) {
            for (int i = 0; i < row.length; i++) {
               n += row[i];
            }
         }
      }

      return n;
   }

   static int find(int[][] rows, int key) {
      for (int[] row : rows) {
         if (row != null && row.length != 0 && row[0] == key) {
            return row.length;
         }
      }

      return -1;
   }

   static void validate(long[] limits) {
      for (long l : limits) {
         if (l != -1L && l < 0L) {
            throw new IllegalStateException("negative " + l);
         }
      }
   }

   static int slot(Object[] input, Object key) {
      int n = input.length;
      int i = 0;

      while (i < n && input[i] != key) {
         i++;
      }

      return i;
   }

   static int last(TestJavacLoops.Source source) {
      int current;
      do {
         current = source.next();
      } while (source.more());

      return current;
   }

   int checked(int[] values) {
      int n = 0;

      try {
         for (int v : values) {
            n += 100 / v;
         }

         return n;
      } catch (ArithmeticException e) {
         throw new IllegalStateException("zero", e);
      }
   }

   interface Source {
      boolean more();

      int next();
   }
}
