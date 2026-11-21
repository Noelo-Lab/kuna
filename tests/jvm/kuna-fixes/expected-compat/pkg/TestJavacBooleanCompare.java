package pkg;

public class TestJavacBooleanCompare {
   static final boolean DEFAULT_ON = true;
   TestJavacBooleanCompare.Box items;
   boolean flag;

   boolean nonEmpty() {
      return !this.items.isEmpty();
   }

   boolean guard(Object o) {
      return this.items.has(o);
   }

   boolean either(TestJavacBooleanCompare.Box b) {
      return b != null && !b.isEmpty();
   }

   boolean both(long a, double b) {
      return a != 0L || b != 0.0;
   }

   boolean onTrue(Object o) {
      return this.items.has(o);
   }

   boolean changed() {
      return !this.flag;
   }

   boolean plain(Object o) {
      return !this.items.has(o);
   }

   boolean notBox(Object o) {
      return !(o instanceof TestJavacBooleanCompare.Box b) ? false : b.isEmpty();
   }

   interface Box {
      boolean isEmpty();

      boolean has(Object var1);
   }
}
