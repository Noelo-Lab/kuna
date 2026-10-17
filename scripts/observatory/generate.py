"""Drive the observatory generators and render the pages.

    python -m scripts.observatory.generate [--out DIR] [--only a,b] [--render-only]

Each generator writes observatory/data/<name>.json; render() then rebuilds
every page from whatever datasets exist, so --only/--render-only refresh
cheaply. docs/observatory.md is the user-facing reference."""
import argparse
from pathlib import Path

from scripts.observatory import atlas, common, render

GENERATORS = {
    "atlas": atlas.generate,
}


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(prog="python -m scripts.observatory.generate")
    parser.add_argument("--out", type=Path, default=common.repo_root() / "observatory")
    parser.add_argument("--only", help="comma-separated generator names", default=None)
    parser.add_argument("--render-only", action="store_true", help="rebuild pages from existing JSON")
    parser.add_argument("--kuna", type=Path, help="built kuna binary (used by the walkthrough)")
    args = parser.parse_args(argv)

    names = [] if args.render_only else list(GENERATORS)
    if args.only is not None:
        names = [name.strip() for name in args.only.split(",") if name.strip()]
        unknown = [name for name in names if name not in GENERATORS]
        if unknown:
            parser.error(f"unknown generator(s): {', '.join(unknown)} (have: {', '.join(GENERATORS)})")

    for name in names:
        data = GENERATORS[name](root=common.repo_root())
        target = args.out / "data" / f"{name}.json"
        common.write_json(target, data)
        print(f"observatory: {name} -> {target}")

    pages = render.render(args.out)
    print(f"observatory: rendered {len(pages)} page(s) -> {args.out}/")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
