package pkg;

import kotlin.Metadata;
import kotlin.ResultKt;
import kotlin.coroutines.Continuation;
import kotlin.coroutines.intrinsics.IntrinsicsKt;
import kotlin.coroutines.jvm.internal.Boxing;
import kotlin.coroutines.jvm.internal.ContinuationImpl;
import kotlin.jvm.functions.Function3;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000&\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0004\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\u0018\u00002\u00020\u0001:\u0001\u000fB\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u0016\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u0007H\u0086@¢\u0006\u0002\u0010\bJH\u0010\t\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u00072\u0006\u0010\n\u001a\u00020\u00052(\u0010\u000b\u001a$\b\u0001\u0012\u0004\u0012\u00020\u0005\u0012\u0004\u0012\u00020\u0005\u0012\n\u0012\b\u0012\u0004\u0012\u00020\u00050\r\u0012\u0006\u0012\u0004\u0018\u00010\u00010\fH\u0086@¢\u0006\u0002\u0010\u000e",
   d2 = {
         "Lpkg/TestSuspendResultCast;",
         "",
         "<init>",
         "()V",
         "sum",
         "",
         "src",
         "Lpkg/TestSuspendResultCast$Source;",
         "(Lpkg/TestSuspendResultCast$Source;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "fold",
         "initial",
         "op",
         "Lkotlin/Function3;",
         "Lkotlin/coroutines/Continuation;",
         "(Lpkg/TestSuspendResultCast$Source;ILkotlin/jvm/functions/Function3;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Source"
   }
)
public final class TestSuspendResultCast {
   // $VF: Irreducible bytecode was duplicated to produce valid code
   @Nullable
   public final Object sum(@NotNull TestSuspendResultCast.Source src, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label39: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label39;
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
               return TestSuspendResultCast.this.sum(null, (Continuation<? super Integer>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var7 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      int total;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            total = 0;
            break;
         case 1:
            total = $continuation.I$0;
            src = (TestSuspendResultCast.Source)$continuation.L$0;
            ResultKt.throwOnFailure($result);
            Integer var10000 = (Integer)$result;
            if ((Integer)$result == null) {
               return Boxing.boxInt(total);
            }

            int v = var10000;
            total += v;
            break;
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      while (true) {
         $continuation.L$0 = src;
         $continuation.I$0 = total;
         $continuation.label = 1;
         Integer var10 = (Integer)src.next($continuation);
         if (var10 == var7) {
            return var7;
         }

         var10 = var10;
         if (var10 == null) {
            return Boxing.boxInt(total);
         }

         int var9 = var10;
         total += var9;
      }
   }

   // $VF: Irreducible bytecode was duplicated to produce valid code
   @Nullable
   public final Object fold(
      @NotNull TestSuspendResultCast.Source src,
      int initial,
      @NotNull Function3<? super Integer, ? super Integer, ? super Continuation<? super Integer>, ? extends Object> op,
      @NotNull Continuation<? super Integer> $completion
   ) {
      Continuation $continuation;
      label50: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label50;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            Object L$1;
            int I$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendResultCast.this.fold(null, 0, null, (Continuation<? super Integer>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var9 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      int acc;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            acc = initial;
            break;
         case 1:
            acc = $continuation.I$0;
            op = (Function3)$continuation.L$1;
            src = (TestSuspendResultCast.Source)$continuation.L$0;
            ResultKt.throwOnFailure($result);
            Integer var10000 = (Integer)$result;
            if ((Integer)$result == null) {
               return Boxing.boxInt(acc);
            }

            int v = var10000;
            Integer var10001 = Boxing.boxInt(acc);
            Integer var10002 = Boxing.boxInt(v);
            $continuation.L$0 = src;
            $continuation.L$1 = op;
            $continuation.label = 2;
            Object var12 = op.invoke(var10001, var10002, $continuation);
            if (var12 == var9) {
               return var9;
            }

            acc = ((Number)var12).intValue();
            break;
         case 2:
            op = (Function3)$continuation.L$1;
            src = (TestSuspendResultCast.Source)$continuation.L$0;
            ResultKt.throwOnFailure($result);
            acc = ((Number)$result).intValue();
            break;
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      while (true) {
         $continuation.L$0 = src;
         $continuation.L$1 = op;
         $continuation.I$0 = acc;
         $continuation.label = 1;
         Integer var13 = (Integer)src.next($continuation);
         if (var13 == var9) {
            return var9;
         }

         var13 = var13;
         if (var13 == null) {
            return Boxing.boxInt(acc);
         }

         int var11 = var13;
         Integer var16 = Boxing.boxInt(acc);
         Integer var17 = Boxing.boxInt(var11);
         $continuation.L$0 = src;
         $continuation.L$1 = op;
         $continuation.label = 2;
         Object var15 = op.invoke(var16, var17, $continuation);
         if (var15 == var9) {
            return var9;
         }

         acc = ((Number)var15).intValue();
      }
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u0010\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0000\bf\u0018\u00002\u00020\u0001J\u0010\u0010\u0002\u001a\u0004\u0018\u00010\u0003H¦@¢\u0006\u0002\u0010\u0004",
      d2 = {"Lpkg/TestSuspendResultCast$Source;", "", "next", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;"}
   )
   public interface Source {
      @Nullable
      Object next(@NotNull Continuation<? super Integer> var1);
   }
}
