package pkg;

import kotlin.Metadata;
import kotlin.ResultKt;
import kotlin.coroutines.Continuation;
import kotlin.coroutines.intrinsics.IntrinsicsKt;
import kotlin.coroutines.jvm.internal.ContinuationImpl;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000\u001a\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\u0018\u00002\u00020\u0001:\u0001\tB\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u0016\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u0007H\u0086@¢\u0006\u0002\u0010\b",
   d2 = {
         "Lpkg/TestSuspendTryFinally;",
         "",
         "<init>",
         "()V",
         "runThenDone",
         "",
         "step",
         "Lpkg/TestSuspendTryFinally$Step;",
         "(Lpkg/TestSuspendTryFinally$Step;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Step"
   }
)
public final class TestSuspendTryFinally {
   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public final Object runThenDone(@NotNull TestSuspendTryFinally.Step step, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label95: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label95;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendTryFinally.this.runThenDone(null, (Continuation<? super Integer>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var7 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      Object var10000;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            boolean var14 = false /* VF: Semaphore variable */;

            try {
               var14 = true;
               $continuation.L$0 = step;
               $continuation.label = 1;
               var10000 = step.run($continuation);
               var14 = false;
            } finally {
               if (var14) {
                  step.done();
               }
            }

            if (var10000 == var7) {
               return var7;
            }
            break;
         case 1:
            step = (TestSuspendTryFinally.Step)$continuation.L$0;
            boolean var18 = false /* VF: Semaphore variable */;

            try {
               var18 = true;
               ResultKt.throwOnFailure($result);
               var10000 = $result;
               var18 = false;
               break;
            } finally {
               if (var18) {
                  step.done();
               }
            }
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      try {
         return var10000;
      } finally {
         step.done();
      }
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u0016\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u0002\bf\u0018\u00002\u00020\u0001J\u000e\u0010\u0002\u001a\u00020\u0003H¦@¢\u0006\u0002\u0010\u0004J\b\u0010\u0005\u001a\u00020\u0006H&",
      d2 = {"Lpkg/TestSuspendTryFinally$Step;", "", "run", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "done", ""}
   )
   public interface Step {
      @Nullable
      Object run(@NotNull Continuation<? super Integer> var1);

      void done();
   }
}
