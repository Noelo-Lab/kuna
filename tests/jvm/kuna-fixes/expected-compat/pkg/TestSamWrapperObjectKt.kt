@file:SourceDebugExtension(["SMAP\nTestSamWrapperObject.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSamWrapperObject.kt\npkg/TestSamWrapperObjectKt\n*L\n1#1,21:1\n4#1:22\n*S KotlinDebug\n*F\n+ 1 TestSamWrapperObject.kt\npkg/TestSamWrapperObjectKt\n*L\n6#1:22\n*E\n"])

package pkg

import java.util.Comparator
import kotlin.jvm.internal.SourceDebugExtension

public inline fun <T> byLength(crossinline f: (T) -> String): Comparator<T> {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "f"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: bipush 0
   // 07: istore 1
   // 08: new pkg/TestSamWrapperObjectKt$byLength$1
   // 0b: dup
   // 0c: aload 0
   // 0d: invokespecial pkg/TestSamWrapperObjectKt$byLength$1.<init> (Lkotlin/jvm/functions/Function1;)V
   // 10: checkcast java/util/Comparator
   // 13: areturn
}

public inline fun <T> byLengthOf(crossinline f: (T) -> String): Comparator<T> {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "f"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: bipush 0
   // 07: istore 1
   // 08: bipush 0
   // 09: istore 2
   // 0a: new pkg/TestSamWrapperObjectKt$byLength$1
   // 0d: dup
   // 0e: aload 0
   // 0f: invokespecial pkg/TestSamWrapperObjectKt$byLength$1.<init> (Lkotlin/jvm/functions/Function1;)V
   // 12: checkcast java/util/Comparator
   // 15: areturn
}

public inline fun <T> nullsFirstBy(crossinline f: (T) -> String?): Comparator<T> {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "f"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: bipush 0
   // 07: istore 1
   // 08: new pkg/TestSamWrapperObjectKt$nullsFirstBy$1
   // 0b: dup
   // 0c: aload 0
   // 0d: invokespecial pkg/TestSamWrapperObjectKt$nullsFirstBy$1.<init> (Lkotlin/jvm/functions/Function1;)V
   // 10: checkcast java/util/Comparator
   // 13: areturn
}

public inline fun twice(crossinline g: (Int) -> Int): (Int) -> Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "g"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: bipush 0
   // 07: istore 1
   // 08: new pkg/TestSamWrapperObjectKt$twice$1
   // 0b: dup
   // 0c: aload 0
   // 0d: invokespecial pkg/TestSamWrapperObjectKt$twice$1.<init> (Lkotlin/jvm/functions/Function1;)V
   // 10: checkcast kotlin/jvm/functions/Function1
   // 13: areturn
}
