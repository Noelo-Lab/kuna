# Ghidra namestyle disconnected parameter bodies

The standalone `namestyle ghidra` policy already printed an unnamed prototype
slot as `param_1`, but `Funcdata::link_proto_params` independently materialized
the symbol bound into the body as `a0`. P6 therefore named the input
HighVariable `a0`, while P9's signature and parameter-declaration suppression
both expected `param_1`. The C output declared a new, uninitialized `a0` local
and used it instead of the formal parameter.

The repair gives all three sites one parameter-name helper. An empty prototype
name becomes `aN` exactly when the local `name_style_angr` flag is set and
becomes `param_N` otherwise. A nonempty recovered or explicit name is returned
unchanged. This deliberately does not consult `kuna_name_style()`: GUI Ghidra
mode sets the separate address-style override while retaining angr local and
parameter names.

This is an unflagged correction to the existing `namestyle` contract. It does
not change parameter recovery, types, storage, data flow, local naming, or
address-derived names. The separate incomplete Ghidra-style naming of some
ordinary locals reported in the need remains out of scope.
