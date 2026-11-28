package pkg;

public class TestAmbiguousTypeVariableArgument {
   public interface Sink {
      void put(Object o);
   }

   public static class Buf implements Sink {
      public int count;

      public void put(Object o) {
         this.count++;
      }
   }

   public interface Maker {
      Object make();
   }

   public <A extends Sink> A appendTo(A sink, Object[] parts) {
      for (Object part : parts) {
         sink.put(part);
      }
      return sink;
   }

   public Buf appendTo(Buf buf, Object[] parts) {
      this.appendTo((Sink) buf, parts);
      return buf;
   }

   public Buf appendAll(Buf buf, Object[] parts) {
      return (Buf) this.appendTo((Sink) buf, parts);
   }

   public static <T> String show(T value) {
      return "generic";
   }

   public static String show(String value) {
      return "string";
   }

   public String showGeneric(String value) {
      return show((Object) value);
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

   public static Maker make(Maker maker) {
      return maker;
   }

   public Object lambda() {
      return make((Maker) () -> "x");
   }
}
