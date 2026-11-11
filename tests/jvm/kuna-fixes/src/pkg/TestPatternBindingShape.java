package pkg;

public class TestPatternBindingShape {
   public static class Node {
      Node next;

      int size() {
         return 1;
      }
   }

   public static class Leaf extends Node {
      int weight;
   }

   Node head;
   static Node shared;
   Node[] slots;

   public int fieldCast() {
      if (head instanceof Leaf) {
         Leaf leaf = (Leaf) head;
         return leaf.weight;
      }
      return -1;
   }

   public int staticFieldCast() {
      if (shared instanceof Leaf) {
         Leaf leaf = (Leaf) shared;
         return leaf.weight;
      }
      return 0;
   }

   public int elementCast(int i) {
      if (slots[i] instanceof Leaf) {
         Leaf leaf = (Leaf) slots[i];
         return leaf.weight;
      }
      return 0;
   }

   public int chainedFieldCast(Node node) {
      if (node.next instanceof Leaf) {
         Leaf leaf = (Leaf) node.next;
         return leaf.weight + node.size();
      }
      return node.size();
   }

   public boolean negatedCast(Object o) {
      if (!(o instanceof TestPatternBindingShape)) {
         return false;
      }
      TestPatternBindingShape other = (TestPatternBindingShape) o;
      return other.head == head;
   }

   public boolean equalsFalseCast(Object o) {
      if (o instanceof TestPatternBindingShape == false) {
         return false;
      }
      TestPatternBindingShape other = (TestPatternBindingShape) o;
      return other.slots == slots;
   }

   public int fieldPattern() {
      if (head instanceof Leaf leaf) {
         return leaf.weight;
      }
      return -1;
   }

   public int negatedPattern(Node node) {
      if (!(node instanceof Leaf leaf)) {
         return -1;
      }
      return leaf.weight;
   }

   public Runnable captured(Node node) {
      return new Runnable() {
         public void run() {
            if (node instanceof Leaf leaf) {
               leaf.weight++;
            }
         }
      };
   }
}
