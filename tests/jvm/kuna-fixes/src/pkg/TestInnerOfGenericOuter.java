package pkg;

public class TestInnerOfGenericOuter<E> {
   E value;

   static class Ref<T> {
      private T referent;

      Ref(T referent) {
         this.referent = referent;
      }

      T get() {
         return this.referent;
      }
   }

   class Itr {
      E last;

      boolean isDetached() {
         return this.last == null;
      }
   }

   class Itrs {
      Node head;

      class Node extends Ref<Itr> {
         Node next;

         E item() {
            return TestInnerOfGenericOuter.this.value;
         }

         Node(Itr it, Node next) {
            super(it);
            this.next = next;
         }
      }

      void sweep() {
         Node p = this.head;
         Itr it = p.get();
         if (it == null || it.isDetached()) {
            this.head = p.next;
         }
      }

      E lastOfHead() {
         return this.head.get().last;
      }
   }

   static String firstOf(TestInnerOfGenericOuter<String>.Itrs itrs) {
      return itrs.head.get().last;
   }

   static int itemLength(TestInnerOfGenericOuter<String>.Itrs.Node node) {
      return node.item().length();
   }

   static int lengthOf(TestInnerOfGenericOuter<String>.Itrs.Node node) {
      TestInnerOfGenericOuter<String>.Itr it = node.get();
      return it.last.length();
   }
}
