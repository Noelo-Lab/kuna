package pkg;

import kotlin.Metadata;
import kotlin.ResultKt;
import kotlin.Unit;
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
   d1 = "\u0000 \n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0004\u0018\u00002\u00020\u0001:\u0003\u000b\f\rB\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u001e\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u00072\u0006\u0010\b\u001a\u00020\tH\u0086@¢\u0006\u0002\u0010\n",
   d2 = {
         "Lpkg/TestKtHeadDefinitionScope;",
         "",
         "<init>",
         "()V",
         "drain",
         "",
         "sink",
         "Lpkg/TestKtHeadDefinitionScope$Sink;",
         "src",
         "Lpkg/TestKtHeadDefinitionScope$Source;",
         "(Lpkg/TestKtHeadDefinitionScope$Sink;Lpkg/TestKtHeadDefinitionScope$Source;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Cursor",
         "Source",
         "Sink"
   }
)
public final class TestKtHeadDefinitionScope {
   // $VF: Irreducible bytecode was duplicated to produce valid code
   @Nullable
   public final Object drain(
      @NotNull TestKtHeadDefinitionScope.Sink sink, @NotNull TestKtHeadDefinitionScope.Source src, @NotNull Continuation<? super Integer> $completion
   ) {
      Continuation $continuation;
      label48: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label48;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            Object L$1;
            Object L$2;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestKtHeadDefinitionScope.this.drain(null, null, (Continuation<? super Integer>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var8 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      TestKtHeadDefinitionScope.Cursor e;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            e = src.iterator();
            break;
         case 1:
            e = (TestKtHeadDefinitionScope.Cursor)$continuation.L$2;
            src = (TestKtHeadDefinitionScope.Source)$continuation.L$1;
            sink = (TestKtHeadDefinitionScope.Sink)$continuation.L$0;
            ResultKt.throwOnFailure($result);
            if (!(Boolean)$result) {
               int var9 = src.count();
               return Boxing.boxInt(var9 + 1);
            }

            int v = e.next();
            $continuation.L$0 = sink;
            $continuation.L$1 = src;
            $continuation.L$2 = e;
            $continuation.label = 2;
            if (sink.put(v, $continuation) == var8) {
               return var8;
            }
            break;
         case 2:
            e = (TestKtHeadDefinitionScope.Cursor)$continuation.L$2;
            src = (TestKtHeadDefinitionScope.Source)$continuation.L$1;
            sink = (TestKtHeadDefinitionScope.Sink)$continuation.L$0;
            ResultKt.throwOnFailure($result);
            break;
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      int v;
      do {
         $continuation.L$0 = sink;
         $continuation.L$1 = src;
         $continuation.L$2 = e;
         $continuation.label = 1;
         Object var10000 = e.hasNext($continuation);
         if (var10000 == var8) {
            return var8;
         }

         if (!(Boolean)var10000) {
            int ex = src.count();
            return Boxing.boxInt(ex + 1);
         }

         v = e.next();
         $continuation.L$0 = sink;
         $continuation.L$1 = src;
         $continuation.L$2 = e;
         $continuation.label = 2;
      } while (sink.put(v, $continuation) != var8);

      return var8;
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u0016\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\u000b\n\u0002\b\u0002\n\u0002\u0010\b\bf\u0018\u00002\u00020\u0001J\u000e\u0010\u0002\u001a\u00020\u0003H¦B¢\u0006\u0002\u0010\u0004J\t\u0010\u0005\u001a\u00020\u0006H¦\u0002",
      d2 = {"Lpkg/TestKtHeadDefinitionScope$Cursor;", "", "hasNext", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "next", ""}
   )
   public interface Cursor {
      @Nullable
      Object hasNext(@NotNull Continuation<? super Boolean> var1);

      int next();
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u0016\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0010\b\n\u0000\bf\u0018\u00002\u00020\u0001J\u0016\u0010\u0002\u001a\u00020\u00032\u0006\u0010\u0004\u001a\u00020\u0005H¦@¢\u0006\u0002\u0010\u0006",
      d2 = {"Lpkg/TestKtHeadDefinitionScope$Sink;", "", "put", "", "v", "", "(ILkotlin/coroutines/Continuation;)Ljava/lang/Object;"}
   )
   public interface Sink {
      @Nullable
      Object put(int var1, @NotNull Continuation<? super Unit> var2);
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u0014\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\b\bf\u0018\u00002\u00020\u0001J\t\u0010\u0002\u001a\u00020\u0003H¦\u0002J\b\u0010\u0004\u001a\u00020\u0005H&",
      d2 = {"Lpkg/TestKtHeadDefinitionScope$Source;", "", "iterator", "Lpkg/TestKtHeadDefinitionScope$Cursor;", "count", ""}
   )
   public interface Source {
      @NotNull
      TestKtHeadDefinitionScope.Cursor iterator();

      int count();
   }
}
