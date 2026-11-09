package pkg;

import java.util.List;
import kotlin.Metadata;
import kotlin.ResultKt;
import kotlin.coroutines.Continuation;
import kotlin.coroutines.intrinsics.IntrinsicsKt;
import kotlin.coroutines.jvm.internal.Boxing;
import kotlin.coroutines.jvm.internal.ContinuationImpl;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000 \n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010!\n\u0002\b\u0004\u0018\u00002\u00020\u0001:\u0001\rB\u0007¢\u0006\u0004\b\u0002\u0010\u0003J$\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u00072\f\u0010\b\u001a\b\u0012\u0004\u0012\u00020\u00050\tH\u0086@¢\u0006\u0002\u0010\nJ\u0016\u0010\u000b\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u0007H\u0086@¢\u0006\u0002\u0010\f",
   d2 = {
         "Lpkg/TestSuspendLoopFinally;",
         "",
         "<init>",
         "()V",
         "drain",
         "",
         "src",
         "Lpkg/TestSuspendLoopFinally$Source;",
         "out",
         "",
         "(Lpkg/TestSuspendLoopFinally$Source;Ljava/util/List;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "first",
         "(Lpkg/TestSuspendLoopFinally$Source;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Source"
   }
)
public final class TestSuspendLoopFinally {
   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Irreducible bytecode was duplicated to produce valid code
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public final Object drain(@NotNull TestSuspendLoopFinally.Source src, @NotNull List<Integer> out, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label271: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label271;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            Object L$1;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendLoopFinally.this.drain(null, null, (Continuation<? super Integer>)this);
            }
         };
      }

      Throwable cause;
      label265: {
         Object $result = $continuation.result;
         Object var8 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
         boolean var19;
         switch ($continuation.label) {
            case 0:
               ResultKt.throwOnFailure($result);
               cause = null;
               boolean var52 = false /* VF: Semaphore variable */;

               try {
                  var52 = true;
                  var52 = false;
                  break;
               } catch (Throwable var57) {
                  cause = var57;
                  throw var57;
               } finally {
                  if (var52) {
                     src.close(cause);
                  }
               }
            case 1:
               cause = null;
               out = (List)$continuation.L$1;
               src = (TestSuspendLoopFinally.Source)$continuation.L$0;
               boolean var41 = false /* VF: Semaphore variable */;

               Object var10000;
               try {
                  var41 = true;
                  ResultKt.throwOnFailure($result);
                  var10000 = (Integer)$result;
                  var41 = false;
               } catch (Throwable var53) {
                  cause = var53;
                  throw var53;
               } finally {
                  if (var41) {
                     src.close(cause);
                  }
               }

               var19 = false;

               try {
                  var19 = true;
                  var10000 = var10000;
                  if (var10000 == null) {
                     var19 = false;
                     break label265;
                  }

                  int e = var10000;
                  out.add(Boxing.boxInt(e));
                  var19 = false;
                  break;
               } catch (Throwable var62) {
                  cause = var62;
                  throw var62;
               } finally {
                  if (var19) {
                     src.close(cause);
                  }
               }
            default:
               throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
         }

         while (true) {
            boolean var30 = false /* VF: Semaphore variable */;

            Object var78;
            try {
               var30 = true;
               $continuation.L$0 = src;
               $continuation.L$1 = out;
               $continuation.label = 1;
               var78 = (Integer)src.next($continuation);
               var30 = false;
            } catch (Throwable e) {
               cause = e;
               throw e;
            } finally {
               if (var30) {
                  src.close(cause);
               }
            }

            if (var78 == var8) {
               return var8;
            }

            var19 = false;

            try {
               var19 = true;
               var78 = var78;
               if (var78 == null) {
                  var19 = false;
                  break;
               }

               int var63 = var78;
               out.add(Boxing.boxInt(var63));
               var19 = false;
            } catch (Throwable var59) {
               cause = var59;
               throw var59;
            } finally {
               if (var19) {
                  src.close(cause);
               }
            }
         }
      }

      src.close(cause);
      return Boxing.boxInt(out.size());
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public final Object first(@NotNull TestSuspendLoopFinally.Source src, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label125: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label125;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendLoopFinally.this.first(null, (Continuation<? super Integer>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var8 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      Throwable cause;
      Object var10000;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            cause = null;
            boolean var21 = false /* VF: Semaphore variable */;

            try {
               var21 = true;
               $continuation.L$0 = src;
               $continuation.label = 1;
               var10000 = src.next($continuation);
               var21 = false;
            } catch (Throwable e) {
               cause = e;
               throw e;
            } finally {
               if (var21) {
                  src.close(cause);
               }
            }

            if (var10000 == var8) {
               return var8;
            }
            break;
         case 1:
            cause = null;
            src = (TestSuspendLoopFinally.Source)$continuation.L$0;
            boolean var28 = false /* VF: Semaphore variable */;

            try {
               var28 = true;
               ResultKt.throwOnFailure($result);
               var10000 = $result;
               var28 = false;
               break;
            } catch (Throwable var31) {
               cause = var31;
               throw var31;
            } finally {
               if (var28) {
                  src.close(cause);
               }
            }
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      try {
         Integer v = (Integer)var10000;
         return Boxing.boxInt(v != null ? v : -1);
      } catch (Throwable var29) {
         cause = var29;
         throw var29;
      } finally {
         src.close(cause);
      }
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u001c\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u0002\n\u0000\n\u0002\u0010\u0003\bf\u0018\u00002\u00020\u0001J\u0010\u0010\u0002\u001a\u0004\u0018\u00010\u0003H¦@¢\u0006\u0002\u0010\u0004J\u0012\u0010\u0005\u001a\u00020\u00062\b\u0010\u0007\u001a\u0004\u0018\u00010\bH&",
      d2 = {"Lpkg/TestSuspendLoopFinally$Source;", "", "next", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "close", "", "cause", ""}
   )
   public interface Source {
      @Nullable
      Object next(@NotNull Continuation<? super Integer> var1);

      void close(@Nullable Throwable var1);
   }
}
