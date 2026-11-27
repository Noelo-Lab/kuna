package pkg;

public class TestJavacLoops {
   interface Source {
      boolean more();

      int next();
   }

   private int total;
   private int skipped;
   private int offset;

   int lengths(int[][] rows) {
      for (int[] row : rows) {
         if (row != null) {
            this.total += row.length;
         } else {
            return -1;
         }
      }
      return this.total;
   }

   int prefix(int[] values, int n) {
      int sum = 0;
      for (int i = 0; i < n; i++) {
         if (values[i] >= 0) {
            sum += values[i];
         } else {
            return -1;
         }
      }
      return sum;
   }

   void classify(Source source) {
      while (source.more()) {
         int v = source.next();
         if (v > 10) {
            this.total += v;
         } else if (v > 0) {
            this.skipped += v;
         } else {
            throw new IllegalStateException("negative " + v);
         }
      }
   }

   int scan(int[] seq) {
      while (this.offset < seq.length) {
         int v = seq[this.offset++];
         if (v == 0) {
            this.skipped++;
            continue;
         }
         this.total += v;
      }
      return this.total;
   }

   int count(Source source) {
      while (source.more()) {
         int v = source.next();
         if (v < 0) {
            this.skipped++;
            continue;
         }
         String text = String.valueOf(v);
         this.total += text.length();
      }
      return this.total;
   }

   int sum(int[][] rows) {
      int n = 0;
      for (int[] row : rows) {
         if (row == null) {
            continue;
         }
         for (int i = 0; i < row.length; i++) {
            n += row[i];
         }
      }
      return n;
   }

   static int find(int[][] rows, int key) {
      for (int[] row : rows) {
         if (row == null) {
            continue;
         }
         if (row.length == 0) {
            continue;
         }
         if (row[0] != key) {
            continue;
         }
         return row.length;
      }
      return -1;
   }

   static void validate(long[] limits) {
      for (long l : limits) {
         if (l == -1L) {
            continue;
         }
         if (l < 0L) {
            throw new IllegalStateException("negative " + l);
         }
      }
   }

   static int slot(Object[] input, Object key) {
      int n = input.length;
      int i;
      for (i = 0; i < n; i++) {
         if (input[i] == key) {
            break;
         }
      }
      return i;
   }

   static int last(Source source) {
      while (true) {
         int current = source.next();
         if (!source.more()) {
            return current;
         }
      }
   }

   int checked(int[] values) {
      int n = 0;
      try {
         for (int v : values) {
            n += 100 / v;
         }
      } catch (ArithmeticException e) {
         throw new IllegalStateException("zero", e);
      }
      return n;
   }
}
