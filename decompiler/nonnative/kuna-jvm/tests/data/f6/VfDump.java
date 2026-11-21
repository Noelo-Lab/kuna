package org.jetbrains.java.decompiler.struct;

import java.io.File;
import java.io.PrintStream;
import java.lang.reflect.Field;
import java.util.HashMap;
import java.util.Map;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.extern.IContextSource;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;

// Oracle for the kuna-jvm context sources: prints what Vineflower 1.12.0 registers.
//   java -cp vineflower-1.12.0.jar:<out> org.jetbrains.java.decompiler.struct.VfDump runtime <spec>
//   java -cp vineflower-1.12.0.jar:<out> org.jetbrains.java.decompiler.struct.VfDump space <path>...
public class VfDump {
  public static void main(String[] args) throws Exception {
    PrintStream out = new PrintStream(System.out, false, "UTF-8");
    Map<String, Object> props = new HashMap<>();
    props.put("include-runtime", args[0].equals("runtime") ? args[1] : "");
    new Fernflower(null, props, IFernflowerLogger.NO_OP);
    StructContext ctx = DecompilerContext.getStructContext();
    if (args[0].equals("space")) {
      for (int i = 1; i < args.length; i++) {
        try {
          ctx.addSpace(new File(args[i]), false);
        } catch (Throwable t) {
          out.println("THROW " + t.getClass().getSimpleName());
        }
      }
    }
    Field field = ContextUnit.class.getDeclaredField("source");
    field.setAccessible(true);
    for (ContextUnit unit : ctx.getUnits()) {
      IContextSource source = (IContextSource) field.get(unit);
      out.println("UNIT " + source.getName());
      IContextSource.Entries entries;
      try {
        entries = source.getEntries();
      } catch (Throwable t) {
        out.println("THROW " + t.getClass().getSimpleName());
        continue;
      }
      out.println("R " + unit.isRoot() + " " + source.isLazy());
      for (IContextSource.Entry e : entries.classes()) out.println("C " + e.multirelease() + " " + e.basePath());
      for (String d : entries.directories()) out.println("D " + d);
      for (IContextSource.Entry e : entries.others()) out.println("O " + e.multirelease() + " " + e.basePath());
      out.println("K " + entries.childContexts().size());
    }
    out.flush();
  }
}
