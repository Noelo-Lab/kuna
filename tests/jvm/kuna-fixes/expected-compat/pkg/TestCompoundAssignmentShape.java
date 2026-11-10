package pkg;

public class TestCompoundAssignmentShape {
   long total;
   int count;
   byte bits;
   char letter;
   int[] slots = new int[4];
   TestCompoundAssignmentShape next;

   static int bump(int[] a) {
      a[0] = 10;
      return 1;
   }

   void addLength(String s) {
      this.total = this.total + s.length();
   }

   void addOther(TestCompoundAssignmentShape other) {
      this.total = this.total + other.total;
   }

   void addPlain(int n) {
      this.count += n;
   }

   void mask(int m) {
      this.bits = (byte)(this.bits & m);
   }

   void addLong(long delta) {
      this.count = (int)(this.count + delta);
   }

   void shiftLetter(int n) {
      this.letter = (char)(this.letter + n);
   }

   void clearBits(byte[] dest, byte[] src, int i) {
      dest[i] = (byte)(dest[i] & ~src[i]);
   }

   void addSlot(int i) {
      this.slots[i] = this.slots[i] + 2;
   }

   void addNext() {
      this.next.count += 3;
   }

   void orFirst(int y) {
      this.count |= y;
   }

   static int bumpFirst(int[] a) {
      a[0] += bump(a);
      return a[0];
   }
}
