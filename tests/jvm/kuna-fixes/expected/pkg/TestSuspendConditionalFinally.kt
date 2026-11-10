package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.Boxing
import kotlin.coroutines.jvm.internal.ContinuationImpl

public class TestSuspendConditionalFinally {
   public suspend fun drain(sink: pkg.TestSuspendConditionalFinally.Sink, src: pkg.TestSuspendConditionalFinally.Source, consume: Boolean) {
      var `$continuation`: Continuation
      run label908@{
         if (`$completion` is <unrepresentable>) {
            `$continuation` = `$completion` as <unrepresentable>
            if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
               `$continuation`.label -= -2147483648
               return@label908
            }
         }

         `$continuation` = object : ContinuationImpl(`$completion`) {
            @JvmField internal var `L$0`: Any? = null
            @JvmField internal var `L$1`: Any? = null
            @JvmField internal var `L$2`: Any? = null
            @JvmField internal var `Z$0`: Boolean = false
            @JvmField internal var label: Int = 0

            fun invokeSuspend(`$result`: Any): Any? {
               this.result = `$result`
               this.label = this.label or -2147483648
               return this@TestSuspendConditionalFinally.drain(null, null, false, this as Continuation<in Unit>)
            }
         }
      }

      var cause: Throwable
      run label949@{
         val `$result`: Any = `$continuation`.result
         val var9: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
         var e: TestSuspendConditionalFinally.Cursor
         var var28: Boolean
         var var47: Boolean
         when (`$continuation`.label) {
            0 -> {
               ResultKt.throwOnFailure(`$result`)
               cause = null
               var var123: Boolean = false /* VF: Semaphore variable */

               try {
                  var123 = true
                  e = src.open()
                  var123 = false
               } catch (var126: Throwable) {
                  cause = var126
                  throw var126
               } finally {
                  if (var123) {
                     if (consume) {
                        src.cancel(cause)
                     }
                  }
               }
            }
            1 -> {
               consume = `$continuation`.`Z$0`
               e = `$continuation`.`L$2` as TestSuspendConditionalFinally.Cursor
               cause = null
               src = `$continuation`.`L$1` as TestSuspendConditionalFinally.Source
               sink = `$continuation`.`L$0` as TestSuspendConditionalFinally.Sink
               var var85: Boolean = false /* VF: Semaphore variable */

               var var10000: Any
               try {
                  var85 = true
                  ResultKt.throwOnFailure(`$result`)
                  var10000 = `$result`
                  var85 = false
               } catch (var124: Throwable) {
                  cause = var124
                  throw var124
               } finally {
                  if (var85) {
                     if (consume) {
                        src.cancel(cause)
                     }
                  }
               }

               var47 = false

               try {
                  var47 = true
                  if (!(var10000 as Boolean)) {
                     var47 = false
                     return@label949
                  }

                  val var10001: Int = e.take()
                  `$continuation`.`L$0` = sink
                  `$continuation`.`L$1` = src
                  `$continuation`.`L$2` = e
                  `$continuation`.`Z$0` = consume
                  `$continuation`.label = 2
                  var10000 = sink.put(var10001, `$continuation`)
                  var47 = false
               } catch (var139: Throwable) {
                  cause = var139
                  throw var139
               } finally {
                  if (var47) {
                     if (consume) {
                        src.cancel(cause)
                     }
                  }
               }

               if (var10000 === var9) {
                  return var9
               }

               var28 = false

               try {
                  var28 = true
                  var28 = false
               } catch (var141: Throwable) {
                  cause = var141
                  throw var141
               } finally {
                  if (var28) {
                     if (consume) {
                        src.cancel(cause)
                     }
                  }
               }
            }
            2 -> {
               consume = `$continuation`.`Z$0`
               e = `$continuation`.`L$2` as TestSuspendConditionalFinally.Cursor
               cause = null
               src = `$continuation`.`L$1` as TestSuspendConditionalFinally.Source
               sink = `$continuation`.`L$0` as TestSuspendConditionalFinally.Sink
               var var104: Boolean = false /* VF: Semaphore variable */

               try {
                  var104 = true
                  ResultKt.throwOnFailure(`$result`)
                  var104 = false
               } catch (var128: Throwable) {
                  cause = var128
                  throw var128
               } finally {
                  if (var104) {
                     if (consume) {
                        src.cancel(cause)
                     }
                  }
               }

               var28 = false

               try {
                  var28 = true
                  var28 = false
               } catch (var137: Throwable) {
                  cause = var137
                  throw var137
               } finally {
                  if (var28) {
                     if (consume) {
                        src.cancel(cause)
                     }
                  }
               }
            }
            else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
         }

         while (true) {
            var var66: Boolean = false /* VF: Semaphore variable */

            var var166: Any
            try {
               var66 = true
               `$continuation`.`L$0` = sink
               `$continuation`.`L$1` = src
               `$continuation`.`L$2` = e
               `$continuation`.`Z$0` = consume
               `$continuation`.label = 1
               var166 = e.hasMore(`$continuation`)
               var66 = false
            } catch (var125: Throwable) {
               cause = var125
               throw var125
            } finally {
               if (var66) {
                  if (consume) {
                     src.cancel(cause)
                  }
               }
            }

            if (var166 === var9) {
               return var9
            }

            var47 = false

            try {
               var47 = true
               if (!(var166 as Boolean)) {
                  var47 = false
                  break
               }

               val var168: Int = e.take()
               `$continuation`.`L$0` = sink
               `$continuation`.`L$1` = src
               `$continuation`.`L$2` = e
               `$continuation`.`Z$0` = consume
               `$continuation`.label = 2
               var166 = sink.put(var168, `$continuation`)
               var47 = false
            } catch (var134: Throwable) {
               cause = var134
               throw var134
            } finally {
               if (var47) {
                  if (consume) {
                     src.cancel(cause)
                  }
               }
            }

            if (var166 === var9) {
               return var9
            }

            var28 = false

            try {
               var28 = true
               var28 = false
            } catch (var127: Throwable) {
               cause = var127
               throw var127
            } finally {
               if (var28) {
                  if (consume) {
                     src.cancel(cause)
                  }
               }
            }
         }
      }

      if (consume) {
         src.cancel(cause)
      }

      return Unit.INSTANCE
   }

   public suspend fun sum(src: pkg.TestSuspendConditionalFinally.Source, cursor: pkg.TestSuspendConditionalFinally.Cursor, flag: Boolean): Int {
      var `$continuation`: Continuation
      run label391@{
         if (`$completion` is <unrepresentable>) {
            `$continuation` = `$completion` as <unrepresentable>
            if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
               `$continuation`.label -= -2147483648
               return@label391
            }
         }

         `$continuation` = object : ContinuationImpl(`$completion`) {
            @JvmField internal var `L$0`: Any? = null
            @JvmField internal var `L$1`: Any? = null
            @JvmField internal var `Z$0`: Boolean = false
            @JvmField internal var `I$0`: Int = 0
            @JvmField internal var label: Int = 0

            fun invokeSuspend(`$result`: Any): Any? {
               this.result = `$result`
               this.label = this.label or -2147483648
               return this@TestSuspendConditionalFinally.sum(null, null, false, this as Continuation<in Int>)
            }
         }
      }

      var var6: Int
      run label402@{
         val `$result`: Any = `$continuation`.result
         val var9: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
         var var15: Boolean
         var var39: Int
         when (`$continuation`.label) {
            0 -> {
               ResultKt.throwOnFailure(`$result`)
               var var33: Boolean = false /* VF: Semaphore variable */

               try {
                  var33 = true
                  var39 = 0
                  var33 = false
               } finally {
                  if (var33) {
                     src.closeIf(flag)
                  }
               }
            }
            1 -> {
               var39 = `$continuation`.`I$0`
               flag = `$continuation`.`Z$0`
               cursor = `$continuation`.`L$1` as TestSuspendConditionalFinally.Cursor
               src = `$continuation`.`L$0` as TestSuspendConditionalFinally.Source
               var var27: Boolean = false /* VF: Semaphore variable */

               var var10000: Any
               try {
                  var27 = true
                  ResultKt.throwOnFailure(`$result`)
                  var10000 = `$result`
                  var27 = false
               } finally {
                  if (var27) {
                     src.closeIf(flag)
                  }
               }

               var15 = false

               try {
                  var15 = true
                  if (!(var10000 as Boolean)) {
                     var6 = Boxing.boxInt(var39)
                     var15 = false
                     return@label402
                  }

                  var39 = var39 + cursor.take()
                  var15 = false
               } finally {
                  if (var15) {
                     src.closeIf(flag)
                  }
               }
            }
            else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
         }

         while (true) {
            var var21: Boolean = false /* VF: Semaphore variable */

            var var53: Any
            try {
               var21 = true
               `$continuation`.`L$0` = src
               `$continuation`.`L$1` = cursor
               `$continuation`.`Z$0` = flag
               `$continuation`.`I$0` = var39
               `$continuation`.label = 1
               var53 = cursor.hasMore(`$continuation`)
               var21 = false
            } finally {
               if (var21) {
                  src.closeIf(flag)
               }
            }

            if (var53 === var9) {
               return var9
            }

            var15 = false

            try {
               var15 = true
               if (!(var53 as Boolean)) {
                  var6 = Boxing.boxInt(var39)
                  var15 = false
                  break
               }

               var39 += cursor.take()
               var15 = false
            } finally {
               if (var15) {
                  src.closeIf(flag)
               }
            }
         }
      }

      src.closeIf(flag)
      return var6
   }

   public interface Cursor {
      public abstract suspend fun hasMore(): Boolean

      public abstract fun take(): Int
   }

   public interface Sink {
      public abstract suspend fun put(v: Int)
   }

   public interface Source {
      public abstract fun open(): pkg.TestSuspendConditionalFinally.Cursor

      public abstract fun cancel(cause: Throwable?)

      public abstract fun closeIf(flag: Boolean)
   }
}
