package pkg;

public class TestSwitchTableFillers {
    static int width(int len) {
        switch (len) {
            case 1:
                return 8;
            case 2:
                return 16;
            case 4:
                return 32;
            default:
                throw new IllegalArgumentException("len " + len);
        }
    }

    static int explicitGap(int x) {
        switch (x) {
            case 1:
                return 10;
            case 3:
                return 30;
            case 2:
            default:
                return 99;
        }
    }

    static long size(int n) {
        long v = 0;
        switch (n) {
            case 1:
                v = 1;
                break;
            case 2:
                v = 2;
                break;
            case 4:
                v = 4;
                break;
            case 8:
                v = 8;
                break;
        }
        return v;
    }

    static int groups(int x) {
        switch (x) {
            case 2:
            case 8:
                return 1;
            case 1:
            case 5:
                return 2;
            default:
                return 0;
        }
    }
}
