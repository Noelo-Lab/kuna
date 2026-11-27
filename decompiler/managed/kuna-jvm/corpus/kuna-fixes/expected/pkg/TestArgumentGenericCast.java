package pkg;

public class TestArgumentGenericCast {
   static int writeGeneric(TestArgumentGenericCast.Bag<String, Object> m) {
      return m.size();
   }

   static int writeTo(TestArgumentGenericCast.Bag<String, String> inner) {
      return writeGeneric((TestArgumentGenericCast.Bag<String, Object>)(TestArgumentGenericCast.Bag)inner);
   }

   static boolean load(TestArgumentGenericCast.Lst<TestArgumentGenericCast.Bag<String, ?>> result, TestArgumentGenericCast.Props props) {
      return result.add((TestArgumentGenericCast.Bag<String, ?>)(TestArgumentGenericCast.Bag)props);
   }

   public static void main(String[] args) {
      TestArgumentGenericCast.Props p = new TestArgumentGenericCast.Props();
      System.out
         .println(
            (new TestArgumentGenericCast.BagSource("x", p)).source.size() + writeTo(null == args ? null : new TestArgumentGenericCast.Bag<String, String>() {
               @Override
               public int size() {
                  return 2;
               }
            }) + (load(new TestArgumentGenericCast.Lst<>(), p) ? 1 : 0) + ((new TestArgumentGenericCast.Sorted()).cmp == null ? 0 : 1)
         );
   }

   interface Bag<K, V> {
      int size();
   }

   static class BagSource extends TestArgumentGenericCast.Src<TestArgumentGenericCast.Bag<String, Object>> {
      BagSource(String name, TestArgumentGenericCast.Props source) {
         super(name, (TestArgumentGenericCast.Bag<String, Object>)(TestArgumentGenericCast.Bag)source);
      }
   }

   interface Cmp<T> {
      int compare(T var1, T var2);
   }

   static class Lst<E> {
      int n;

      boolean add(E e) {
         this.n++;
         return true;
      }
   }

   static class Ord<T> implements TestArgumentGenericCast.Cmp<T> {
      static <C extends TestArgumentGenericCast.Rank> TestArgumentGenericCast.Ord<C> natural() {
         return new TestArgumentGenericCast.Ord<>();
      }

      @Override
      public int compare(T a, T b) {
         return 0;
      }
   }

   static class Props implements TestArgumentGenericCast.Bag<Object, Object> {
      @Override
      public int size() {
         return 1;
      }
   }

   interface Rank<T> {
      int rank(T var1);
   }

   static class Sorted<E> {
      final TestArgumentGenericCast.Cmp<? super E> cmp;

      Sorted() {
         this((TestArgumentGenericCast.Cmp<? super E>)TestArgumentGenericCast.Ord.natural());
      }

      Sorted(TestArgumentGenericCast.Cmp<? super E> cmp) {
         this.cmp = cmp;
      }
   }

   static class Src<T> {
      final T source;

      Src(String name, T source) {
         this.source = source;
      }
   }
}
