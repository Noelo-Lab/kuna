package pkg;

public class TestInnerOfGenericOuter<E> {
   E value;

   static String firstOf(TestInnerOfGenericOuter<String>.Itrs itrs) {
      return (String)itrs.head.get().last;
   }

   static int itemLength(TestInnerOfGenericOuter<String>.Itrs.Node node) {
      return node.item().length();
   }

   static int lengthOf(TestInnerOfGenericOuter<String>.Itrs.Node node) {
      TestInnerOfGenericOuter<String>.Itr it = node.get();
      return ((String)it.last).length();
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
         TestInnerOfGenericOuter<E>.Itrs.Node p = this.head;
         TestInnerOfGenericOuter<E>.Itr it = p.get();
         if (it == null || it.isDetached()) {
            this.head = p.next;
         }
      }

      E lastOfHead() {
         return this.head.get().last;
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
