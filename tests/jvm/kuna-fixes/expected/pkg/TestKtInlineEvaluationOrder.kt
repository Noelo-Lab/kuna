package pkg

public class TestKtInlineEvaluationOrder {
   public fun callOrder(src: pkg.TestKtInlineEvaluationOrder.Src) {
      val first: Int = src.next()
      src.take(src.next(), first)
   }

   public fun conditional(src: pkg.TestKtInlineEvaluationOrder.Src, k: Int) {
      val v: Int = src.next()
      src.take1(if (k > 0) v else 0)
   }

   public fun shortCircuit(src: pkg.TestKtInlineEvaluationOrder.Src, enabled: Boolean) {
      val v: Int = src.next()
      if (enabled && v < 0) {
         src.run()
      }
   }

   public fun element(a: IntArray, src: pkg.TestKtInlineEvaluationOrder.Src) {
      val x: Int = a[0]
      src.take(src.next(), x)
   }

   public fun scoped(src: pkg.TestKtInlineEvaluationOrder.Src) {
      val token: Int = src.enter()

      try {
         src.run()
      } finally {
         src.leave(token)
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
