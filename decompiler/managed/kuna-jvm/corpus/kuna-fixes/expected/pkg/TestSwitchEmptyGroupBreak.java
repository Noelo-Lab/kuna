package pkg;

public class TestSwitchEmptyGroupBreak {
   int total;

   static int decode(char c, int pos) {
      switch (c) {
         case 'p':
            pos += 3;
            break;
         case 'q':
            pos += 2;
            break;
         case '.':
         case '/':
            break;
         case 'a':
            pos = 7;
            break;
         case 'm':
            pos = 9;
            break;
         default:
            pos = -pos;
      }

      return pos;
   }

   void add(int k, int v) {
      switch (k) {
         case 1:
            this.total += 2;
            break;
         case 2:
            int w = v * 3;
            if (w < 0) {
               throw new IllegalArgumentException();
            }

            this.total = w + 1;
      }

      this.total--;
   }
}
