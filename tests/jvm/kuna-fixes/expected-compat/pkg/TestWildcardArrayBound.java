package pkg;

public class TestWildcardArrayBound {
   public static int count(TestWildcardArrayBound.Source<int[]> sink, TestWildcardArrayBound.Source<? extends String[]> names) {
      return ((String[])names.get()).length;
   }

   public static class BytesEncoder extends TestWildcardArrayBound.Encoder<byte[]> {
      @Override
      public int encode(TestWildcardArrayBound.Source<byte[]> source) {
         return source.get().length;
      }
   }

   public abstract static class Encoder<T> {
      public abstract int encode(TestWildcardArrayBound.Source<? extends T> var1);
   }

   public interface Source<T> {
      T get();
   }
}
