package pkg;

public class TestUpdateOnOnePath {
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

   static void rename(TestUpdateOnOnePath.Symbol symbol, String name) {
      for (int i = 1; i < 2147483647; i++) {
         try {
            symbol.rename(name + "_" + i);
            break;
         } catch (TestUpdateOnOnePath.Duplicate var4) {
         } catch (TestUpdateOnOnePath.Invalid e) {
            break;
         }
      }
   }

   static class Duplicate extends Exception {
   }

   static class Invalid extends Exception {
   }

   interface Symbol {
      void rename(String var1) throws TestUpdateOnOnePath.Duplicate, TestUpdateOnOnePath.Invalid;
   }
}
