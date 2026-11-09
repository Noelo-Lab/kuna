package pkg;

public class TestWildcardArrayBound {
   public interface Source<T> {
      T get();
   }

   public abstract static class Encoder<T> {
      public abstract int encode(Source<? extends T> source);
   }

   public static class BytesEncoder extends Encoder<byte[]> {
      @Override
      public int encode(Source<? extends byte[]> source) {
         return source.get().length;
      }
   }

   public static int count(Source<? super int[]> sink, Source<? extends String[]> names) {
      return names.get().length;
   }
}
