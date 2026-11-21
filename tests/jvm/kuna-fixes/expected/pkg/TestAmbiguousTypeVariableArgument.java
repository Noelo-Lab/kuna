package pkg;

public class TestAmbiguousTypeVariableArgument {
   public <A extends TestAmbiguousTypeVariableArgument.Sink> A appendTo(A sink, Object[] parts) {
      for (Object part : parts) {
         sink.put(part);
      }

      return sink;
   }

   public TestAmbiguousTypeVariableArgument.Buf appendTo(TestAmbiguousTypeVariableArgument.Buf buf, Object[] parts) {
      this.appendTo((TestAmbiguousTypeVariableArgument.Sink)buf, parts);
      return buf;
   }

   public TestAmbiguousTypeVariableArgument.Buf appendAll(TestAmbiguousTypeVariableArgument.Buf buf, Object[] parts) {
      return (TestAmbiguousTypeVariableArgument.Buf)this.appendTo((TestAmbiguousTypeVariableArgument.Sink)buf, parts);
   }

   public static <T> String show(T value) {
      return "generic";
   }

   public static String show(String value) {
      return "string";
   }

   public String showGeneric(String value) {
      return show((Object)value);
   }

   public String showString(String value) {
      return show(value);
   }

   public static <T> T[] slice(T[] items, int from) {
      return items;
   }

   public static byte[] slice(byte[] items, int from) {
      return items;
   }

   public byte[][] rows(byte[][] grid) {
      return slice(grid, 1);
   }

   public static <T> T make(T maker) {
      return maker;
   }

   public static TestAmbiguousTypeVariableArgument.Maker make(TestAmbiguousTypeVariableArgument.Maker maker) {
      return maker;
   }

   public Object lambda() {
      return make(() -> "x");
   }

   public static class Buf implements TestAmbiguousTypeVariableArgument.Sink {
      public int count;

      @Override
      public void put(Object o) {
         this.count++;
      }
   }

   public interface Maker {
      Object make();
   }

   public interface Sink {
      void put(Object var1);
   }
}
