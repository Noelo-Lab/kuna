package ext;

public final class TestEnum2 extends Enum<TestEnum2> {
   public static final TestEnum2 A = new TestEnum2("A", 0);
   public static final TestEnum2 B = new TestEnum2("B", 1);
   public static final TestEnum2 C = new TestEnum2("C", 2);

   public static TestEnum2[] values() {
      return (TestEnum2[])$VALUES.clone();
   }

   public static TestEnum2 valueOf(String name) {
      return (TestEnum2)Enum.valueOf(TestEnum2.class, name);
   }

   private TestEnum2(String param1, int nullx) {
      super(var1, nullx);
   }
}
