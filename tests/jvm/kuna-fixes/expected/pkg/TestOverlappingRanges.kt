package pkg

import java.io.Closeable

public class TestOverlappingRanges(store: pkg.TestOverlappingRanges.Store) {
   private final val store: pkg.TestOverlappingRanges.Store

   init {
      this.store = store
   }

   public fun civilized(res: Closeable, name: String): Boolean {
      val var3: Closeable = res
      var var4: Throwable = null
      var var21: Boolean = false /* VF: Semaphore variable */

      var var8: Boolean
      run label83@{
         try {
            var21 = true

            try {
               this.store.delete(name)
               var8 = true
               var21 = false
               return@label83
            } catch (var25: TestOverlappingRanges.IoFailure) {
               var21 = false
            }
         } catch (var26: Throwable) {
            var4 = var26
            throw var26
         } finally {
            if (var21) {
               CloseableKt.closeFinally(res, var4)
            }
         }

         CloseableKt.closeFinally(res, null)
         this.store.delete(name)
         return false
      }

      try {
         CloseableKt.closeFinally(var3, var4)
         return var8
      } catch (var24: TestOverlappingRanges.IoFailure) {
         try {
         } catch (var22: Throwable) {
            var4 = var22
            throw var22
         } finally {
            CloseableKt.closeFinally(res, var4)
         }

         this.store.delete(name)
         return false
      }
   }

   public fun readUninterruptibly() {
      var interrupted: Boolean = false

      while (true) {
         while (true) {
            var var25: Boolean = false /* VF: Semaphore variable */

            try {
               var25 = true
               this.store.read()
               var25 = false
               break
            } catch (var28: TestOverlappingRanges.InterruptedFailure) {
               this.store.clearInterrupt()
               interrupted = true
               var25 = false
               continue
            } catch (var29: TestOverlappingRanges.IoFailure) {
               this.store.log(var29)
               var25 = false
            } finally {
               if (var25) {
                  if (interrupted) {
                     this.store.interrupt()
                  }
               }
            }

            if (interrupted) {
               this.store.interrupt()
            }

            return
         }

         try {
            if (interrupted) {
               this.store.interrupt()
            }

            return
         } catch (var31: TestOverlappingRanges.InterruptedFailure) {
            var var17: Boolean = false /* VF: Semaphore variable */

            try {
               var17 = true
               this.store.clearInterrupt()
               interrupted = true
               var17 = false
            } finally {
               if (var17) {
                  if (interrupted) {
                     this.store.interrupt()
                  }
               }
            }
         } catch (var32: TestOverlappingRanges.IoFailure) {
            val e: TestOverlappingRanges.IoFailure = var32

            try {
               this.store.log(e)
            } finally {
               if (interrupted) {
                  this.store.interrupt()
               }
            }

            return
         }
      }
   }

   public fun logged(name: String): Boolean {
      val handle: Any = this.store.open(name)
      var cause: Throwable = null
      var var17: Boolean = false /* VF: Semaphore variable */

      var e: Boolean
      run label80@{
         try {
            try {
               var17 = true
               this.store.delete(name)
               e = true
               var17 = false
               return@label80
            } catch (var21: TestOverlappingRanges.IoFailure) {
               this.store.log(var21)
               var17 = false
            }
         } catch (var22: Throwable) {
            cause = var22
            throw var22
         } finally {
            if (var17) {
               this.store.close(handle, cause)
            }
         }

         this.store.close(handle, null)
         this.store.delete(name)
         return false
      }

      try {
         this.store.close(handle, cause)
         return e
      } catch (var20: TestOverlappingRanges.IoFailure) {
         val var24: TestOverlappingRanges.IoFailure = var20

         try {
            this.store.log(var24)
         } catch (ex: Throwable) {
            cause = ex
            throw ex
         } finally {
            this.store.close(handle, cause)
         }

         this.store.delete(name)
         return false
      }
   }

   public fun sharedTail(name: String, flag: Boolean): Boolean {
      val handle: Any = this.store.open(name)
      var cause: Throwable = null
      var var19: Boolean = false /* VF: Semaphore variable */

      run label87@{
         var e: Boolean
         try {
            var19 = true
            if (!flag) {
               var19 = false
               return@label87
            }

            try {
               this.store.delete(name)
               e = true
               var19 = false
            } catch (var23: TestOverlappingRanges.IoFailure) {
               this.store.log(var23)
               var19 = false
               return@label87
            }
         } catch (var24: Throwable) {
            cause = var24
            throw var24
         } finally {
            if (var19) {
               this.store.close(handle, cause)
            }
         }

         try {
            this.store.close(handle, cause)
            return e
         } catch (var22: TestOverlappingRanges.IoFailure) {
            val var26: TestOverlappingRanges.IoFailure = var22
            var var12: Boolean = false /* VF: Semaphore variable */

            try {
               var12 = true
               this.store.log(var26)
               var12 = false
            } catch (ex: Throwable) {
               cause = ex
               throw ex
            } finally {
               if (var12) {
                  this.store.close(handle, cause)
               }
            }
         }
      }

      this.store.close(handle, null)
      this.store.delete(name)
      return false
   }

   public fun countedRead(): Int {
      var interrupted: Boolean = false
      var attempts: Int = 0

      while (true) {
         var var3: Int
         try {
            while (true) {
               attempts++

               try {
                  this.store.read()
                  var3 = attempts
                  break
               } catch (var10: TestOverlappingRanges.InterruptedFailure) {
                  interrupted = true
               }
            }
         } catch (var11: Throwable) {
            if (interrupted) {
               this.store.interrupt()
            }

            throw var11
         }

         try {
            if (interrupted) {
               this.store.interrupt()
            }

            return var3
         } catch (var12: TestOverlappingRanges.InterruptedFailure) {
            var var8: Boolean = false /* VF: Semaphore variable */

            try {
               var8 = true
               interrupted = true
               var8 = false
            } finally {
               if (var8) {
                  if (interrupted) {
                     this.store.interrupt()
                  }
               }
            }
         }
      }
   }

   public class InterruptedFailure : TestOverlappingRanges.IoFailure()

   public open class IoFailure : RuntimeException()

   public interface Store {
      public abstract fun open(name: String): Any

      public abstract fun delete(name: String)

      public abstract fun close(handle: Any, cause: Throwable?)

      public abstract fun read()

      public abstract fun clearInterrupt()

      public abstract fun interrupt()

      public abstract fun log(e: pkg.TestOverlappingRanges.IoFailure)
   }
}
