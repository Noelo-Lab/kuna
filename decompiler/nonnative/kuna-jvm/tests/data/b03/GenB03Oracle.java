// Golden vectors for the B03 slice (NewExprent, VarExprent, FieldExprent), computed by
// the real Vineflower 1.12.0 classes. Regenerate with `sh gen.sh <vineflower-1.12.0.jar>`.
// Every scenario resets EXPRESSION_COUNTER to 1; tests/b03_exprents.rs mirrors the builds.

import java.lang.reflect.Field;
import java.util.*;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.vars.CheckTypesResult;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;

public class GenB03Oracle {
  static final StringBuilder OUT = new StringBuilder();

  static void p(String tag, Object... fields) {
    OUT.append(tag);
    for (Object f : fields) {
      String s = String.valueOf(f);
      if (s.isEmpty() || s.contains(" ")) throw new IllegalStateException("bad token in " + tag + ": '" + s + "'");
      OUT.append(' ').append(s);
    }
    OUT.append('\n');
  }

  static void setField(Object o, Class<?> cls, String name, Object value) {
    try {
      Field f = cls.getDeclaredField(name);
      f.setAccessible(true);
      f.set(o, value);
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static void install() throws Exception {
    Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    uf.setAccessible(true);
    sun.misc.Unsafe u = (sun.misc.Unsafe) uf.get(null);
    DecompilerContext ctx = (DecompilerContext) u.allocateInstance(DecompilerContext.class);
    setField(ctx, DecompilerContext.class, "counterContainer", new CounterContainer());
    setField(ctx, DecompilerContext.class, "properties", new HashMap<String, Object>(IFernflowerPreferences.getDefaults()));
    setField(ctx, DecompilerContext.class, "staticProps", new HashMap<>());
    ClassesProcessor cp = (ClassesProcessor) u.allocateInstance(ClassesProcessor.class);
    setField(cp, ClassesProcessor.class, "mapRootClasses", new HashMap<>());
    setField(ctx, DecompilerContext.class, "classProcessor", cp);
    DecompilerContext.setCurrentContext(ctx);
  }

  static void reset() {
    DecompilerContext.getCounterContainer().setCounter(CounterContainer.EXPRESSION_COUNTER, 1);
  }

  static String hex(String s) {
    StringBuilder sb = new StringBuilder("x");
    for (char c : s.toCharArray()) sb.append(String.format("%04x", (int) c));
    return sb.toString();
  }

  static String vt(VarType t) {
    if (t == null) return "N";
    return t.type.name() + ":" + t.arrayDim + ":" + (t.value == null ? "N" : t.value);
  }

  static String bounds(List<CheckTypesResult.ExprentTypePair> pairs) {
    if (pairs.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (CheckTypesResult.ExprentTypePair pair : pairs) {
      if (sb.length() > 0) sb.append(',');
      sb.append(pair.exprent.id).append('=').append(vt(pair.type));
    }
    return sb.toString();
  }

  static String text(Exprent e) {
    return hex(e.toJava(0).convertToStringAndAllowDataDiscard());
  }

  static String b(boolean v) {
    return v ? "1" : "0";
  }

  public static void main(String[] args) throws Exception {
    install();

    String[] boxed = {"java/lang/Boolean", "java/lang/Byte", "java/lang/Character", "java/lang/Short",
      "java/lang/Integer", "java/lang/Long", "java/lang/Float", "java/lang/Double", "java/lang/Void"};
    for (String cls : boxed) {
      reset();
      FieldExprent f = new FieldExprent("TYPE", cls, true, null, FieldDescriptor.parseDescriptor("Ljava/lang/Class;"), null);
      p("field_type", cls, text(f));
    }

    reset();
    FieldExprent condy = new FieldExprent("TYPE", "java/lang/Long", true, null,
      FieldDescriptor.parseDescriptor("Ljava/lang/Class;"), null, false, true);
    p("field_condy", text(condy));

    int[][] vars = {{5, 0}, {5, 2}, {0, 0}, {10003, 7}};
    for (int[] iv : vars) {
      reset();
      VarExprent v = new VarExprent(iv[0], VarType.VARTYPE_INT, null);
      v.setVersion(iv[1]);
      CheckTypesResult none = v.checkExprTypeBounds();
      v.setBoundType(VarType.VARTYPE_STRING);
      CheckTypesResult r = v.checkExprTypeBounds();
      p("var", iv[0], iv[1], text(v), v.getName(), vt(v.getInferredExprType(null)), none == null ? "N" : "R",
        bounds(r.getLowerBounds()), bounds(r.getUpperBounds()));
    }

    reset();
    NewExprent empty = new NewExprent(new VarType(CodeType.INT, 1), new ArrayList<>(), null);
    empty.setDirectArrayInit(true);
    p("new_direct_empty", text(empty), hex(empty.getLambdaMethodKey()), hex(empty.getLambdaContentMethodKey()),
      b(empty.doesClassHaveMethodsNamedSame()), vt(empty.getInferredExprType(null)), b(empty.isAnonymous()));

    reset();
    List<Exprent> dims = new ArrayList<>();
    dims.add(new ConstExprent(3, false, null));
    NewExprent ints = new NewExprent(new VarType(CodeType.INT, 1), dims, null);
    ints.setLstArrayElements(new ArrayList<>(List.of(new ConstExprent(1, true, null), new ConstExprent(2, false, null))));
    CheckTypesResult r = ints.checkExprTypeBounds();
    p("new_bounds", "int1", bounds(r.getLowerBounds()), bounds(r.getUpperBounds()));

    reset();
    NewExprent strs = new NewExprent(new VarType(CodeType.OBJECT, 1, "java/lang/String"), new ArrayList<>(), null);
    strs.setLstArrayElements(new ArrayList<>(List.of(new ConstExprent(VarType.VARTYPE_NULL, null, null))));
    r = strs.checkExprTypeBounds();
    p("new_bounds", "str1", bounds(r.getLowerBounds()), bounds(r.getUpperBounds()));

    reset();
    List<Exprent> dims2 = new ArrayList<>();
    dims2.add(new ConstExprent(2, false, null));
    dims2.add(new ConstExprent(4, false, null));
    NewExprent grid = new NewExprent(new VarType(CodeType.LONG, 2), dims2, null);
    grid.setLstArrayElements(new ArrayList<>(List.of(new ConstExprent(VarType.VARTYPE_NULL, null, null))));
    r = grid.checkExprTypeBounds();
    p("new_bounds", "long2", bounds(r.getLowerBounds()), bounds(r.getUpperBounds()));

    reset();
    NewExprent obj = new NewExprent(new VarType(CodeType.OBJECT, 0, "java/lang/Object"), new ArrayList<>(), null);
    r = obj.checkExprTypeBounds();
    p("new_bounds", "obj0", bounds(r.getLowerBounds()), bounds(r.getUpperBounds()));

    System.out.print(OUT);
  }
}
