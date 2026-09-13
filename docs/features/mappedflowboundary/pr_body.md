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
- End an unmapped fall-through, or an instruction whose bytes run past the
  mapped image, with a warning and missing halt; retain other paths. A
  truncated entry instruction, mapped read failures and in-lined callees keep
  their errors.
- Gate matching linked ELF x86 images with `mappedflowboundary` (default on).

## Tests

Synthetic ELF32/64 recovery for a clean segment end and for trailing zero
padding, partial/overlapping instructions, queued NOPs, declared ranges, failed
reloads, overlays off/on, overrides, in-lining and closed streams;
wrong-ISA/external controls.
