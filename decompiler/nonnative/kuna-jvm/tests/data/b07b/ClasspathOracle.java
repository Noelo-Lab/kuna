import java.nio.file.Files;
import java.nio.file.Path;
import java.lang.reflect.Method;

import org.jetbrains.java.decompiler.modules.decompiler.ClasspathHelper;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;

/**
 * Golden vectors for kuna-jvm's ClasspathHelper: runs Vineflower's own
 * ClasspathHelper.findMethod over the queries in classpath_queries.tsv.
 *
 *   javac -cp vineflower-1.12.0.jar -d out ClasspathOracle.java
 *   java -cp vineflower-1.12.0.jar:out ClasspathOracle classpath_queries.tsv > classpath_helper.tsv
 */
public class ClasspathOracle {
  public static void main(String[] args) throws Exception {
    System.out.println("# java.version=" + System.getProperty("java.version"));
    for (String line : Files.readAllLines(Path.of(args[0]))) {
      if (line.isEmpty() || line.startsWith("#")) {
        continue;
      }
      String[] q = line.split("\t");
      Method m = ClasspathHelper.findMethod(q[0], q[1], MethodDescriptor.parseDescriptor(q[2]));
      String result = m == null ? "null\t-" : m.toString() + "\t" + m.isVarArgs();
      System.out.println(q[0] + "\t" + q[1] + "\t" + q[2] + "\t" + result);
    }
  }
}
