package pkg;

public class TestVarMergeLostDefinition {
    interface Item {
    }

    interface Src {
        void use(Object v);
        void work();
        boolean done();
        Item item();
    }

    public void afterTry(Src src) {
        {
            String s = "a";
        }
        Item k;
        try {
            k = src.item();
            src.work();
        } finally {
            src.work();
        }
        src.use(k);
    }

    public void afterLoop(Src src) {
        {
            String s = "a";
        }
        Item k;
        do {
            k = src.item();
        } while (!src.done());
        src.use(k);
    }
}
