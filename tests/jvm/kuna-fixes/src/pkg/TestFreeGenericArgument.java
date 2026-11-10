package pkg;

public class TestFreeGenericArgument {
   public interface Pred<T> {
      boolean test(T t);

      default Pred<T> and(Pred<? super T> other) {
         return t -> test(t) && other.test(t);
      }
   }

   public interface Fn<A, B> {
      B apply(A a);
   }

   public static final class Preds {
      public static <T> Pred<T> always() {
         return t -> true;
      }
   }

   public static final class Seq<T> {
      final T[] items;

      Seq(T[] items) {
         this.items = items;
      }

      @SafeVarargs
      public static <T> Seq<T> of(T... values) {
         return new Seq<>(values);
      }

      public <R> Seq<R> flatMap(Fn<? super T, Seq<R>> f) {
         return null;
      }

      public <R> Seq<R> map(Fn<? super T, R> f) {
         return null;
      }

      public Lst<T> toList() {
         return null;
      }
   }

   public interface Lst<T> {
      int size();
   }

   public static final class Shard {
      boolean primary;
      boolean started;

      public boolean isPrimary() {
         return primary;
      }

      public boolean isStarted() {
         return started;
      }
   }

   static final Lst<Pred<Shard>> CLAUSES = Seq.<Pred<Shard>>of(s -> !s.isPrimary(), Preds.always())
      .flatMap(p -> Seq.<Pred<Shard>>of(Shard::isStarted, Preds.always()).map(p::and))
      .toList();

   static int count(Lst<Shard> shards) {
      return Seq.<Pred<Shard>>of(s -> s.isStarted(), pkglib.ExtPreds.never()).toList().size() + shards.size();
   }

   static Seq<Pred<Shard>> pertinent(Pred<Shard> first) {
      return Seq.of(first, Preds.always());
   }
}
