import java.lang.reflect.*;
import java.nio.file.*;
import java.util.*;

// Gen LIST OUT: writes OUT/bp/Probe.java with one static method per backported JDK method of LIST
// (lines "owner#name(desc)ret"), each calling exactly that method on its parameters, and OUT/index.tsv
// (probe index, list line, invoke kind: static / virtual / interface).
public class Gen {
    static String src(Class<?> c) { return c.getCanonicalName(); }

    public static void main(String[] a) throws Exception {
        List<String> lines = Files.readAllLines(Path.of(a[0]));
        StringBuilder sb = new StringBuilder("package bp;\n@SuppressWarnings(\"all\")\npublic class Probe {\n");
        StringBuilder idx = new StringBuilder();
        int n = 0;
        for (String line : lines) {
            if (!line.startsWith("java/")) continue;
            String owner = line.substring(0, line.indexOf('#')).replace('/', '.');
            String rest = line.substring(line.indexOf('#') + 1);
            String name = rest.substring(0, rest.indexOf('('));
            String desc = rest.substring(rest.indexOf('('));
            Class<?> c;
            try { c = Class.forName(owner); } catch (Throwable t) { continue; }
            Method found = null;
            for (Method m : c.getMethods()) {
                if (m.getName().equals(name) && descOf(m).equals(desc)) found = m;
            }
            if (found == null || !Modifier.isPublic(c.getModifiers())) { System.err.println("skip " + line); continue; }
            boolean st = Modifier.isStatic(found.getModifiers());
            StringBuilder params = new StringBuilder();
            StringBuilder args = new StringBuilder();
            int k = 0;
            if (!st) { params.append(src(c)).append(" r"); }
            for (Class<?> p : found.getParameterTypes()) {
                if (params.length() > 0) params.append(", ");
                params.append(src(p)).append(" a").append(k);
                if (args.length() > 0) args.append(", ");
                args.append("a").append(k);
                k++;
            }
            String call = (st ? src(c) : "r") + "." + name + "(" + args + ")";
            String ret = src(found.getReturnType());
            String throwsc = found.getExceptionTypes().length > 0 ? " throws Throwable" : "";
            sb.append("  public static ").append(ret).append(" p").append(n).append("(").append(params).append(")").append(throwsc).append(" { ")
              .append(ret.equals("void") ? call + ";" : "return " + call + ";").append(" }\n");
            idx.append(n).append('\t').append(line).append('\t').append(st ? (c.isInterface() ? "istatic" : "static") : c.isInterface() ? "interface" : "virtual").append('\n');
            n++;
        }
        sb.append("}\n");
        Files.createDirectories(Path.of(a[1], "bp"));
        Files.writeString(Path.of(a[1], "bp", "Probe.java"), sb);
        Files.writeString(Path.of(a[1], "index.tsv"), idx);
        System.err.println(n + " probes");
    }

    static String d(Class<?> c) {
        if (c.isArray()) return c.getName().replace('.', '/');
        if (c.isPrimitive()) {
            return switch (c.getName()) {
                case "int" -> "I"; case "long" -> "J"; case "boolean" -> "Z"; case "byte" -> "B";
                case "char" -> "C"; case "short" -> "S"; case "float" -> "F"; case "double" -> "D"; default -> "V";
            };
        }
        return "L" + c.getName().replace('.', '/') + ";";
    }

    static String descOf(Method m) {
        StringBuilder s = new StringBuilder("(");
        for (Class<?> p : m.getParameterTypes()) s.append(d(p));
        return s.append(")").append(d(m.getReturnType())).toString();
    }
}
