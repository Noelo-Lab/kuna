package pkg;

public class TestConcatOperandTypeBuilder {
   boolean closed = true;
   String low = "1";
   String high = "2";
   float f = 0.1F;
   int n = 66;
   char c = 'A';
   long l = 3L;

   public String toString() {
      return "range:" + (this.closed ? '[' : '(') + this.low + ',' + this.high + (this.closed ? ']' : ')');
   }

   static String describe(char c2) {
      return "char '" + c2 + "' with value " + (int)c2;
   }

   String widened() {
      return "f=" + (double)this.f + " n=" + (float)this.n + " d=" + (double)this.n + " l=" + (double)this.l + " L=" + (long)this.n;
   }

   String chars() {
      return "c=" + (char)this.n + (char)(this.n + 1) + " i=" + (int)this.c + (this.c + 1) + 'x' + 65;
   }

   String append(String s, char ch, float x) {
      s = s + (int)ch;
      s = s + (double)x;
      return s;
   }

   static String values(char ch, float x) {
      return (int)ch + String.valueOf(x);
   }
}
