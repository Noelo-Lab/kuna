package pkg;

public class TestPatternChainSlots {
   Object target;

   int separate() {
      if (this.target instanceof TestPatternChainSlots.Box box) {
         return box.w;
      }

      if (this.target instanceof TestPatternChainSlots.Ring ring) {
         return ring.r;
      }

      return 0;
   }

   String chained(Object value) {
      if (value instanceof TestPatternChainSlots.Box box && box.w > 0) {
         return "box" + box.w;
      } else if (value instanceof TestPatternChainSlots.Ring ring) {
         return "ring" + ring.r;
      } else {
         return null;
      }
   }

   int flagFirst(boolean flag, Object value) {
      if (flag) {
         int doubled = value.hashCode() * 2;
         return doubled;
      } else if (value instanceof TestPatternChainSlots.Ring ring) {
         int r = ring.r;
         return r + 1;
      } else {
         return -1;
      }
   }

   int flagSeparate(boolean flag, Object value) {
      if (flag) {
         int doubled = value.hashCode() * 2;
         return doubled;
      }

      if (value instanceof TestPatternChainSlots.Ring ring) {
         int r = ring.r;
         return r + 1;
      }

      return -1;
   }

   boolean bounded(StringBuilder b, Object value, int limit) {
      if (value instanceof TestPatternChainSlots.Box box) {
         return box.w < limit;
      } else {
         CharSequence text = (CharSequence)(value instanceof CharSequence chars ? chars : String.valueOf(value));
         int room = limit - b.length();
         if (text.length() <= room) {
            b.append(text);
            return true;
         }

         b.append(text, 0, room);
         return false;
      }
   }

   static class Box implements TestPatternChainSlots.Shape {
      int w;

      @Override
      public int size() {
         return this.w;
      }
   }

   static class Ring implements TestPatternChainSlots.Shape {
      int r;

      @Override
      public int size() {
         return this.r * 2;
      }
   }

   interface Shape {
      int size();
   }
}
