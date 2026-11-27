package pkg;

public class DelegationJava {
    private static int calls;

    public static String label(String s) {
        calls++;
        return s;
    }

    public static String count() {
        calls++;
        return "n";
    }
}
