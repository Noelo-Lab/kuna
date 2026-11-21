package pkg;

public class TestCharIntSlot {
   public static int offset(CharSequence s, int base) {
      int i = base;
      char sign;
      {
         char c = s.charAt(0);
         if (c == '+' || c == '-') {
            i++;
            sign = c;
         } else {
            sign = ' ';
         }
      }
      int start = i;
      return start * 2 + (sign == '-' ? 1 : 0);
   }
}
