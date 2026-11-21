package pkg;

public class TestPatternBindingShape {
   TestPatternBindingShape.Node head;
   static TestPatternBindingShape.Node shared;
   TestPatternBindingShape.Node[] slots;

   public int fieldCast() {
      if (this.head instanceof TestPatternBindingShape.Leaf) {
         TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)this.head;
         return leaf.weight;
      }

      return -1;
   }

   public int staticFieldCast() {
      if (shared instanceof TestPatternBindingShape.Leaf) {
         TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)shared;
         return leaf.weight;
      }

      return 0;
   }

   public int elementCast(int i) {
      if (this.slots[i] instanceof TestPatternBindingShape.Leaf) {
         TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)this.slots[i];
         return leaf.weight;
      }

      return 0;
   }

   public int chainedFieldCast(TestPatternBindingShape.Node node) {
      if (node.next instanceof TestPatternBindingShape.Leaf) {
         TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)node.next;
         return leaf.weight + node.size();
      }

      return node.size();
   }

   public boolean negatedCast(Object o) {
      if (!(o instanceof TestPatternBindingShape)) {
         return false;
      }

      TestPatternBindingShape other = (TestPatternBindingShape)o;
      return other.head == this.head;
   }

   public boolean equalsFalseCast(Object o) {
      if (!(o instanceof TestPatternBindingShape)) {
         return false;
      }

      TestPatternBindingShape other = (TestPatternBindingShape)o;
      return other.slots == this.slots;
   }

   public int fieldPattern() {
      if (this.head instanceof TestPatternBindingShape.Leaf leaf) {
         return leaf.weight;
      }

      return -1;
   }

   public int negatedPattern(TestPatternBindingShape.Node node) {
      if (!(node instanceof TestPatternBindingShape.Leaf leaf)) {
         return -1;
      }

      return leaf.weight;
   }

   public Runnable captured(final TestPatternBindingShape.Node node) {
      return new Runnable() {
         public void run() {
            if (node instanceof TestPatternBindingShape.Leaf leaf) {
               leaf.weight++;
            }
         }
      };
   }

   public static class Leaf extends TestPatternBindingShape.Node {
      int weight;
   }

   public static class Node {
      TestPatternBindingShape.Node next;

      int size() {
         return 1;
      }
   }
}
