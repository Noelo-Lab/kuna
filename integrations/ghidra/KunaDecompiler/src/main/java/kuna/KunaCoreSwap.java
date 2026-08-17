/* kuna — Ghidra extension that swaps the native decompiler core for kuna.
 * Apache-2.0. kuna is derived from Ghidra (https://github.com/NationalSecurityAgency/ghidra).
 */
package kuna;

import java.io.File;
import java.io.FileNotFoundException;
import java.lang.reflect.Field;

import ghidra.app.decompiler.DecompileProcessFactory;
import ghidra.framework.Application;
import ghidra.framework.OSFileNotFoundException;
import ghidra.framework.OperatingSystem;
import ghidra.framework.Platform;
import ghidra.util.Msg;

/**
 * The mechanism that points Ghidra's decompiler-process factory at the kuna binary,
 * and back at the stock one.
 *
 * <p>Every decompiler consumer in Ghidra (the Decompiler window, the switch/param-id
 * analyzers, BSim, scripts, ...) funnels process creation through the static
 * {@code ghidra.app.decompiler.DecompileProcessFactory.get()}, which resolves the
 * executable once and caches it in the {@code private static String exepath} field
 * (Ghidra 12.2, DecompileProcessFactory.java:28). {@code getExePath()} early-returns
 * when the field is non-null, so writing the field wins for every <em>future</em>
 * process spawn — there is no other seam (no system property, env var, service, or
 * ExtensionPoint anywhere in {@code ghidra.app.decompiler}).
 *
 * <p>The reflective write is legal without {@code --add-opens}: Ghidra runs a flat
 * {@code URLClassLoader} classpath ({@code ghidra.GhidraClassLoader}), so this class
 * and {@code DecompileProcessFactory} both live in the unnamed module and
 * {@code setAccessible(true)} is unrestricted.
 *
 * <p>Scope caveats, shared by both directions of the swap:
 * <ul>
 * <li>The field is static, i.e. JVM-wide: the swap affects every tool in this Ghidra
 * session, not just the tool hosting the plugin.</li>
 * <li>A {@link ghidra.app.decompiler.DecompInterface} that already spawned a process
 * keeps talking to it (the old binary) until that process is respawned — program
 * reopen, decompiler reset, or a process error/timeout. Only new spawns see the new
 * path.</li>
 * </ul>
 */
public final class KunaCoreSwap {

	/**
	 * The extension's module name — the directory name the extension installs under,
	 * used to scope the two-arg {@link Application#getOSFile(String, String)} lookup
	 * to this module's {@code os/<platform>/} directory.
	 */
	public static final String MODULE_NAME = "KunaDecompiler";

	/** Dev override: {@code -Dkuna.decompiler.exe=/abs/path/to/kuna_ghidra}. */
	public static final String EXE_PROPERTY = "kuna.decompiler.exe";

	private static final String KUNA_EXECNAME = "kuna_ghidra";
	private static final String KUNA_WIN_EXECNAME = "kuna_ghidra.exe";

	// Mirror DecompileProcessFactory.EXECNAME / WIN32_EXECNAME (Ghidra 12.2, :29-30)
	private static final String STOCK_EXECNAME = "decompile";
	private static final String STOCK_WIN_EXECNAME = "decompile.exe";

	private static final String EXEPATH_FIELD = "exepath";

	/** The kuna path last installed by {@link #swapToKuna()}, for {@link #isKunaActive()}. */
	private static String lastKunaPath;

	/**
	 * Live plugin instances that requested the kuna core. The {@code exepath} field is
	 * JVM-wide, so if two tools host the plugin, only the LAST one to leave should restore
	 * the stock core — otherwise closing one tool silently reverts kuna for the other.
	 * Guarded by the class monitor (all mutators are {@code synchronized}).
	 */
	private static final java.util.Set<Object> ACTIVE_PLUGINS =
		java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<>());

	private KunaCoreSwap() {
		// static-only
	}

	/**
	 * Registers a plugin instance as a kuna-core user (call from the plugin's swap).
	 *
	 * @return the number of live registrants after adding this one
	 */
	public static synchronized int register(Object plugin) {
		ACTIVE_PLUGINS.add(plugin);
		return ACTIVE_PLUGINS.size();
	}

	/**
	 * Unregisters a plugin instance.
	 *
	 * @return the number of live registrants remaining (0 = this was the last one, so the
	 *         caller should restore the stock core)
	 */
	public static synchronized int unregister(Object plugin) {
		ACTIVE_PLUGINS.remove(plugin);
		return ACTIVE_PLUGINS.size();
	}

	private static boolean isWindows() {
		return Platform.CURRENT_PLATFORM.getOperatingSystem() == OperatingSystem.WINDOWS;
	}

	/**
	 * Resolves the kuna decompiler binary: (1) the {@value #EXE_PROPERTY} system
	 * property, (2) this extension's {@code os/<platform>/kuna_ghidra} via the two-arg
	 * module-scoped {@link Application#getOSFile(String, String)}.
	 *
	 * @return the executable file (exists and is executable)
	 * @throws FileNotFoundException with user-actionable instructions if not found
	 */
	public static File findKunaBinary() throws FileNotFoundException {
		String override = System.getProperty(EXE_PROPERTY);
		if (override != null && !override.isBlank()) {
			File file = new File(override);
			if (!file.isFile()) {
				throw new FileNotFoundException(
					"-D" + EXE_PROPERTY + " points at a missing file: " + file.getAbsolutePath());
			}
			return checkExecutable(file);
		}

		String exeName = isWindows() ? KUNA_WIN_EXECNAME : KUNA_EXECNAME;
		try {
			return checkExecutable(Application.getOSFile(MODULE_NAME, exeName));
		}
		catch (OSFileNotFoundException e) {
			throw new FileNotFoundException("The kuna decompiler binary '" + exeName +
				"' was not found in the " + MODULE_NAME + " extension.\n\n" +
				"Build it and copy it into the installed extension, then restart Ghidra:\n" +
				"    cd <kuna-repo>/decompiler && cargo build --release -p kuna-ghidra\n" +
				"    cp target/release/" + exeName + " <ghidra user settings>/Extensions/" +
				MODULE_NAME + "/os/" + Platform.CURRENT_PLATFORM.getDirectoryName() + "/\n\n" +
				"Alternatively point the JVM property -D" + EXE_PROPERTY +
				"=/abs/path/to/" + exeName + " at a build.");
		}
	}

	/**
	 * Verifies the binary is executable, first attempting to set the exec bit itself
	 * (self-heal for zips unpacked by tools that drop unix modes); throws only if the
	 * bit still cannot be gained.
	 */
	private static File checkExecutable(File file) throws FileNotFoundException {
		if (!file.canExecute()) {
			file.setExecutable(true, false);
		}
		if (!file.canExecute()) {
			throw new FileNotFoundException("The kuna decompiler binary exists but is not " +
				"executable: " + file.getAbsolutePath() + " (chmod +x it).");
		}
		return file;
	}

	/**
	 * Resolves the stock native decompiler binary. Uses the single-arg
	 * {@link Application#getOSFile(String)}: it searches the calling class's module
	 * first — this ({@value #MODULE_NAME}) module, which does not ship a file named
	 * {@code decompile} — then falls back to scanning every module, where the
	 * Features/Decompiler module's binary is found ({@code build/os/<platform>/} is
	 * preferred over {@code os/<platform>/} within a module, so a dev-built or
	 * file-dropped binary there wins, exactly as it does for the stock factory).
	 *
	 * @return the stock executable file
	 * @throws OSFileNotFoundException if no module provides the binary
	 */
	public static File findStockBinary() throws OSFileNotFoundException {
		String exeName = isWindows() ? STOCK_WIN_EXECNAME : STOCK_EXECNAME;
		return Application.getOSFile(exeName);
	}

	private static Field exepathField() throws NoSuchFieldException {
		Field field = DecompileProcessFactory.class.getDeclaredField(EXEPATH_FIELD);
		field.setAccessible(true); // flat classpath / unnamed module: no --add-opens needed
		return field;
	}

	/**
	 * Reads the currently installed executable path (reflective read of
	 * {@code DecompileProcessFactory.exepath}).
	 *
	 * @return the cached path, or null if unset (the factory will lazily resolve the
	 *         stock binary on the next spawn)
	 * @throws ReflectiveOperationException if the Ghidra version renamed the field
	 */
	public static String getInstalledExePath() throws ReflectiveOperationException {
		return (String) exepathField().get(null);
	}

	/**
	 * Writes {@code DecompileProcessFactory.exepath}. {@code getExePath()} early-returns
	 * when the field is non-null, so a non-null value set here wins for every future
	 * {@code DecompileProcessFactory.get()}; overwriting an already-cached value is
	 * equally effective for future spawns (already-open DecompInterfaces keep their
	 * running process until it respawns). Passing null restores the factory's own lazy
	 * stock resolution.
	 */
	public static void setInstalledExePath(String path) throws ReflectiveOperationException {
		exepathField().set(null, path);
	}

	/**
	 * @return true if the most recent successful {@link #swapToKuna()} path is still the
	 *         installed one (false on reflection failure)
	 */
	public static synchronized boolean isKunaActive() {
		try {
			String current = getInstalledExePath();
			return current != null && current.equals(lastKunaPath);
		}
		catch (ReflectiveOperationException | SecurityException e) {
			return false;
		}
	}

	/**
	 * Points the decompiler-process factory at the kuna binary. Shows an error dialog
	 * (and leaves the factory untouched) if the binary cannot be resolved or the
	 * reflective write fails.
	 *
	 * @param originator the object requesting the swap (for Msg attribution)
	 * @return true if kuna is now the core for future decompiler spawns
	 */
	public static synchronized boolean swapToKuna(Object originator) {
		File kunaBinary;
		try {
			kunaBinary = findKunaBinary();
		}
		catch (FileNotFoundException e) {
			Msg.showError(originator, null, "Kuna Decompiler Binary Not Found", e.getMessage());
			return false;
		}
		String path = kunaBinary.getAbsolutePath();
		try {
			setInstalledExePath(path);
		}
		catch (ReflectiveOperationException | SecurityException e) {
			showReflectionError(originator, e);
			return false;
		}
		lastKunaPath = path;
		Msg.info(originator, "Decompiler core is now KUNA: " + path +
			" (already-running decompiler processes keep the previous core until they respawn)");
		return true;
	}

	/**
	 * Points the decompiler-process factory back at the stock {@code decompile} binary.
	 * If the stock binary cannot be resolved from here, the cache is cleared (null) so
	 * the factory re-runs its own stock resolution — and shows its own error — on the
	 * next spawn.
	 *
	 * @param originator the object requesting the restore (for Msg attribution)
	 * @return true if the stock core is restored for future decompiler spawns
	 */
	public static synchronized boolean restoreStock(Object originator) {
		String path = null;
		try {
			path = findStockBinary().getAbsolutePath();
		}
		catch (OSFileNotFoundException e) {
			Msg.warn(originator, "Stock decompiler binary not found (" + e.getMessage() +
				"); clearing the cached path so Ghidra re-resolves it on the next decompile");
		}
		try {
			setInstalledExePath(path);
		}
		catch (ReflectiveOperationException | SecurityException e) {
			showReflectionError(originator, e);
			return false;
		}
		lastKunaPath = null;
		Msg.info(originator, "Decompiler core is now STOCK: " +
			(path != null ? path : "(lazily re-resolved on next decompile)") +
			" (already-running decompiler processes keep the previous core until they respawn)");
		return true;
	}

	private static void showReflectionError(Object originator, Exception e) {
		Msg.showError(originator, null, "Kuna Core Swap Failed",
			"Could not access ghidra.app.decompiler.DecompileProcessFactory." + EXEPATH_FIELD +
				" by reflection. This Ghidra version may have changed the factory's " +
				"field layout; the " + MODULE_NAME + " extension targets Ghidra 12.2.",
			e);
	}
}
