package pkg;

public class TestUnseenClassOverload {
   static String widenFloat(float f) {
      return UnseenSink.value(f);
   }

   static String widenInt(int i) {
      return UnseenSink.value(i) + UnseenSink.value(i) + UnseenSink.value(i) + UnseenSink.value(i);
   }

   static String exact(long l, double d) {
      return UnseenSink.value(l) + UnseenSink.value(d);
   }

   static String unbox(Boolean b) {
      return UnseenSink.flag(b) + UnseenSink.flag(b);
   }

   static String array(String[] names) {
      return UnseenSink.take(names) + UnseenSink.take(null) + UnseenSink.take(new Object[]{"a", "b"});
   }
}
