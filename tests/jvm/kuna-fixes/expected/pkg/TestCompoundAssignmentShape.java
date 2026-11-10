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
      this.total += s.length();
   }

   void addOther(TestCompoundAssignmentShape other) {
      this.total += other.total;
   }

   void addPlain(int n) {
      this.count = this.count + n;
   }

   void mask(int m) {
      this.bits &= m;
   }

   void addLong(long delta) {
      this.count += delta;
   }

   void shiftLetter(int n) {
      this.letter += n;
   }

   void clearBits(byte[] dest, byte[] src, int i) {
      dest[i] &= ~src[i];
   }

   void addSlot(int i) {
      this.slots[i] += 2;
   }

   void addNext() {
      this.next.count = this.next.count + 3;
   }

   void orFirst(int y) {
      this.count = y | this.count;
   }

   static int bumpFirst(int[] a) {
      a[0] = bump(a) + a[0];
      return a[0];
   }
}
