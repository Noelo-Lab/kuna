package pkg;

import java.util.List;
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
   d1 = "\u00002\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0010\u0002\n\u0002\b\u0002\n\u0002\u0010!\n\u0002\b\u0002\n\u0002\u0010\u0001\n\u0002\b\u0002\u0018\u00002\u00020\u0001:\u0001\u0013B\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u001e\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u00072\u0006\u0010\b\u001a\u00020\u0005H\u0086@¢\u0006\u0002\u0010\tJ,\u0010\n\u001a\u00020\u000b2\u0006\u0010\u0006\u001a\u00020\u00072\u0006\u0010\f\u001a\u00020\u00072\f\u0010\r\u001a\b\u0012\u0004\u0012\u00020\u00050\u000eH\u0086@¢\u0006\u0002\u0010\u000fJ$\u0010\u0010\u001a\u00020\u00112\u0006\u0010\u0006\u001a\u00020\u00072\f\u0010\r\u001a\b\u0012\u0004\u0012\u00020\u00050\u000eH\u0086@¢\u0006\u0002\u0010\u0012",
   d2 = {
         "Lpkg/TestSuspendLoopTry;",
         "",
         "<init>",
         "()V",
         "indexOf",
         "",
         "src",
         "Lpkg/TestSuspendLoopTry$Source;",
         "x",
         "(Lpkg/TestSuspendLoopTry$Source;ILkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "pairs",
         "",
         "other",
         "out",
         "",
         "(Lpkg/TestSuspendLoopTry$Source;Lpkg/TestSuspendLoopTry$Source;Ljava/util/List;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "watch",
         "",
         "(Lpkg/TestSuspendLoopTry$Source;Ljava/util/List;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Source"
   }
)
public final class TestSuspendLoopTry {
   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Irreducible bytecode was duplicated to produce valid code
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public final Object indexOf(@NotNull TestSuspendLoopTry.Source src, int x, @NotNull Continuation<? super Integer> $completion) {
      Continuation $continuation;
      label383: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label383;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            int I$0;
            int I$1;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendLoopTry.this.indexOf(null, 0, (Continuation<? super Integer>)this);
            }
         };
      }

      Throwable cause;
      Integer var7;
      label392: {
         label390: {
            Object $result = $continuation.result;
            Object var10 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
            int i;
            boolean var23;
            Object var10000;
            switch ($continuation.label) {
               case 0:
                  ResultKt.throwOnFailure($result);
                  cause = null;
                  i = 0;
                  boolean var62 = false /* VF: Semaphore variable */;

                  try {
                     var62 = true;
                     var62 = false;
                  } catch (Throwable var80) {
                     cause = var80;
                     throw var80;
                  } finally {
                     if (var62) {
                        src.close(cause);
                     }
                  }

                  var23 = false;

                  try {
                     var23 = true;
                     $continuation.L$0 = src;
                     $continuation.I$0 = x;
                     $continuation.I$1 = i;
                     $continuation.label = 1;
                     var10000 = src.next($continuation);
                     var23 = false;
                  } catch (Throwable var87) {
                     cause = var87;
                     throw var87;
                  } finally {
                     if (var23) {
                        src.close(cause);
                     }
                  }

                  if (var10000 == var10) {
                     return var10;
                  }
                  break;
               case 1:
                  i = $continuation.I$1;
                  x = $continuation.I$0;
                  cause = null;
                  src = (TestSuspendLoopTry.Source)$continuation.L$0;
                  boolean var75 = false /* VF: Semaphore variable */;

                  try {
                     var75 = true;
                     ResultKt.throwOnFailure($result);
                     var10000 = $result;
                     var75 = false;
                     break;
                  } catch (Throwable var76) {
                     cause = var76;
                     throw var76;
                  } finally {
                     if (var75) {
                        src.close(cause);
                     }
                  }
               default:
                  throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
            }

            do {
               boolean var49 = false /* VF: Semaphore variable */;

               try {
                  var49 = true;
                  Integer var103 = (Integer)var10000;
                  if ((Integer)var10000 == null) {
                     var49 = false;
                     break label390;
                  }

                  int v = var103;
                  if (v == x) {
                     var7 = Boxing.boxInt(i);
                     var49 = false;
                     break label392;
                  }

                  var49 = false;
               } catch (Throwable var84) {
                  cause = var84;
                  throw var84;
               } finally {
                  if (var49) {
                     src.close(cause);
                  }
               }

               boolean var36 = false /* VF: Semaphore variable */;

               try {
                  var36 = true;
                  i++;
                  var36 = false;
               } catch (Throwable var82) {
                  cause = var82;
                  throw var82;
               } finally {
                  if (var36) {
                     src.close(cause);
                  }
               }

               var23 = false;

               try {
                  var23 = true;
                  $continuation.L$0 = src;
                  $continuation.I$0 = x;
                  $continuation.I$1 = i;
                  $continuation.label = 1;
                  var10000 = src.next($continuation);
                  var23 = false;
               } catch (Throwable e) {
                  cause = e;
                  throw e;
               } finally {
                  if (var23) {
                     src.close(cause);
                  }
               }
            } while (var10000 != var10);

            return var10;
         }

         src.close(cause);
         return Boxing.boxInt(-1);
      }

      src.close(cause);
      return var7;
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Irreducible bytecode was duplicated to produce valid code
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public final Object pairs(
      @NotNull TestSuspendLoopTry.Source src,
      @NotNull TestSuspendLoopTry.Source other,
      @NotNull List<Integer> out,
      @NotNull Continuation<? super Unit> $completion
   ) {
      Continuation $continuation;
      label623: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label623;
            }
         }

         $continuation = new ContinuationImpl($completion) {
            Object L$0;
            Object L$1;
            Object L$2;
            int I$0;
            int label;

            @Nullable
            public final Object invokeSuspend(@NotNull Object $result) {
               this.result = $result;
               this.label |= -2147483648;
               return TestSuspendLoopTry.this.pairs(null, null, null, (Continuation<? super Unit>)this);
            }
         };
      }

      Throwable cause;
      label617: {
         Object $result = $continuation.result;
         Object var9 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
         boolean var28;
         boolean var47;
         switch ($continuation.label) {
            case 0:
               ResultKt.throwOnFailure($result);
               cause = null;
               boolean var123 = false /* VF: Semaphore variable */;

               try {
                  var123 = true;
                  var123 = false;
                  break;
               } catch (Throwable var128) {
                  cause = var128;
                  throw var128;
               } finally {
                  if (var123) {
                     src.close(cause);
                  }
               }
            case 1:
               cause = null;
               out = (List)$continuation.L$2;
               other = (TestSuspendLoopTry.Source)$continuation.L$1;
               src = (TestSuspendLoopTry.Source)$continuation.L$0;
               boolean var85 = false /* VF: Semaphore variable */;

               Object var170;
               try {
                  var85 = true;
                  ResultKt.throwOnFailure($result);
                  var170 = $result;
                  var85 = false;
               } catch (Throwable var124) {
                  cause = var124;
                  throw var124;
               } finally {
                  if (var85) {
                     src.close(cause);
                  }
               }

               var47 = false;

               int var142;
               try {
                  var47 = true;
                  Integer var171 = (Integer)var170;
                  if ((Integer)var170 == null) {
                     var47 = false;
                     break label617;
                  }

                  var142 = var171;
                  $continuation.L$0 = src;
                  $continuation.L$1 = other;
                  $continuation.L$2 = out;
                  $continuation.I$0 = var142;
                  $continuation.label = 2;
                  var170 = other.ready($continuation);
                  var47 = false;
               } catch (Throwable var139) {
                  cause = var139;
                  throw var139;
               } finally {
                  if (var47) {
                     src.close(cause);
                  }
               }

               if (var170 == var9) {
                  return var9;
               }

               var28 = false;

               try {
                  var28 = true;
                  if ((Boolean)var170) {
                     out.add(Boxing.boxInt(var142));
                     var28 = false;
                  } else {
                     var28 = false;
                  }
                  break;
               } catch (Throwable var141) {
                  cause = var141;
                  throw var141;
               } finally {
                  if (var28) {
                     src.close(cause);
                  }
               }
            case 2:
               int v = $continuation.I$0;
               cause = null;
               out = (List)$continuation.L$2;
               other = (TestSuspendLoopTry.Source)$continuation.L$1;
               src = (TestSuspendLoopTry.Source)$continuation.L$0;
               boolean var104 = false /* VF: Semaphore variable */;

               Object var10000;
               try {
                  var104 = true;
                  ResultKt.throwOnFailure($result);
                  var10000 = $result;
                  var104 = false;
               } catch (Throwable var132) {
                  cause = var132;
                  throw var132;
               } finally {
                  if (var104) {
                     src.close(cause);
                  }
               }

               var28 = false;

               try {
                  var28 = true;
                  if ((Boolean)var10000) {
                     out.add(Boxing.boxInt(v));
                     var28 = false;
                  } else {
                     var28 = false;
                  }
                  break;
               } catch (Throwable var137) {
                  cause = var137;
                  throw var137;
               } finally {
                  if (var28) {
                     src.close(cause);
                  }
               }
            default:
               throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
         }

         while (true) {
            boolean var66 = false /* VF: Semaphore variable */;

            Object var173;
            try {
               var66 = true;
               $continuation.L$0 = src;
               $continuation.L$1 = other;
               $continuation.L$2 = out;
               $continuation.label = 1;
               var173 = src.next($continuation);
               var66 = false;
            } catch (Throwable e) {
               cause = e;
               throw e;
            } finally {
               if (var66) {
                  src.close(cause);
               }
            }

            if (var173 == var9) {
               return var9;
            }

            var47 = false;

            int var143;
            try {
               var47 = true;
               Integer var174 = (Integer)var173;
               if ((Integer)var173 == null) {
                  var47 = false;
                  break;
               }

               var143 = var174;
               $continuation.L$0 = src;
               $continuation.L$1 = other;
               $continuation.L$2 = out;
               $continuation.I$0 = var143;
               $continuation.label = 2;
               var173 = other.ready($continuation);
               var47 = false;
            } catch (Throwable var134) {
               cause = var134;
               throw var134;
            } finally {
               if (var47) {
                  src.close(cause);
               }
            }

            if (var173 == var9) {
               return var9;
            }

            var28 = false;

            try {
               var28 = true;
               if ((Boolean)var173) {
                  out.add(Boxing.boxInt(var143));
                  var28 = false;
               } else {
                  var28 = false;
               }
            } catch (Throwable var130) {
               cause = var130;
               throw var130;
            } finally {
               if (var28) {
                  src.close(cause);
               }
            }
         }
      }

      src.close(cause);
      return Unit.INSTANCE;
   }

   // $VF: Handled exception range with multiple entry points by splitting it
   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   // $VF: Irreducible bytecode was duplicated to produce valid code
   // $VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.
   @Nullable
   public final Object watch(@NotNull TestSuspendLoopTry.Source src, @NotNull List<Integer> out, @NotNull Continuation<?> $completion) {
      Continuation $continuation;
      label1050: {
         if ($completion instanceof <unrepresentable>) {
            $continuation = (<unrepresentable>)$completion;
            if (($continuation.label & -2147483648) != 0) {
               $continuation.label -= -2147483648;
               break label1050;
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
               return TestSuspendLoopTry.this.watch(null, null, (Continuation<?>)this);
            }
         };
      }

      Object $result = $continuation.result;
      Object var8 = IntrinsicsKt.getCOROUTINE_SUSPENDED();
      int old;
      boolean var23;
      boolean var38;
      boolean var98;
      Object var10000;
      switch ($continuation.label) {
         case 0:
            ResultKt.throwOnFailure($result);
            boolean var128 = false /* VF: Semaphore variable */;

            try {
               var128 = true;
               old = -1;
               var128 = false;
            } finally {
               if (var128) {
                  src.close(null);
               }
            }

            var23 = false;

            try {
               var23 = true;
               $continuation.L$0 = src;
               $continuation.L$1 = out;
               $continuation.I$0 = old;
               $continuation.label = 1;
               var10000 = src.next($continuation);
               var23 = false;
            } finally {
               if (var23) {
                  src.close(null);
               }
            }

            if (var10000 == var8) {
               return var8;
            }
            break;
         case 1:
            old = $continuation.I$0;
            out = (List)$continuation.L$1;
            src = (TestSuspendLoopTry.Source)$continuation.L$0;
            boolean var68 = false /* VF: Semaphore variable */;

            try {
               var68 = true;
               ResultKt.throwOnFailure($result);
               var10000 = $result;
               var68 = false;
               break;
            } finally {
               if (var68) {
                  src.close(null);
               }
            }
         case 2:
            old = $continuation.I$0;
            out = (List)$continuation.L$1;
            src = (TestSuspendLoopTry.Source)$continuation.L$0;
            boolean var83 = false /* VF: Semaphore variable */;

            try {
               var83 = true;
               ResultKt.throwOnFailure($result);
               var10000 = $result;
               var83 = false;
            } finally {
               if (var83) {
                  src.close(null);
               }
            }

            var38 = false;

            label1056: {
               try {
                  var38 = true;
                  if ((Boolean)var10000) {
                     var38 = false;
                     break label1056;
                  }

                  $continuation.L$0 = src;
                  $continuation.L$1 = out;
                  $continuation.I$0 = old;
                  $continuation.label = 3;
                  var10000 = src.next($continuation);
                  var38 = false;
               } finally {
                  if (var38) {
                     src.close(null);
                  }
               }

               if (var10000 == var8) {
                  return var8;
               }

               var98 = false;

               try {
                  var98 = true;
                  var98 = false;
               } finally {
                  if (var98) {
                     src.close(null);
                  }
               }
            }

            var23 = false;

            try {
               var23 = true;
               $continuation.L$0 = src;
               $continuation.L$1 = out;
               $continuation.I$0 = old;
               $continuation.label = 1;
               var10000 = src.next($continuation);
               var23 = false;
            } finally {
               if (var23) {
                  src.close(null);
               }
            }

            if (var10000 == var8) {
               return var8;
            }
            break;
         case 3:
            old = $continuation.I$0;
            out = (List)$continuation.L$1;
            src = (TestSuspendLoopTry.Source)$continuation.L$0;
            boolean var113 = false /* VF: Semaphore variable */;

            try {
               var113 = true;
               ResultKt.throwOnFailure($result);
               var113 = false;
            } finally {
               if (var113) {
                  src.close(null);
               }
            }

            var98 = false;

            try {
               var98 = true;
               var98 = false;
            } finally {
               if (var98) {
                  src.close(null);
               }
            }

            var23 = false;

            try {
               var23 = true;
               $continuation.L$0 = src;
               $continuation.L$1 = out;
               $continuation.I$0 = old;
               $continuation.label = 1;
               var10000 = src.next($continuation);
               var23 = false;
            } finally {
               if (var23) {
                  src.close(null);
               }
            }

            if (var10000 == var8) {
               return var8;
            }
            break;
         default:
            throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
      }

      do {
         boolean var53 = false /* VF: Semaphore variable */;

         try {
            var53 = true;
            int v = (Integer)var10000 != null ? (Integer)var10000 : -1;
            if (v != old) {
               out.add(Boxing.boxInt(v));
               old = v;
            }

            $continuation.L$0 = src;
            $continuation.L$1 = out;
            $continuation.I$0 = old;
            $continuation.label = 2;
            var10000 = src.ready($continuation);
            var53 = false;
         } finally {
            if (var53) {
               src.close(null);
            }
         }

         if (var10000 == var8) {
            return var8;
         }

         var38 = false;

         label1059: {
            try {
               var38 = true;
               if ((Boolean)var10000) {
                  var38 = false;
                  break label1059;
               }

               $continuation.L$0 = src;
               $continuation.L$1 = out;
               $continuation.I$0 = old;
               $continuation.label = 3;
               var10000 = src.next($continuation);
               var38 = false;
            } finally {
               if (var38) {
                  src.close(null);
               }
            }

            if (var10000 == var8) {
               return var8;
            }

            var98 = false;

            try {
               var98 = true;
               var98 = false;
            } finally {
               if (var98) {
                  src.close(null);
               }
            }
         }

         var23 = false;

         try {
            var23 = true;
            $continuation.L$0 = src;
            $continuation.L$1 = out;
            $continuation.I$0 = old;
            $continuation.label = 1;
            var10000 = src.next($continuation);
            var23 = false;
         } finally {
            if (var23) {
               src.close(null);
            }
         }
      } while (var10000 != var8);

      return var8;
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\"\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u000b\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0010\u0003\bf\u0018\u00002\u00020\u0001J\u0010\u0010\u0002\u001a\u0004\u0018\u00010\u0003H¦@¢\u0006\u0002\u0010\u0004J\u000e\u0010\u0005\u001a\u00020\u0006H¦@¢\u0006\u0002\u0010\u0004J\u0012\u0010\u0007\u001a\u00020\b2\b\u0010\t\u001a\u0004\u0018\u00010\nH&",
      d2 = {"Lpkg/TestSuspendLoopTry$Source;", "", "next", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "ready", "", "close", "", "cause", ""}
   )
   public interface Source {
      @Nullable
      Object next(@NotNull Continuation<? super Integer> var1);

      @Nullable
      Object ready(@NotNull Continuation<? super Boolean> var1);

      void close(@Nullable Throwable var1);
   }
}
