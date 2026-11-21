package pkg;

import pkglib.ExtPreds;

public class TestFreeGenericArgument {
   static final TestFreeGenericArgument.Lst<TestFreeGenericArgument.Pred<TestFreeGenericArgument.Shard>> CLAUSES = TestFreeGenericArgument.Seq.of(
         s -> !s.isPrimary(), TestFreeGenericArgument.Preds.always()
      )
      .flatMap(p -> TestFreeGenericArgument.Seq.of(TestFreeGenericArgument.Shard::isStarted, TestFreeGenericArgument.Preds.always()).map(p::and))
      .toList();

   static int count(TestFreeGenericArgument.Lst<TestFreeGenericArgument.Shard> shards) {
      return TestFreeGenericArgument.Seq.of(s -> s.isStarted(), ExtPreds.never()).toList().size() + shards.size();
   }

   static TestFreeGenericArgument.Seq<TestFreeGenericArgument.Pred<TestFreeGenericArgument.Shard>> pertinent(
      TestFreeGenericArgument.Pred<TestFreeGenericArgument.Shard> first
   ) {
      return TestFreeGenericArgument.Seq.of(first, TestFreeGenericArgument.Preds.always());
   }

   public interface Fn<A, B> {
      B apply(A var1);
   }

   public interface Lst<T> {
      int size();
   }

   public interface Pred<T> {
      boolean test(T var1);

      default TestFreeGenericArgument.Pred<T> and(TestFreeGenericArgument.Pred<? super T> other) {
         return t -> this.test(t) && other.test(t);
      }
   }

   public static final class Preds {
      public static <T> TestFreeGenericArgument.Pred<T> always() {
         return t -> true;
      }
   }

   public static final class Seq<T> {
      final T[] items;

      Seq(T[] items) {
         this.items = items;
      }

      @SafeVarargs
      public static <T> TestFreeGenericArgument.Seq<T> of(T... values) {
         return new TestFreeGenericArgument.Seq<>(values);
      }

      public <R> TestFreeGenericArgument.Seq<R> flatMap(TestFreeGenericArgument.Fn<? super T, TestFreeGenericArgument.Seq<R>> f) {
         return null;
      }

      public <R> TestFreeGenericArgument.Seq<R> map(TestFreeGenericArgument.Fn<? super T, R> f) {
         return null;
      }

      public TestFreeGenericArgument.Lst<T> toList() {
         return null;
      }
   }

   public static final class Shard {
      boolean primary;
      boolean started;

      public boolean isPrimary() {
         return this.primary;
      }

      public boolean isStarted() {
         return this.started;
      }
   }
}
