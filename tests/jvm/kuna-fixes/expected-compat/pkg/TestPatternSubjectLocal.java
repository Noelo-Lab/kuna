package pkg;

public class TestPatternSubjectLocal {
   TestPatternSubjectLocal.Entry root;

   public int namedLocal(String key) {
      return this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group ? group.count : 0;
   }

   public int namedLocalNegated(String key) {
      return this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group ? group.count : -1;
   }

   public int namedLocalInLoop(String[] keys) {
      int total = 0;

      for (String key : keys) {
         if (this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group && group.count > 0) {
            total += group.count;
         }
      }

      return total;
   }

   public int subjectExpression(String key) {
      return this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group ? group.count : 0;
   }

   public int fieldSubject() {
      return this.root.parent instanceof TestPatternSubjectLocal.Group group ? group.count : 0;
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
