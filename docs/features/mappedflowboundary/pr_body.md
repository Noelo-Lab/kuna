## Problem

A mapped return is lost when another path falls off the image; JSON reports null code.

```sh
python3 -c "import base64; open('edge.elf','wb').write(base64.b64decode('f0VMRgEBAQAAAAAAAAAAAAIAAwABAAAAAAABADQAAAAAAAAAAAAAADQAIAABACgAAAAAAAEAAABUAAAAAAABAAAAAQAPAAAADwAAAAUAAAABAAAAhcB1BrgHAAAAw7sFAAAA'))"
kuna decompile edge.elf 0x10000 --addr --json
```

```text
      "code": null,
      "error": "Unable to load 512 bytes at r0x000101f1",
```

## Fix

- Check actual instruction bytes against the live map before emitting p-code.
  Preserve overlapbranch's complete mapped target without lifting padding.
- End unmapped fall-through with a warning and missing halt; retain other paths.
- Gate matching linked ELF x86 images with `mappedflowboundary` (default on).

## Tests

Synthetic ELF32/64 recovery, partial/overlapping instructions, queued NOPs,
declared ranges, failed reloads, overlays off/on, overrides and closed streams;
wrong-ISA/external controls.
