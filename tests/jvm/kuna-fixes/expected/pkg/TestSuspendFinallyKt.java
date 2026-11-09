package pkg;

import kotlin.Metadata;
import kotlin.ResultKt;
import kotlin.Unit;
import kotlin.coroutines.Continuation;
import kotlin.coroutines.intrinsics.IntrinsicsKt;
import kotlin.coroutines.jvm.internal.Boxing;
import kotlin.coroutines.jvm.internal.ContinuationImpl;
import kotlin.jvm.functions.Function0;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 2,
   xi = 48,
   d1 = "\u0000\u001e\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0004\n\u0002\u0010\u000b\n\u0000\u001a\u000e\u0010\u0000\u001a\u00020\u0001H\u0086@¢\u0006\u0002\u0010\u0002\u001a\u001c\u0010\u0003\u001a\u00020\u00042\f\u0010\u0005\u001a\b\u0012\u0004\u0012\u00020\u00040\u0006H\u0086@¢\u0006\u0002\u0010\u0007\u001a\u001c\u0010\b\u001a\u00020\u00012\f\u0010\u0005\u001a\b\u0012\u0004\u0012\u00020\u00040\u0006H\u0086@¢\u0006\u0002\u0010\u0007\u001a$\u0010\t\u001a\u00020\u00012\u0006\u0010\n\u001a\u00020\u000b2\f\u0010\u0005\u001a\b\u0012\u0004\u0012\u00020\u00040\u0006H\u0086@¢\u0006\u0002\u0010\f",
   d2 = {
         "step",
         "",
         "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "once",
         "",
         "done",
         "Lkotlin/Function0;",
         "(Lkotlin/jvm/functions/Function0;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "twice",
         "early",
         "flag",
         "",
         "(ZLkotlin/jvm/functions/Function0;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;"
   }
)
public final class TestSuspendFinallyKt {
   @Nullable
   public static final Object step(@NotNull Continuation<? super Integer> $completion) {
      return Boxing.boxInt(1);
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public static final Object once(@NotNull Function0<Unit> done, @NotNull Continuation<? super Unit> $completion) {
      Continuation $continuation;
      label99: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label99;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendFinallyKt.once(null, (Continuation<? super Unit>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var5 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      Object var10000;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            boolean var12 = false /* VF: Semaphore variable */;

            try {
               var12 = true;
               $continuation.L$0 = done;
               $continuation.label = 1;
               var10000 = step($continuation);
               var12 = false;
            } finally {
               if (var12) {
                  done.invoke();
               }
            }

            if (var10000 == var5) {
               return var5;
            }
            break;
         case 1:
            done = (Function0)$continuation.L$0;
            boolean var16 = false /* VF: Semaphore variable */;

            try {
               var16 = true;
               ResultKt.throwOnFailure($result);
               var10000 = $result;
               var16 = false;
               break;
            } finally {
               if (var16) {
                  done.invoke();
               }
            }
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      try {
         ((Number)var10000).intValue();
      } finally {
         done.invoke();
      }

      return Unit.INSTANCE;
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public static final Object twice(@NotNull Function0<Unit> done, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label217: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label217;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            int I$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendFinallyKt.twice(null, (Continuation<? super Integer>)this);
            }
         };
      }

      int a;
      Object var10000;
      label220: {
         Object $result = $continuation.result;
         Object var7 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
         switch ($continuation.label) {
            case 0:
               ResultKt.throwOnFailure($result);
               boolean var24 = false /* VF: Semaphore variable */;

               try {
                  var24 = true;
                  $continuation.L$0 = done;
                  $continuation.label = 1;
                  var10000 = step($continuation);
                  var24 = false;
               } finally {
                  if (var24) {
                     done.invoke();
                  }
               }

               if (var10000 == var7) {
                  return var7;
               }
               break;
            case 1:
               done = (Function0)$continuation.L$0;
               boolean var30 = false /* VF: Semaphore variable */;

               try {
                  var30 = true;
                  ResultKt.throwOnFailure($result);
                  var10000 = $result;
                  var30 = false;
                  break;
               } finally {
                  if (var30) {
                     done.invoke();
                  }
               }
            case 2:
               a = $continuation.I$0;
               done = (Function0)$continuation.L$0;
               boolean var36 = false /* VF: Semaphore variable */;

               try {
                  var36 = true;
                  ResultKt.throwOnFailure($result);
                  var10000 = $result;
                  var36 = false;
                  break label220;
               } finally {
                  if (var36) {
                     done.invoke();
                  }
               }
            default:
               throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
         }

         boolean var18 = false /* VF: Semaphore variable */;

         try {
            var18 = true;
            a = ((Number)var10000).intValue();
            $continuation.L$0 = done;
            $continuation.I$0 = a;
            $continuation.label = 2;
            var10000 = step($continuation);
            var18 = false;
         } finally {
            if (var18) {
               done.invoke();
            }
         }

         if (var10000 == var7) {
            return var7;
         }
      }

      try {
         int b = ((Number)var10000).intValue();
         return Boxing.boxInt(a + b);
      } finally {
         done.invoke();
      }
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public static final Object early(boolean flag, @NotNull Function0<Unit> done, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label190: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label190;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendFinallyKt.early(false, null, (Continuation<? super Integer>)this);
            }
         };
      }

      Integer var3;
      label184: {
         Object $result = $continuation.result;
         Object var7 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
         Object var10000;
         switch ($continuation.label) {
            case 0:
               ResultKt.throwOnFailure($result);
               boolean var21 = false /* VF: Semaphore variable */;

               try {
                  var21 = true;
                  if (flag) {
                     var3 = Boxing.boxInt(0);
                     var21 = false;
                     break label184;
                  }

                  var21 = false;
               } finally {
                  if (var21) {
                     done.invoke();
                  }
               }

               boolean var16 = false /* VF: Semaphore variable */;

               try {
                  var16 = true;
                  $continuation.L$0 = done;
                  $continuation.label = 1;
                  var10000 = step($continuation);
                  var16 = false;
               } finally {
                  if (var16) {
                     done.invoke();
                  }
               }

               if (var10000 == var7) {
                  return var7;
               }
               break;
            case 1:
               done = (Function0)$continuation.L$0;
               boolean var26 = false /* VF: Semaphore variable */;

               try {
                  var26 = true;
                  ResultKt.throwOnFailure($result);
                  var10000 = $result;
                  var26 = false;
                  break;
               } finally {
                  if (var26) {
                     done.invoke();
                  }
               }
            default:
               throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
         }

         try {
            return var10000;
         } finally {
            done.invoke();
         }
      }

      done.invoke();
      return var3;
   }
}
