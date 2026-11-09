// Oracle for kuna's MethodPropertiesProcessor port: for every method of every input class,
// runs MethodPropertiesProcessor.parse the way ClassWrapper.init does and prints the result.
// See gen.sh.
//
// usage: PropsDump OUT INPUT...      INPUT = file.class | dir | jar

import org.jetbrains.java.decompiler.code.MethodProperties;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.modules.code.MethodPropertiesProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.util.DataInputFullStream;

import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;
import java.util.jar.Manifest;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class PropsDump {
  private static final class NullSaver implements IResultSaver {
    public void saveFolder(String path) { }
    public void copyFile(String source, String path, String entryName) { }
    public void saveClassFile(String path, String qualifiedName, String entryName, String content, int[] mapping) { }
    public void createArchive(String path, String archiveName, Manifest manifest) { }
    public void saveDirEntry(String path, String archiveName, String entryName) { }
    public void copyEntry(String source, String path, String archiveName, String entry) { }
    public void saveClassEntry(String path, String archiveName, String qualifiedName, String entryName, String content) { }
    public void closeArchive(String path, String archiveName) { }
  }

  private static List<byte[]> collect(String input) throws Exception {
    List<byte[]> res = new ArrayList<>();
    Path p = Paths.get(input);
    if (input.endsWith(".jar")) {
      try (java.util.zip.ZipFile zip = new java.util.zip.ZipFile(p.toFile())) {
        List<? extends java.util.zip.ZipEntry> entries = Collections.list(zip.entries());
        entries.sort(Comparator.comparing(java.util.zip.ZipEntry::getName));
        for (java.util.zip.ZipEntry e : entries) {
          if (e.getName().endsWith(".class")) res.add(zip.getInputStream(e).readAllBytes());
        }
      }
    } else if (Files.isDirectory(p)) {
      List<Path> files;
      try (Stream<Path> s = Files.walk(p)) {
        files = s.filter(f -> f.toString().endsWith(".class")).sorted().collect(Collectors.toList());
      }
      for (Path f : files) res.add(Files.readAllBytes(f));
    } else {
      res.add(Files.readAllBytes(p));
    }
    return res;
  }

  public static void main(String[] args) throws Exception {
    PluginSources.PLUGIN_SOURCES.clear();
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    options.put(IFernflowerPreferences.LOG_LEVEL, "error");
    new Fernflower(new NullSaver(), options, new PrintStreamLogger(System.err));

    int methods = 0, lambdas = 0, asserts = 0;
    try (PrintStream ps = new PrintStream(Files.newOutputStream(Paths.get(args[0])), false, StandardCharsets.UTF_8)) {
      ps.append("#props vineflower-1.12.0\n");
      for (int a = 1; a < args.length; a++) {
        for (byte[] bytes : collect(args[a])) {
          StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
          DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);
          for (StructMethod mt : cl.getMethods()) {
            MethodProperties props = new MethodProperties(mt);
            MethodPropertiesProcessor.parse(mt, cl, props);
            ps.append("m ").append(cl.qualifiedName).append(' ').append(mt.getName()).append(' ').append(mt.getDescriptor())
              .append(" syn=").append(props.isSyntheticReferenceLambda ? "1" : "0")
              .append(" assert=").append(props.assertField == null ? "-" : props.assertField.getName() + ":" + props.assertField.getDescriptor())
              .append('\n');
            methods++;
            if (props.isSyntheticReferenceLambda) lambdas++;
            if (props.assertField != null) asserts++;
          }
          DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, null);
        }
      }
    }
    System.err.println("methods: " + methods + ", reference lambdas: " + lambdas + ", assert fields: " + asserts);
  }
}
