package pkg;

public class TestInnerOfGenericOuter<E> {
   E value;

   static String firstOf(TestInnerOfGenericOuter<String>.Itrs itrs) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: getfield pkg/TestInnerOfGenericOuter$Itrs.head Lpkg/TestInnerOfGenericOuter$Itrs$Node;
      // 04: invokevirtual pkg/TestInnerOfGenericOuter$Itrs$Node.get ()Ljava/lang/Object;
      // 07: checkcast pkg/TestInnerOfGenericOuter$Itr
      // 0a: getfield pkg/TestInnerOfGenericOuter$Itr.last Ljava/lang/Object;
      // 0d: checkcast java/lang/String
      // 10: areturn
   }

   static int itemLength(TestInnerOfGenericOuter<String>.Itrs.Node node) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: aload 0
      // 1: invokevirtual pkg/TestInnerOfGenericOuter$Itrs$Node.item ()Ljava/lang/Object;
      // 4: checkcast java/lang/String
      // 7: invokevirtual java/lang/String.length ()I
      // a: ireturn
   }

   static int lengthOf(TestInnerOfGenericOuter<String>.Itrs.Node node) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: invokevirtual pkg/TestInnerOfGenericOuter$Itrs$Node.get ()Ljava/lang/Object;
      // 04: checkcast pkg/TestInnerOfGenericOuter$Itr
      // 07: astore 1
      // 08: aload 1
      // 09: getfield pkg/TestInnerOfGenericOuter$Itr.last Ljava/lang/Object;
      // 0c: checkcast java/lang/String
      // 0f: invokevirtual java/lang/String.length ()I
      // 12: ireturn
   }

   class Itr {
      E last;

      boolean isDetached() {
         return this.last == null;
      }
   }

   class Itrs {
      TestInnerOfGenericOuter<E>.Itrs.Node head;

      void sweep() {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 00: aload 0
         // 01: getfield pkg/TestInnerOfGenericOuter$Itrs.head Lpkg/TestInnerOfGenericOuter$Itrs$Node;
         // 04: astore 1
         // 05: aload 1
         // 06: invokevirtual pkg/TestInnerOfGenericOuter$Itrs$Node.get ()Ljava/lang/Object;
         // 09: checkcast pkg/TestInnerOfGenericOuter$Itr
         // 0c: astore 2
         // 0d: aload 2
         // 0e: ifnull 18
         // 11: aload 2
         // 12: invokevirtual pkg/TestInnerOfGenericOuter$Itr.isDetached ()Z
         // 15: ifeq 20
         // 18: aload 0
         // 19: aload 1
         // 1a: getfield pkg/TestInnerOfGenericOuter$Itrs$Node.next Lpkg/TestInnerOfGenericOuter$Itrs$Node;
         // 1d: putfield pkg/TestInnerOfGenericOuter$Itrs.head Lpkg/TestInnerOfGenericOuter$Itrs$Node;
         // 20: return
      }

      E lastOfHead() {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 0: aload 0
         // 1: getfield pkg/TestInnerOfGenericOuter$Itrs.head Lpkg/TestInnerOfGenericOuter$Itrs$Node;
         // 4: invokevirtual pkg/TestInnerOfGenericOuter$Itrs$Node.get ()Ljava/lang/Object;
         // 7: checkcast pkg/TestInnerOfGenericOuter$Itr
         // a: getfield pkg/TestInnerOfGenericOuter$Itr.last Ljava/lang/Object;
         // d: areturn
      }

      class Node extends TestInnerOfGenericOuter.Ref<TestInnerOfGenericOuter<E>.Itr> {
         TestInnerOfGenericOuter<E>.Itrs.Node next;

         E item() {
            return TestInnerOfGenericOuter.this.value;
         }

         Node(TestInnerOfGenericOuter<E>.Itr it, TestInnerOfGenericOuter<E>.Itrs.Node next) {
            super(it);
            this.next = next;
         }
      }
   }

   static class Ref<T> {
      private T referent;

      Ref(T referent) {
         this.referent = referent;
      }

      T get() {
         return this.referent;
      }
   }
}
