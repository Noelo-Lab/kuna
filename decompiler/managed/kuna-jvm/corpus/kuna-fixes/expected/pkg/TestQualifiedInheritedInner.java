package pkg;

public class TestQualifiedInheritedInner extends TestQualifiedInheritedInnerBase {
   class Values {
      Object iterator() {
         return TestQualifiedInheritedInner.this.new Cursor(false);
      }
   }
}
