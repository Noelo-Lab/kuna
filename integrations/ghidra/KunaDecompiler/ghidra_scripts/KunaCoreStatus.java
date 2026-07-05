//Prints which native decompiler executable Ghidra will spawn for the NEXT decompiler
//process (the kuna core-swap seam). Reads the private static field
//ghidra.app.decompiler.DecompileProcessFactory.exepath by reflection: null means the
//stock resolution (Application.getOSFile("decompile")) runs lazily on first use.
//Already-running decompiler processes may still be an older core; this reports what
//future spawns will use.
//@category Kuna
import java.lang.reflect.Field;

import ghidra.app.decompiler.DecompileProcessFactory;
import ghidra.app.script.GhidraScript;

public class KunaCoreStatus extends GhidraScript {

	@Override
	public void run() throws Exception {
		Field field = DecompileProcessFactory.class.getDeclaredField("exepath");
		field.setAccessible(true); // flat classpath / unnamed module: no --add-opens needed
		String path = (String) field.get(null);

		if (path == null) {
			println("DecompileProcessFactory.exepath = (unset)");
			println("Next decompiler process: STOCK core, resolved lazily via " +
				"Application.getOSFile(\"decompile\").");
			println("(The KunaDecompilerPlugin is not active, or the stock core was " +
				"restored without an eager re-resolve.)");
			return;
		}

		println("DecompileProcessFactory.exepath = " + path);
		if (path.contains("kuna_ghidra")) {
			println("Next decompiler process: KUNA core.");
		}
		else if (path.endsWith("decompile") || path.endsWith("decompile.exe")) {
			println("Next decompiler process: STOCK core.");
		}
		else {
			println("Next decompiler process: a custom core (neither kuna_ghidra nor " +
				"the stock 'decompile' name).");
		}
	}
}
