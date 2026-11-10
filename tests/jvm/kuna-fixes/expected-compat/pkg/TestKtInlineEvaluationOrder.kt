package pkg

public class TestKtInlineEvaluationOrder {
   public fun callOrder(src: pkg.TestKtInlineEvaluationOrder.Src) {
      src.take(src.next(), src.next())
   }

   public fun conditional(src: pkg.TestKtInlineEvaluationOrder.Src, k: Int) {
      src.take1(if (k > 0) src.next() else 0)
   }

   public fun shortCircuit(src: pkg.TestKtInlineEvaluationOrder.Src, enabled: Boolean) {
      if (enabled && src.next() < 0) {
         src.run()
      }
   }

   public fun element(a: IntArray, src: pkg.TestKtInlineEvaluationOrder.Src) {
      src.take(src.next(), a[0])
   }

   public fun scoped(src: pkg.TestKtInlineEvaluationOrder.Src) {
      try {
         src.run()
      } finally {
         src.leave(src.enter())
      }
   }

   public fun firstUse(src: pkg.TestKtInlineEvaluationOrder.Src, a: IntArray) {
      src.take(src.next(), a[0])
   }

   public interface Src {
      public abstract fun next(): Int {
      }

      public abstract fun take(a: Int, b: Int) {
      }

      public abstract fun take1(a: Int) {
      }

      public abstract fun enter(): Int {
      }

      public abstract fun leave(token: Int) {
      }

      public abstract fun run() {
      }
   }
}
