package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.Boxing
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestMembersWithoutMetadata.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestMembersWithoutMetadata.kt\npkg/TestMembersWithoutMetadataKt\$feed$1\n+ 2 TestMembersWithoutMetadata.kt\npkg/TestMembersWithoutMetadataKt\n*L\n1#1,20:1\n27#2,11:21\n*E\n"])
/* $VF: local visibility outside of method */ internal class `TestMembersWithoutMetadataKt$counting$$inlined$feed$1` : NoMetaFeed {
   public constructor(var1: Long, var3: NoMetaFeed, var4: Int) {
      this.`$step$inlined` = var1
      this.`$src$inlined` = var3
      this.`$start$inlined` = var4
   }

   public override suspend fun next(): Int {
      var `$continuation`: Continuation
      run label51@{
         if (`$completion` is <unrepresentable>) {
            `$continuation` = `$completion` as <unrepresentable>
            if ((`$continuation`.label and -2147483648) != 0) {
               `$continuation`.label -= -2147483648
               return@label51
            }
         }

         `$continuation` = object : ContinuationImpl(`$completion`) {
            @JvmField internal var label: Int = 0
            @JvmField internal var `L$0`: Any? = null
            @JvmField internal var `J$0`: Long = 0L
            @JvmField internal var `D$0`: Double = 0.0
            @JvmField internal var `F$0`: Float = 0.0F
            @JvmField internal var `I$0`: Int = 0

            fun invokeSuspend(`$result`: Any): Any {
               this.result = `$result`
               this.label = this.label or -2147483648
               return this@`TestMembersWithoutMetadataKt$counting$$inlined$feed$1`.next(this as Continuation<in Int>)
            }
         }
      }

      var total: Long
      var scale: Double
      var ratio: Float
      var seen: Boolean
      var var10000: Any?
      run label55@{
         val `$result`: Any = `$continuation`.result
         val var4: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
         when (`$continuation`.label) {
            0 -> {
               ResultKt.throwOnFailure(`$result`)
               total = this.`$step$inlined`
               scale = 0.5
               ratio = 1.5F
               val var20: NoMetaFeed = this.`$src$inlined`
               `$continuation`.`L$0` = this
               `$continuation`.`J$0` = total
               `$continuation`.`D$0` = scale
               `$continuation`.`F$0` = ratio
               `$continuation`.label = 1
               var10000 = var20.next(`$continuation`)
               if (var10000 === var4) {
                  return var4
               }
            }
            1 -> {
               ratio = `$continuation`.`F$0`
               scale = `$continuation`.`D$0`
               total = `$continuation`.`J$0`
               this = `$continuation`.`L$0` as TestMembersWithoutMetadataKt$counting$$inlined$feed$1
               ResultKt.throwOnFailure(`$result`)
               var10000 = `$result`
            }
            2 -> {
               seen = (boolean)`$continuation`.`I$0`
               ratio = `$continuation`.`F$0`
               scale = `$continuation`.`D$0`
               total = `$continuation`.`J$0`
               ResultKt.throwOnFailure(`$result`)
               var10000 = `$result`
               return@label55
            }
            else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
         }

         val a: Int = (var10000 as Number).toInt()
         total = total + a
         scale = scale + a
         ratio = ratio + a
         seen = a > this.`$start$inlined`
         val var21: NoMetaFeed = this.`$src$inlined`
         `$continuation`.`L$0` = null
         `$continuation`.`J$0` = total
         `$continuation`.`D$0` = scale
         `$continuation`.`F$0` = ratio
         `$continuation`.`I$0` = seen
         `$continuation`.label = 2
         var10000 = var21.next(`$continuation`)
         if (var10000 === var4) {
            return var4
         }
      }

      val b: Int = (var10000 as Number).toInt()
      return Boxing.boxInt((if (seen != 0) b + total.toInt() + scale.toInt() + ratio.toInt() else b) + 1)
   }
}
