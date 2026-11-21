package pkg;

public class TestPatternSubjectLocal {
   public static class Entry {
      Entry parent;

      Entry lookup(String key) {
         return parent;
      }
   }

   public static class Group extends Entry {
      int count;
   }

   Entry root;

   public int namedLocal(String key) {
      Entry entry = root.lookup(key);
      if (entry instanceof Group group) {
         return group.count;
      }
      return 0;
   }

   public int namedLocalNegated(String key) {
      Entry entry = root.lookup(key);
      if (!(entry instanceof Group group)) {
         return -1;
      }
      return group.count;
   }

   public int namedLocalInLoop(String[] keys) {
      int total = 0;
      for (String key : keys) {
         Entry entry = root.lookup(key);
         if (entry instanceof Group group && group.count > 0) {
            total += group.count;
         }
      }
      return total;
   }

   public int subjectExpression(String key) {
      if (root.lookup(key) instanceof Group group) {
         return group.count;
      }
      return 0;
   }

   public int fieldSubject() {
      if (root.parent instanceof Group group) {
         return group.count;
      }
      return 0;
   }
}
