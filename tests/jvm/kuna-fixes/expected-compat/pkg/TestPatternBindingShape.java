package pkg;

public class TestPatternBindingShape {
   TestPatternBindingShape.Node head;
   static TestPatternBindingShape.Node shared;
   TestPatternBindingShape.Node[] slots;

   public int fieldCast() {
      return this.head instanceof TestPatternBindingShape.Leaf leaf ? leaf.weight : -1;
   }

   public int staticFieldCast() {
      return shared instanceof TestPatternBindingShape.Leaf leaf ? leaf.weight : 0;
   }

   public int elementCast(int i) {
      return this.slots[i] instanceof TestPatternBindingShape.Leaf leaf ? leaf.weight : 0;
   }

   public int chainedFieldCast(TestPatternBindingShape.Node node) {
      return node.next instanceof TestPatternBindingShape.Leaf leaf ? leaf.weight + node.size() : node.size();
   }

   public boolean negatedCast(Object o) {
      return !(o instanceof TestPatternBindingShape other) ? false : other.head == this.head;
   }

   public boolean equalsFalseCast(Object o) {
      return !(o instanceof TestPatternBindingShape other) ? false : other.slots == this.slots;
   }

   public int fieldPattern() {
      return this.head instanceof TestPatternBindingShape.Leaf leaf ? leaf.weight : -1;
   }

   public int negatedPattern(TestPatternBindingShape.Node node) {
      return node instanceof TestPatternBindingShape.Leaf leaf ? leaf.weight : -1;
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
