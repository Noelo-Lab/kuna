package mx;

import java.util.concurrent.TimeUnit;

public class Maps {
    static int state(Thread.State s, int r) {
        switch (s) {
            case NEW:
                r += 10;
                break;
            case BLOCKED:
                r += 20;
                break;
        }
        return r * 3 + 1;
    }

    static int unit(TimeUnit u, int r) {
        switch (u) {
            case SECONDS:
                r += 10;
                break;
            case HOURS:
                r += 20;
                break;
            case DAYS:
                r += 30;
                break;
        }
        return r * 5 + 1;
    }

    static int both(TimeUnit u, Thread.State s) {
        return unit(u, 1) + state(s, 2);
    }
}
