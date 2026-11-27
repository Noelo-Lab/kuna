package pkg;

public class TestTernaryIntLocal {
   interface Source {
      int next();
   }

   static int decode(Source d) {
      int logical = 0;
      int sum = 0;
      for (;;) {
         int id = d.next();
         if (id == 0) {
            break;
         } else if (id == 1) {
            logical = d.next();
         }
         sum += id;
      }
      int result = logical != 0 ? logical : sum;
      return result;
   }

   static int orDefault(Source d) {
      int value = 0;
      while (d.next() == 1) {
         if (d.next() == 2) {
            value = d.next();
         }
      }
      int result = value != 0 ? value : 7;
      return result;
   }
}
