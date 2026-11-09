package pkg

import kotlin.jvm.functions.Function1
import kotlin.jvm.functions.Function3
import kotlin.reflect.KFunction

public fun twice(x: Int): Int {
   return x * 2
}

public fun bound(adder: Adder): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "adder"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: new pkg/TestReferenceClassesKt$bound$ref$1
   // 09: dup
   // 0a: aload 0
   // 0b: invokespecial pkg/TestReferenceClassesKt$bound$ref$1.<init> (Ljava/lang/Object;)V
   // 0e: checkcast kotlin/reflect/KFunction
   // 11: astore 1
   // 12: aload 1
   // 13: checkcast kotlin/jvm/functions/Function2
   // 16: bipush 1
   // 17: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
   // 1a: bipush 2
   // 1b: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
   // 1e: invokeinterface kotlin/jvm/functions/Function2.invoke (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object; 3
   // 23: checkcast java/lang/Number
   // 26: invokevirtual java/lang/Number.intValue ()I
   // 29: ireturn
}

public fun unbound(adder: Adder): Int {
   return (((<unrepresentable>.INSTANCE as KFunction) as Function3)(adder, 1, 2) as java.lang.Number).intValue()
}

public fun constructor(): Int {
   return (((<unrepresentable>.INSTANCE as KFunction) as Function1)(3) as Adder).getK()
}

public fun topLevel(x: Int): Int {
   return (((<unrepresentable>.INSTANCE as KFunction) as Function1)(x) as java.lang.Number).intValue()
}
