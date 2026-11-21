package pkg

import kotlin.jvm.functions.Function3
import kotlin.reflect.KFunction

public fun boundMangled(h: MangledHolder): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "h"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: new pkg/TestReferenceMangledKt$boundMangled$ref$1
   // 09: dup
   // 0a: aload 0
   // 0b: invokespecial pkg/TestReferenceMangledKt$boundMangled$ref$1.<init> (Ljava/lang/Object;)V
   // 0e: checkcast kotlin/reflect/KFunction
   // 11: astore 1
   // 12: aload 1
   // 13: checkcast kotlin/jvm/functions/Function2
   // 16: bipush 1
   // 17: invokestatic pkg/MangledTag.constructor-impl (I)I
   // 1a: invokestatic pkg/MangledTag.box-impl (I)Lpkg/MangledTag;
   // 1d: bipush 2
   // 1e: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
   // 21: invokeinterface kotlin/jvm/functions/Function2.invoke (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object; 3
   // 26: checkcast java/lang/Number
   // 29: invokevirtual java/lang/Number.intValue ()I
   // 2c: ireturn
}

public fun unboundMangled(h: MangledHolder): Int {
   return (((<unrepresentable>.INSTANCE as KFunction) as Function3)(
         h, MangledTag.box_impl/* $VF was: box-impl */(MangledTag.constructor_impl/* $VF was: constructor-impl */(3)), 4
      ) as java.lang.Number)
      .intValue()
   }
