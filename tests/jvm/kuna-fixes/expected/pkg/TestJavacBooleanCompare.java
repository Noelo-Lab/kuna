package pkg;

public class TestJavacBooleanCompare {
   static final boolean DEFAULT_ON = true;
   TestJavacBooleanCompare.Box items;
   boolean flag;

   boolean nonEmpty() {
      return false == this.items.isEmpty();
   }

   boolean guard(Object o) {
      if (false == this.items.has(o)) {
         return false;
      }

      return true;
   }

   boolean either(TestJavacBooleanCompare.Box b) {
      return (b == null || b.isEmpty()) == false;
   }

   boolean both(long a, double b) {
      return (a == 0L && b == 0.0) == false;
   }

   boolean onTrue(Object o) {
      return this.items.has(o) == true;
   }

   boolean changed() {
      return this.flag != true;
   }

   boolean plain(Object o) {
      return !this.items.has(o);
   }

   boolean notBox(Object o) {
      if (false == (o instanceof TestJavacBooleanCompare.Box)) {
         return false;
      }

      TestJavacBooleanCompare.Box b = (TestJavacBooleanCompare.Box)o;
      return b.isEmpty();
   }

   interface Box {
      boolean isEmpty();

      boolean has(Object var1);
   }
}
