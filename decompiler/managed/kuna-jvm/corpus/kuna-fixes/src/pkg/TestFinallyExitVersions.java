package pkg;

public class TestFinallyExitVersions {
    static class Handle {
        private int closed;

        void close() {
            closed++;
        }

        public String toString() {
            return "Handle" + closed;
        }
    }

    interface Src {
        void work();
        void use(Object v);
        int g();
        Object obj();
        Handle open(Object from);
        Object next();
    }

    public void typed(Src src) {
        Object x = "a";
        try {
            src.work();
        } finally {
            x = src.obj();
        }
        src.use(x);
    }

    public void conditional(Src src, Object o) {
        Object x = o;
        try {
            src.work();
        } finally {
            if (src.g() > 0) {
                x = "s";
            }
        }
        src.use(x);
    }

    public void withCatch(Src src) {
        Object x = "a";
        try {
            src.work();
        } catch (IllegalStateException e) {
            x = "c";
        } finally {
            x = src.obj();
        }
        src.use(x);
    }

    public void nested(Src src) {
        Object x = "a";
        try {
            try {
                src.work();
            } finally {
                x = src.obj();
            }
            src.use(x);
        } finally {
            x = "b";
        }
        src.use(x);
    }

    public void loop(Src src) {
        Object x = "a";
        while (src.g() > 0) {
            try {
                if (src.g() > 5) {
                    break;
                }
                src.work();
            } finally {
                x = src.obj();
            }
        }
        src.use(x);
    }

    public void reset(Src src) {
        Object cf = src.next();
        Handle handle = null;
        while (cf != null) {
            try {
                handle = src.open(cf);
                src.use(handle);
            } finally {
                if (handle != null) {
                    handle.close();
                    src.use(handle);
                    handle = null;
                }
            }
            cf = src.next();
        }
    }

    public Object local(Src src) {
        boolean flag = false;
        try {
            Object r = src.next();
            flag = true;
            return r;
        } finally {
            int t = src.g();
            if (t > 0 && flag) {
                src.use(src.obj());
            }
        }
    }
}
