package pkg;

public class TestUnseenClassOverload {
    static String widenFloat(float f) {
        return UnseenSink.value((double) f);
    }

    static String widenInt(int i) {
        return UnseenSink.value((long) i) + UnseenSink.value((float) i) + UnseenSink.value((double) i) + UnseenSink.value(i);
    }

    static String exact(long l, double d) {
        return UnseenSink.value(l) + UnseenSink.value(d);
    }

    static String unbox(Boolean b) {
        return UnseenSink.flag(b.booleanValue()) + UnseenSink.flag(b);
    }

    static String array(String[] names) {
        return UnseenSink.take((Object) names) + UnseenSink.take((Object) null) + UnseenSink.take("a", "b");
    }
}
