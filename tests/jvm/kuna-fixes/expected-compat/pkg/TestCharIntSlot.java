package pkg;

public class TestCharIntSlot {
   public static int offset(CharSequence s, int base) {
      int i = base;
      char c = s.charAt(0);
      char sign;
      if (c != '+' && c != '-') {
         sign = ' ';
      } else {
         i++;
         sign = c;
      }

      c = (char)i;
      return c * 2 + (sign == 45 ? 1 : 0);
   }
}
