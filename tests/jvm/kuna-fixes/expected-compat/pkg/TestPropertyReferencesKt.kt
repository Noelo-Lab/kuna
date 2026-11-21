package pkg

import kotlin.reflect.KMutableProperty1
import kotlin.reflect.KProperty0
import kotlin.reflect.KProperty1

public final val answer: Int = 42

public final val doubled: Int
   public final get() {
      return `$this$doubled`.getSize() * 2
   }


public fun bound(box: Box): String {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "box"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: new pkg/TestPropertyReferencesKt$bound$1
   // 09: dup
   // 0a: aload 0
   // 0b: invokespecial pkg/TestPropertyReferencesKt$bound$1.<init> (Ljava/lang/Object;)V
   // 0e: invokestatic java/lang/String.valueOf (Ljava/lang/Object;)Ljava/lang/String;
   // 11: areturn
}

public fun boundMutable(box: Box): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "box"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: new pkg/TestPropertyReferencesKt$boundMutable$ref$1
   // 09: dup
   // 0a: aload 0
   // 0b: invokespecial pkg/TestPropertyReferencesKt$boundMutable$ref$1.<init> (Ljava/lang/Object;)V
   // 0e: checkcast kotlin/reflect/KMutableProperty0
   // 11: astore 1
   // 12: aload 1
   // 13: invokeinterface kotlin/reflect/KMutableProperty0.get ()Ljava/lang/Object; 1
   // 18: checkcast java/lang/Number
   // 1b: invokevirtual java/lang/Number.intValue ()I
   // 1e: ireturn
}

public fun member(box: Box): Int {
   return ((<unrepresentable>.INSTANCE as KProperty1).get(box) as java.lang.Number).intValue()
}

public fun mutableMember(box: Box) {
   (<unrepresentable>.INSTANCE as KMutableProperty1).set(box, 1)
}

public fun topLevel(): Int {
   return ((<unrepresentable>.INSTANCE as KProperty0).get() as java.lang.Number).intValue()
}

public fun extension(box: Box): Int {
   return ((<unrepresentable>.INSTANCE as KProperty1).get(box) as java.lang.Number).intValue()
}
