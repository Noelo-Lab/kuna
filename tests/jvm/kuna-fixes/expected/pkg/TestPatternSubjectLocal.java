package pkg;

public class TestPatternSubjectLocal {
   TestPatternSubjectLocal.Entry root;

   public int namedLocal(String key) {
      TestPatternSubjectLocal.Entry entry = this.root.lookup(key);
      if (entry instanceof TestPatternSubjectLocal.Group group) {
         return group.count;
      }

      return 0;
   }

   public int namedLocalNegated(String key) {
      TestPatternSubjectLocal.Entry entry = this.root.lookup(key);
      if (!(entry instanceof TestPatternSubjectLocal.Group group)) {
         return -1;
      }

      return group.count;
   }

   public int namedLocalInLoop(String[] keys) {
      int total = 0;

      for (String key : keys) {
         TestPatternSubjectLocal.Entry entry = this.root.lookup(key);
         if (entry instanceof TestPatternSubjectLocal.Group group && group.count > 0) {
            total += group.count;
         }
      }

      return total;
   }

   public int subjectExpression(String key) {
      if (this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group) {
         return group.count;
      }

      return 0;
   }

   public int fieldSubject() {
      if (this.root.parent instanceof TestPatternSubjectLocal.Group group) {
         return group.count;
      }

      return 0;
   }

   public static class Entry {
      TestPatternSubjectLocal.Entry parent;

      TestPatternSubjectLocal.Entry lookup(String key) {
         return this.parent;
      }
   }

   public static class Group extends TestPatternSubjectLocal.Entry {
      int count;
   }
}
