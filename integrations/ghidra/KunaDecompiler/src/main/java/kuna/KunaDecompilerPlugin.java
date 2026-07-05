/* kuna — Ghidra extension that swaps the native decompiler core for kuna.
 * Apache-2.0. kuna is derived from Ghidra (https://github.com/NationalSecurityAgency/ghidra).
 */
package kuna;

import docking.action.ToggleDockingAction;
import docking.action.builder.ToggleActionBuilder;
import docking.tool.ToolConstants;
import ghidra.MiscellaneousPluginPackage;
import ghidra.framework.plugintool.Plugin;
import ghidra.framework.plugintool.PluginInfo;
import ghidra.framework.plugintool.PluginTool;
import ghidra.framework.plugintool.util.PluginStatus;

/**
 * Swaps Ghidra's native decompiler core for kuna, the Rust port of the Ghidra decompiler.
 *
 * <p>On plugin load (File → Configure → Miscellaneous → KunaDecompilerPlugin) the plugin
 * resolves the {@code kuna_ghidra} binary shipped in this extension's
 * {@code os/<platform>/} directory (or the {@code -Dkuna.decompiler.exe} dev override)
 * and installs it as the executable behind
 * {@code ghidra.app.decompiler.DecompileProcessFactory}, the single choke point through
 * which every decompiler consumer in Ghidra spawns its native process. The native
 * process is spawned lazily — only on the first decompile — so when this plugin is part
 * of the tool at startup, the swap performed at plugin init precedes the first spawn.
 * When the plugin is instead enabled mid-session (a program is already open and its
 * decompiler process already spawned), that program keeps the stock core until its
 * process respawns; new spawns pick up kuna immediately.
 *
 * <p>The Tools → Kuna Decompiler → "Use Kuna Core" checkbox toggles between the kuna
 * core and the stock {@code decompile} binary at any time. Either direction only
 * affects <em>future</em> process spawns: a program whose decompiler process is already
 * running keeps it until that process restarts (reopen the program, or the process is
 * reset after an error/timeout). There is no public API to force-restart the per-program
 * processes from a plugin (the controllers holding them are private to the Decompiler
 * plugin's providers), so a program reopen is the reliable way to make a swap take
 * effect on an already-decompiled program.
 */
//@formatter:off
@PluginInfo(
	status = PluginStatus.RELEASED,
	packageName = MiscellaneousPluginPackage.NAME,
	category = "Decompiler",
	shortDescription = "Use kuna as the native decompiler core",
	description = "Swaps the native decompiler core for kuna, the Rust port of the " +
		"Ghidra decompiler: points ghidra.app.decompiler.DecompileProcessFactory at the " +
		"kuna_ghidra binary shipped with this extension, so every decompiler consumer in " +
		"this Ghidra session (the Decompiler window, analyzers, scripts) spawns kuna " +
		"instead of the stock 'decompile' process. Tools -> Kuna Decompiler -> " +
		"'Use Kuna Core' toggles between the kuna core and the stock core; " +
		"already-running decompiler processes keep their current core until they " +
		"respawn (program reopen, or process error/timeout). Removing the plugin " +
		"restores the stock core for future spawns."
)
//@formatter:on
public class KunaDecompilerPlugin extends Plugin {

	private ToggleDockingAction useKunaAction;

	public KunaDecompilerPlugin(PluginTool tool) {
		super(tool);
	}

	@Override
	protected void init() {
		super.init();

		// Perform the swap now, before the user's first decompile can spawn a process.
		boolean kunaActive = KunaCoreSwap.swapToKuna(this);
		if (kunaActive) {
			KunaCoreSwap.register(this);
		}

		useKunaAction = new ToggleActionBuilder("Use Kuna Core", getName())
				.menuPath(ToolConstants.MENU_TOOLS, "Kuna Decompiler", "Use Kuna Core")
				.description("Checked: future decompiler processes spawn the kuna core " +
					"(kuna_ghidra). Unchecked: the stock 'decompile' binary. Programs " +
					"with an already-running decompiler process keep it until that " +
					"process restarts (reopen the program, or the process is reset " +
					"after an error/timeout).")
				.selected(kunaActive)
				.onAction(context -> onToggle())
				.buildAndInstall(tool);
	}

	private void onToggle() {
		// The docking framework flips the toggle state before invoking the callback,
		// so isSelected() is the state the user just requested.
		if (useKunaAction.isSelected()) {
			if (KunaCoreSwap.swapToKuna(this)) {
				KunaCoreSwap.register(this);
			}
			else {
				useKunaAction.setSelected(false); // swap failed: reflect reality
			}
		}
		else {
			// This tool no longer wants kuna; restore only once the last tool leaves.
			if (KunaCoreSwap.unregister(this) == 0) {
				if (!KunaCoreSwap.restoreStock(this)) {
					KunaCoreSwap.register(this); // restore failed: kuna still active
					useKunaAction.setSelected(true);
				}
			}
		}
	}

	@Override
	protected void dispose() {
		// Leaving the tool (or being un-configured) should not silently leave kuna as
		// the JVM-wide core. The factory field is static, so restore only when this was
		// the LAST tool holding the kuna core — otherwise closing one tool would revert
		// kuna for the others still using it.
		if (KunaCoreSwap.unregister(this) == 0 && KunaCoreSwap.isKunaActive()) {
			KunaCoreSwap.restoreStock(this);
		}
		super.dispose();
	}
}
