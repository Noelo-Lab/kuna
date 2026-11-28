// A loop's update reached from one case or one handler only keeps that path's exit: javac -g --release 17
package pkg;

public class TestUpdateOnOnePath {
   static class Duplicate extends Exception {
   }

   static class Invalid extends Exception {
   }

   interface Symbol {
      void rename(String name) throws Duplicate, Invalid;
   }

   static int scan(String s, int start) {
      for (int i = start; i < s.length(); i++) {
         switch (s.charAt(i)) {
            case '0':
            case '1':
               break;
            default:
               return i;
         }
      }
      return -1;
   }

   static void rename(Symbol symbol, String name) {
      for (int i = 1; i < Integer.MAX_VALUE; i++) {
         try {
            symbol.rename(name + "_" + i);
            break;
         } catch (Duplicate e) {
         } catch (Invalid e) {
            break;
         }
      }
   }
}
