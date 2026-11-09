package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestVarMergeLiveness.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestVarMergeLiveness.kt\npkg/TestVarMergeLiveness\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,18:1\n1#2:19\n*E\n"])
public class TestVarMergeLiveness {
   public fun find(k: pkg.TestVarMergeLiveness.Key): pkg.TestVarMergeLiveness.Node? {
      return if (k.v > 0) TestVarMergeLiveness.Node(k.v) else null
   }

   public fun empty(): pkg.TestVarMergeLiveness.Node {
      return TestVarMergeLiveness.Node(0)
   }

   public fun larger(a: pkg.TestVarMergeLiveness.Key?, b: pkg.TestVarMergeLiveness.Key?): pkg.TestVarMergeLiveness.Node {
      var var10000: TestVarMergeLiveness.Node
      run label30@{
         if (a != null) {
            var10000 = this.find(a)
            if (var10000 != null) {
               return@label30
            }
         }

         var10000 = this.empty()
      }

      var var9: TestVarMergeLiveness.Node
      run label33@{
         if (b != null) {
            var9 = this.find(b)
            if (var9 != null) {
               return@label33
            }
         }

         var9 = this.empty()
      }

      return if (var10000.size > var9.size) var10000 else var9
   }

   public class Key(v: Int) {
      public final val v: Int

      init {
         this.v = v
      }
   }

   public class Node(size: Int) {
      public final val size: Int

      init {
         this.size = size
      }
   }
}
