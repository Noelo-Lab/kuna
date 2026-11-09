package ext;

public final class TestEnum2 extends Enum<TestEnum2> {
   public static final TestEnum2 A = new TestEnum2("A", 0);// 4
   public static final TestEnum2 B = new TestEnum2("B", 1);// 5
   public static final TestEnum2 C = new TestEnum2("C", 2);// 6

   public static TestEnum2[] values() {
      return (TestEnum2[])$VALUES.clone();
   }

   public static TestEnum2 valueOf(String name) {
      return (TestEnum2)Enum.valueOf(TestEnum2.class, name);
   }

   private TestEnum2(String param1, int nullx) {
      super(var1, nullx);// 3
   }
}

class 'ext/TestEnum2' {
   method '<clinit> ()V' {
      4      3
      5      3
      6      3
      11      4
      12      4
      13      4
      1e      5
      1f      5
      20      5
   }

   method 'values ()[Lext/TestEnum2;' {
      0      8
      1      8
      2      8
      3      8
      4      8
      5      8
      6      8
      7      8
      8      8
      9      8
   }

   method 'valueOf (Ljava/lang/String;)Lext/TestEnum2;' {
      0      12
      1      12
      2      12
      3      12
      4      12
      5      12
      6      12
      7      12
      8      12
      9      12
   }

   method '<init> (Ljava/lang/String;I)V' {
      1      16
      2      16
      3      16
      4      16
      5      16
      6      17
   }
}

Lines mapping:
3 <-> 17
4 <-> 4
5 <-> 5
6 <-> 6
