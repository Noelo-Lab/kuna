package pkg;

public class TestSwitchEmptyGroupBreak {
    static int decode(char c, int pos) {
        switch (c) {
            case 'p': {
                pos += 3;
                break;
            }
            case 'q': {
                pos += 2;
                break;
            }
            case '/':
            case '.':
                break;
            case 'a':
                pos = 7;
                break;
            case 'm':
                pos = 9;
                break;
            default:
                pos = -pos;
                break;
        }
        return pos;
    }
}
