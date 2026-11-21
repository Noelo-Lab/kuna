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
      return "range:" + (this.closed ? 91 : 40) + this.low + ',' + this.high + (this.closed ? 93 : 41);
   }

   static String describe(char c2) {
      return "char '" + c2 + "' with value " + c2;
   }

   String widened() {
      return "f=" + this.f + " n=" + this.n + " d=" + this.n + " l=" + this.l + " L=" + this.n;
   }

   String chars() {
      return "c=" + (char)this.n + (char)(this.n + 1) + " i=" + this.c + (this.c + 1) + 'x' + 65;
   }

   String append(String s, char ch, float x) {
      s = s + ch;
      return s + x;
   }

   static String values(char ch, float x) {
      return ch + String.valueOf(x);
   }
}
