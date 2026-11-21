package pkg;

public class TestRecordPatternInTry {
   TestRecordPatternInTry.Shape load(String key) throws TestRecordPatternInTry.Missing {
      return null;
   }

   public Object describe(String param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: aload 1
      // 02: invokevirtual pkg/TestRecordPatternInTry.load (Ljava/lang/String;)Lpkg/TestRecordPatternInTry$Shape;
      // 05: astore 2
      // 06: aload 2
      // 07: instanceof pkg/TestRecordPatternInTry$Circle
      // 0a: ifeq 24
      // 0d: aload 2
      // 0e: checkcast pkg/TestRecordPatternInTry$Circle
      // 11: astore 3
      // 12: aload 3
      // 13: invokevirtual pkg/TestRecordPatternInTry$Circle.radius ()I
      // 16: istore 5
      // 18: iload 5
      // 1a: istore 4
      // 1c: iload 4
      // 1e: bipush 2
      // 1f: imul
      // 20: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
      // 23: areturn
      // 24: aload 2
      // 25: instanceof pkg/TestRecordPatternInTry$Label
      // 28: ifeq 41
      // 2b: aload 2
      // 2c: checkcast pkg/TestRecordPatternInTry$Label
      // 2f: astore 6
      // 31: aload 6
      // 33: invokevirtual pkg/TestRecordPatternInTry$Label.text ()Ljava/lang/String;
      // 36: astore 7
      // 38: aload 7
      // 3a: astore 5
      // 3c: aload 5
      // 3e: goto 42
      // 41: aconst_null
      // 42: areturn
      // 43: astore 2
      // 44: new java/lang/MatchException
      // 47: dup
      // 48: aload 2
      // 49: invokevirtual java/lang/Throwable.toString ()Ljava/lang/String;
      // 4c: aload 2
      // 4d: invokespecial java/lang/MatchException.<init> (Ljava/lang/String;Ljava/lang/Throwable;)V
      // 50: athrow
      // 51: astore 2
      // 52: aload 2
      // 53: areturn
      // try (11 -> 12): 35 java/lang/Throwable
      // try (27 -> 28): 35 java/lang/Throwable
      // try (0 -> 19): 43 pkg/TestRecordPatternInTry$Missing
      // try (20 -> 34): 43 pkg/TestRecordPatternInTry$Missing
      // try (35 -> 43): 43 pkg/TestRecordPatternInTry$Missing
   }

   public int measure(String key) {
      int size = 0;

      try {
         TestRecordPatternInTry.Shape s = this.load(key);
         if (s instanceof TestRecordPatternInTry.Circle(int r)) {
            size = r;
         } else if (s instanceof TestRecordPatternInTry.Label(String t)) {
            size = t.length();
         }
      } catch (TestRecordPatternInTry.Missing e) {
         size = -1;
      }

      return size;
   }

   record Circle(int radius) implements TestRecordPatternInTry.Shape {
   }

   record Label(String text) implements TestRecordPatternInTry.Shape {
   }

   static class Missing extends Exception {
   }

   sealed interface Shape permits TestRecordPatternInTry.Circle, TestRecordPatternInTry.Label {
   }
}
