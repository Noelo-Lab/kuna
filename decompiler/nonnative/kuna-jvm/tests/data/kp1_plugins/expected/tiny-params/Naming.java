package p;

import java.io.File;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.function.Function;

public abstract class Naming {
   public Naming() {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: aload 0
      // 1: invokespecial java/lang/Object.<init> ()V
      // 4: return
   }

   public static int st(long l, double d, int i, String string, File file, Class<?> class_) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 4
      // 02: bipush 2
      // 03: imul
      // 04: istore 8
      // 06: lload 0
      // 07: lstore 9
      // 09: dload 2
      // 0a: dconst_0
      // 0b: dcmpl
      // 0c: ifle 13
      // 0f: bipush 1
      // 10: goto 14
      // 13: bipush 0
      // 14: istore 11
      // 16: new java/lang/StringBuilder
      // 19: dup
      // 1a: invokespecial java/lang/StringBuilder.<init> ()V
      // 1d: aload 5
      // 1f: invokevirtual java/lang/StringBuilder.append (Ljava/lang/String;)Ljava/lang/StringBuilder;
      // 22: iload 8
      // 24: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 27: invokevirtual java/lang/StringBuilder.toString ()Ljava/lang/String;
      // 2a: astore 12
      // 2c: new java/io/File
      // 2f: dup
      // 30: aload 12
      // 32: invokespecial java/io/File.<init> (Ljava/lang/String;)V
      // 35: astore 13
      // 37: iload 8
      // 39: lload 9
      // 3b: l2i
      // 3c: iadd
      // 3d: iload 11
      // 3f: ifeq 46
      // 42: bipush 1
      // 43: goto 47
      // 46: bipush 0
      // 47: iadd
      // 48: aload 12
      // 4a: invokevirtual java/lang/String.length ()I
      // 4d: iadd
      // 4e: aload 6
      // 50: invokevirtual java/io/File.getName ()Ljava/lang/String;
      // 53: invokevirtual java/lang/String.length ()I
      // 56: iadd
      // 57: aload 7
      // 59: invokevirtual java/lang/Object.hashCode ()I
      // 5c: iadd
      // 5d: aload 13
      // 5f: invokevirtual java/io/File.hashCode ()I
      // 62: iadd
      // 63: ireturn
   }

   public void inst(int[] i, String[][] strings, Object... object) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: bipush 0
      // 001: istore 4
      // 003: iload 4
      // 005: aload 1
      // 006: arraylength
      // 007: if_icmpge 07f
      // 00a: bipush 0
      // 00b: istore 5
      // 00d: iload 5
      // 00f: aload 2
      // 010: arraylength
      // 011: if_icmpge 079
      // 014: bipush 0
      // 015: istore 6
      // 017: iload 6
      // 019: aload 3
      // 01a: arraylength
      // 01b: if_icmpge 073
      // 01e: bipush 0
      // 01f: istore 7
      // 021: iload 7
      // 023: bipush 2
      // 024: if_icmpge 06d
      // 027: bipush 0
      // 028: istore 8
      // 02a: iload 8
      // 02c: bipush 2
      // 02d: if_icmpge 067
      // 030: getstatic java/lang/System.out Ljava/io/PrintStream;
      // 033: new java/lang/StringBuilder
      // 036: dup
      // 037: invokespecial java/lang/StringBuilder.<init> ()V
      // 03a: aload 1
      // 03b: iload 4
      // 03d: iaload
      // 03e: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 041: aload 2
      // 042: iload 5
      // 044: aaload
      // 045: bipush 0
      // 046: aaload
      // 047: invokevirtual java/lang/StringBuilder.append (Ljava/lang/String;)Ljava/lang/StringBuilder;
      // 04a: aload 3
      // 04b: iload 6
      // 04d: aaload
      // 04e: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 051: iload 7
      // 053: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 056: iload 8
      // 058: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 05b: invokevirtual java/lang/StringBuilder.toString ()Ljava/lang/String;
      // 05e: invokevirtual java/io/PrintStream.println (Ljava/lang/String;)V
      // 061: iinc 8 1
      // 064: goto 02a
      // 067: iinc 7 1
      // 06a: goto 021
      // 06d: iinc 6 1
      // 070: goto 017
      // 073: iinc 5 1
      // 076: goto 00d
      // 079: iinc 4 1
      // 07c: goto 003
      // 07f: new java/util/ArrayList
      // 082: dup
      // 083: invokespecial java/util/ArrayList.<init> ()V
      // 086: astore 4
      // 088: aconst_null
      // 089: astore 5
      // 08b: new java/util/HashMap
      // 08e: dup
      // 08f: invokespecial java/util/HashMap.<init> ()V
      // 092: astore 6
      // 094: aload 4
      // 096: new java/lang/StringBuilder
      // 099: dup
      // 09a: invokespecial java/lang/StringBuilder.<init> ()V
      // 09d: aload 5
      // 09f: invokestatic java/lang/String.valueOf (Ljava/lang/Object;)Ljava/lang/String;
      // 0a2: invokevirtual java/lang/StringBuilder.append (Ljava/lang/String;)Ljava/lang/StringBuilder;
      // 0a5: aload 6
      // 0a7: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 0aa: invokevirtual java/lang/StringBuilder.toString ()Ljava/lang/String;
      // 0ad: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
      // 0b2: pop
      // 0b3: bipush 99
      // 0b5: istore 7
      // 0b7: bipush 1
      // 0b8: istore 8
      // 0ba: bipush 2
      // 0bb: istore 9
      // 0bd: bipush 3
      // 0be: istore 10
      // 0c0: ldc 3.0
      // 0c2: fstore 11
      // 0c4: ldc2_w 4.0
      // 0c7: dstore 12
      // 0c9: iload 7
      // 0cb: invokestatic java/lang/Character.valueOf (C)Ljava/lang/Character;
      // 0ce: astore 14
      // 0d0: ldc2_w 4
      // 0d3: invokestatic java/lang/Long.valueOf (J)Ljava/lang/Long;
      // 0d6: astore 15
      // 0d8: iload 8
      // 0da: invokestatic java/lang/Byte.valueOf (B)Ljava/lang/Byte;
      // 0dd: astore 16
      // 0df: iload 9
      // 0e1: invokestatic java/lang/Short.valueOf (S)Ljava/lang/Short;
      // 0e4: astore 17
      // 0e6: bipush 1
      // 0e7: invokestatic java/lang/Boolean.valueOf (Z)Ljava/lang/Boolean;
      // 0ea: astore 18
      // 0ec: ldc p/Naming
      // 0ee: invokevirtual java/lang/Class.getPackage ()Ljava/lang/Package;
      // 0f1: astore 19
      // 0f3: aconst_null
      // 0f4: astore 20
      // 0f6: ldc p/Naming
      // 0f8: astore 21
      // 0fa: bipush 2
      // 0fb: anewarray 34
      // 0fe: astore 22
      // 100: bipush 2
      // 101: bipush 2
      // 102: multianewarray 35 2
      // 106: astore 23
      // 108: new java/lang/StringBuilder
      // 10b: dup
      // 10c: invokespecial java/lang/StringBuilder.<init> ()V
      // 10f: astore 24
      // 111: getstatic java/lang/System.out Ljava/io/PrintStream;
      // 114: aload 24
      // 116: iload 7
      // 118: invokevirtual java/lang/StringBuilder.append (C)Ljava/lang/StringBuilder;
      // 11b: iload 8
      // 11d: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 120: iload 9
      // 122: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 125: iload 10
      // 127: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 12a: fload 11
      // 12c: invokevirtual java/lang/StringBuilder.append (F)Ljava/lang/StringBuilder;
      // 12f: dload 12
      // 131: invokevirtual java/lang/StringBuilder.append (D)Ljava/lang/StringBuilder;
      // 134: aload 14
      // 136: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 139: aload 15
      // 13b: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 13e: aload 16
      // 140: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 143: aload 17
      // 145: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 148: aload 18
      // 14a: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 14d: aload 19
      // 14f: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 152: aload 20
      // 154: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 157: aload 21
      // 159: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 15c: aload 22
      // 15e: arraylength
      // 15f: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 162: aload 23
      // 164: arraylength
      // 165: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 168: invokevirtual java/io/PrintStream.println (Ljava/lang/Object;)V
      // 16b: return
   }

   // $VF: Couldn't set local class definition as local class statement was null
   public void lambdas(String string, int i) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 2
      // 01: aload 1
      // 02: invokedynamic run (ILjava/lang/String;)Ljava/lang/Runnable; bsm=java/lang/invoke/LambdaMetafactory.metafactory (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite; args=[ ()V, p/Naming.lambda$lambdas$0 (ILjava/lang/String;)V, ()V ]
      // 07: astore 3
      // 08: iload 2
      // 09: invokedynamic apply (I)Ljava/util/function/Function; bsm=java/lang/invoke/LambdaMetafactory.metafactory (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite; args=[ (Ljava/lang/Object;)Ljava/lang/Object;, p/Naming.lambda$lambdas$1 (ILjava/lang/String;)Ljava/lang/Integer;, (Ljava/lang/String;)Ljava/lang/Integer; ]
      // 0e: astore 4
      // 10: aload 3
      // 11: invokeinterface java/lang/Runnable.run ()V 1
      // 16: getstatic java/lang/System.out Ljava/io/PrintStream;
      // 19: aload 4
      // 1b: aload 1
      // 1c: invokeinterface java/util/function/Function.apply (Ljava/lang/Object;)Ljava/lang/Object; 2
      // 21: invokevirtual java/io/PrintStream.println (Ljava/lang/Object;)V
      // 24: new p/Naming$1
      // 27: dup
      // 28: aload 0
      // 29: iload 2
      // 2a: aload 1
      // 2b: invokespecial p/Naming$1.<init> (Lp/Naming;ILjava/lang/String;)V
      // 2e: astore 5
      // 30: getstatic java/lang/System.out Ljava/io/PrintStream;
      // 33: new java/lang/StringBuilder
      // 36: dup
      // 37: invokespecial java/lang/StringBuilder.<init> ()V
      // 3a: aload 5
      // 3c: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 3f: ldc ""
      // 41: invokevirtual java/lang/StringBuilder.append (Ljava/lang/String;)Ljava/lang/StringBuilder;
      // 44: new p/Naming$1Local
      // 47: dup
      // 48: aload 0
      // 49: iload 2
      // 4a: invokespecial p/Naming$1Local.<init> (Lp/Naming;I)V
      // 4d: bipush 3
      // 4e: invokevirtual p/Naming$1Local.f (I)I
      // 51: invokevirtual java/lang/StringBuilder.append (I)Ljava/lang/StringBuilder;
      // 54: invokevirtual java/lang/StringBuilder.toString ()Ljava/lang/String;
      // 57: invokevirtual java/io/PrintStream.println (Ljava/lang/String;)V
      // 5a: return
   }

   public abstract void abs(String string, int i);

   public native void nat(long l, Object object);

   public static void keywords(Class<?> class_, Package package_) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: astore 2
      // 02: aload 0
      // 03: astore 3
      // 04: aload 1
      // 05: astore 4
      // 07: aconst_null
      // 08: astore 5
      // 0a: getstatic java/lang/System.out Ljava/io/PrintStream;
      // 0d: new java/lang/StringBuilder
      // 10: dup
      // 11: invokespecial java/lang/StringBuilder.<init> ()V
      // 14: aload 2
      // 15: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 18: ldc ""
      // 1a: invokevirtual java/lang/StringBuilder.append (Ljava/lang/String;)Ljava/lang/StringBuilder;
      // 1d: aload 3
      // 1e: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 21: aload 4
      // 23: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 26: aload 5
      // 28: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
      // 2b: invokevirtual java/lang/StringBuilder.toString ()Ljava/lang/String;
      // 2e: invokevirtual java/io/PrintStream.println (Ljava/lang/String;)V
      // 31: return
   }

   public static <T extends Comparable<T>> T generic(T t, T t1, List<? extends T> list) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: aload 1
      // 02: invokeinterface java/lang/Comparable.compareTo (Ljava/lang/Object;)I 2
      // 07: ifle 0e
      // 0a: aload 0
      // 0b: goto 0f
      // 0e: aload 1
      // 0f: astore 3
      // 10: aload 2
      // 11: invokeinterface java/util/List.iterator ()Ljava/util/Iterator; 1
      // 16: astore 4
      // 18: aload 4
      // 1a: invokeinterface java/util/Iterator.hasNext ()Z 1
      // 1f: ifeq 3f
      // 22: aload 4
      // 24: invokeinterface java/util/Iterator.next ()Ljava/lang/Object; 1
      // 29: checkcast java/lang/Comparable
      // 2c: astore 5
      // 2e: aload 5
      // 30: aload 3
      // 31: invokeinterface java/lang/Comparable.compareTo (Ljava/lang/Object;)I 2
      // 36: ifle 3c
      // 39: aload 5
      // 3b: astore 3
      // 3c: goto 18
      // 3f: aload 3
      // 40: areturn
   }

   public class Inner {
      public Inner(Naming naming) {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 0: aload 0
         // 1: aload 1
         // 2: putfield p/Naming$Inner.this$0 Lp/Naming;
         // 5: aload 0
         // 6: invokespecial java/lang/Object.<init> ()V
         // 9: return
      }

      void m(Naming.Inner inner, Naming naming) {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 00: aload 0
         // 01: astore 3
         // 02: aconst_null
         // 03: astore 4
         // 05: getstatic java/lang/System.out Ljava/io/PrintStream;
         // 08: new java/lang/StringBuilder
         // 0b: dup
         // 0c: invokespecial java/lang/StringBuilder.<init> ()V
         // 0f: aload 3
         // 10: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
         // 13: ldc ""
         // 15: invokevirtual java/lang/StringBuilder.append (Ljava/lang/String;)Ljava/lang/StringBuilder;
         // 18: aload 1
         // 19: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
         // 1c: aload 2
         // 1d: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
         // 20: aload 4
         // 22: invokevirtual java/lang/StringBuilder.append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
         // 25: invokevirtual java/lang/StringBuilder.toString ()Ljava/lang/String;
         // 28: invokevirtual java/io/PrintStream.println (Ljava/lang/String;)V
         // 2b: return
      }
   }
}
