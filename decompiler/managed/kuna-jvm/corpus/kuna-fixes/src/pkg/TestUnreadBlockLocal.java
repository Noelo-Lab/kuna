package pkg;

public class TestUnreadBlockLocal {
    String label(int n) {
        {
            int[] counts = new int[n];
        }
        String text = "n=";
        return text + n;
    }
}
