"""Compare a saved kuna-verify oracle-case and its Unicorn observation."""

import argparse
import json
from dataclasses import dataclass
from itertools import combinations

from oracle_protocol import DEFAULT_LIMITS, MAX_MAPPED, case_digest, load_request

BACKENDS = ("unicorn", "raw-pcode", "final-ir")
PRESERVED = ("RBX", "RBP", "R12", "R13", "R14", "R15", "RSP")
CALL_ARGUMENTS = ("RDI", "RSI", "RDX", "RCX", "R8", "R9")
INTEGER_RETURN_NAMES = {
    "RAX": {1: "AL", 2: "AX", 4: "EAX", 8: "RAX"},
    "RDX": {1: "DL", 2: "DX", 4: "EDX", 8: "RDX"},
}


@dataclass(frozen=True)
class Return:
    register: str
    size: int
    ir_storage: str

    @classmethod
    def parse(cls, text):
        if text == "void":
            return cls("", 0, "")
        reg, colon, width = text.partition(":")
        if not colon or width not in ("1", "2", "4", "8", "16"):
            raise ValueError("return must be void or REGISTER:BYTES")
        size = int(width)
        if reg in INTEGER_RETURN_NAMES:
            storage = INTEGER_RETURN_NAMES[reg].get(size)
        elif reg in ("XMM0", "XMM1"):
            storage = {4: reg + "_Da", 8: reg + "_Qa", 16: reg}.get(size)
        else:
            storage = None
        if storage is None:
            raise ValueError("unsupported return register or width")
        return cls(reg, size, storage)


def named_observations(capture, unicorn):
    observations = capture["observations"]
    results = dict(zip(BACKENDS, (unicorn, observations["raw"], observations["ir"])))
    if any(obs["backend"] != name for name, obs in results.items()):
        raise ValueError("observation backend labels do not match the envelope")
    return results


def differences(values):
    if len(values) < 2 or len(set(values.values())) == 1:
        return None
    candidates = {
        "unicorn": ["shared-lift-or-pcode-semantics", "isa-or-environment-model"],
        "raw-pcode": ["raw-pcode-execution", "machine-abi-model"],
        "final-ir": ["decompiler-transformation", "final-ir-execution", "recovered-abi"],
    }
    outlier = next((side for side, value in values.items() if sum(v == value for v in values.values()) == 1), None)
    plausible = (
        candidates[outlier] if len(values) == 3 and len(set(values.values())) == 2
        else ["unresolved-backend-disagreement"]
    )
    return {"values": values, "candidate_causes": plausible}


def add_difference(findings, observable, values, **location):
    difference = differences(values)
    if difference:
        findings.append({"observable": observable, **location, **difference})


def watched_regions(case):
    watched = {}
    total = 0
    for region in case["memory"]:
        if not region.get("observe"):
            continue
        at, size = int(region["address"], 16), region["size"]
        if type(size) is not int or size <= 0 or at in watched:
            raise ValueError("invalid observed memory region in case")
        total += size
        if total > MAX_MAPPED:
            raise ValueError("observed memory exceeds the case mapping limit")
        watched[at] = size
    return watched


def memory_snapshot(observation, watched):
    snapshot = {}
    for region in observation["memory"]:
        at = int(region["address"], 16)
        if at in snapshot or at not in watched or len(region["bytes"]) != 2 * watched[at]:
            raise ValueError("observation memory does not match the declared regions")
        data = bytes.fromhex(region["bytes"])
        if len(data) != watched[at]:
            raise ValueError("observation memory contains an invalid hex byte")
        snapshot[at] = data
    if set(snapshot) != set(watched):
        raise ValueError("observation omits declared memory regions")
    return snapshot


def compare_memory(findings, case, observations):
    watched = watched_regions(case)
    snapshots = {name: memory_snapshot(obs, watched) for name, obs in observations.items()}
    for at in sorted(watched):
        names = tuple(snapshots)
        regions = [snapshot[at] for snapshot in snapshots.values()]
        if len(set(regions)) <= 1:
            continue
        # One witness for each combination of agreeing backends in the region.
        patterns = set()
        for off, byte_values in enumerate(zip(*regions)):
            if len(set(byte_values)) == 1:
                continue
            pattern = tuple(lhs == rhs for lhs, rhs in combinations(byte_values, 2))
            if not all(pattern) and pattern not in patterns:
                patterns.add(pattern)
                values = dict(zip(names, map(hex, byte_values)))
                add_difference(findings, "memory-byte", values, address=hex(at + off))
                if len(patterns) == (1 if len(names) == 2 else 4):
                    break


def compare_return(findings, returned, expected, limitations):
    if not expected.size:
        if "final-ir" in returned and returned["final-ir"].get("return") is not None:
            limitations.append("final-ir-return-declaration-differs-from-void-scope")
        return
    values = {}
    mask = (1 << (expected.size * 8)) - 1
    for name, obs in returned.items():
        if name == "final-ir":
            declared = obs.get("return")
            if not declared or declared["storage"] != expected.ir_storage or declared["size"] != expected.size:
                limitations.append("final-ir-return-storage-outside-projection")
                continue
            value = int(declared["value"], 16)
            if not 0 <= value <= mask:
                raise ValueError("final IR return exceeds the reported width")
        else:
            value = int(obs["registers"][expected.register], 16) & mask
        values[name] = hex(value)
    add_difference(findings, "return", values, register=expected.register, bytes=expected.size)


def syscall_prefix(observations):
    unicorn, raw, ir = [observations[name] for name in BACKENDS]
    event = unicorn.get("event")
    if unicorn["status"] != "incomplete" or unicorn.get("reason") != "system-call" or not event:
        return None
    for obs in (raw, ir):
        trace = obs.get("events", [])
        if obs["status"] != "incomplete" or obs.get("calls") or len(trace) != 1:
            return None
        if trace[0]["name"] != "syscall" or trace[0]["address"] != event["address"]:
            return None
    return event


def external_call_prefix(findings, case, observations):
    unicorn, raw, ir = (observations[name] for name in BACKENDS)
    event = unicorn.get("event")
    if unicorn["status"] != "incomplete" or unicorn.get("reason") != "external-entry" or not event:
        return False
    signatures = case.get("external_calls", {})
    at = event["from"]
    targets = {"unicorn": event["target"]}
    for name, obs in (("raw-pcode", raw), ("final-ir", ir)):
        calls, events = obs.get("calls", []), obs.get("events", [])
        if obs["status"] != "incomplete" or len(calls) != 1 or len(events) != 1:
            return False
        if calls[0]["address"] != at or events[0]["address"] != at or events[0]["name"] != "external-call":
            return False
        targets[name] = calls[0]["target"]
    if any(target not in signatures for target in targets.values()):
        raise ValueError("external call is outside the declared signatures")

    compare_memory(findings, case, observations)
    add_difference(findings, "external-call-target", targets, address=at)
    if len(set(targets.values())) == 1:
        arity = signatures[event["target"]]
        if type(arity) is not int or not 0 <= arity <= len(CALL_ARGUMENTS):
            raise ValueError("invalid external call arity")
        args = ir["calls"][0]["arguments"]
        add_difference(findings, "external-call-arity", {
            "unicorn": arity, "raw-pcode": arity, "final-ir": len(args),
        }, address=at)
        for i, register in enumerate(CALL_ARGUMENTS[:arity]):
            values = {
                "unicorn": hex(int(event["registers"][register], 16)),
                "raw-pcode": hex(int(raw["registers"][register], 16)),
            }
            if i < len(args):
                values["final-ir"] = hex(int(args[i], 16))
            add_difference(findings, "external-call-argument", values, address=at, index=i, register=register)
    return True


def compare(capture, unicorn, expected):
    case = capture["case"]
    if (capture["version"], case["version"], unicorn["version"]) != (1, 1, 1):
        raise ValueError("unsupported oracle format version")
    if (case["architecture"], case["abi"]) != ("x86-64", "sysv-amd64"):
        raise ValueError("case is outside the x86-64 SysV comparison model")
    digest = case_digest(case)
    if unicorn.get("case_sha256") != digest or unicorn.get("cpu_model") != case["cpu_model"]:
        raise ValueError("Unicorn observation does not identify the same case and CPU model")
    observations = named_observations(capture, unicorn)
    statuses = {name: obs["status"] for name, obs in observations.items()}
    returned = {name: obs for name, obs in observations.items() if obs["status"] == "returned"}
    findings, limitations = [], []
    if len(returned) != 3:
        limitations.append("one-or-more-backends-did-not-complete")
    terminal = {
        name: "returned" if status == "returned" else "fault-or-trap"
        for name, status in statuses.items() if status in ("returned", "fault", "trap")
    }
    add_difference(findings, "termination", terminal)
    compare_return(findings, returned, expected, limitations)
    if len(returned) >= 2:
        compare_memory(findings, case, returned)
    syscall = syscall_prefix(observations)
    if syscall:
        compare_memory(findings, case, observations)
        number = {"unicorn": hex(int(syscall["registers"]["RAX"], 16))}
        number["raw-pcode"] = hex(int(observations["raw-pcode"]["registers"]["RAX"], 16))
        inputs = observations["final-ir"]["events"][0]["inputs"]
        if inputs:
            number["final-ir"] = hex(int(inputs[0], 16))
        add_difference(findings, "syscall-number", number, address=syscall["address"])
        limitations.append("system-call-effects-and-argument-arity-unmodeled")
    external = external_call_prefix(findings, case, observations)
    if external:
        limitations.append("external-call-effects-unmodeled")
    physical = {name: obs for name, obs in returned.items() if name != "final-ir"}
    if len(physical) == 2:
        for register in PRESERVED:
            values = {
                name: hex(int(obs["registers"][register], 16)) for name, obs in physical.items()
            }
            add_difference(findings, "abi-preserved-register", values, register=register)
    if any(obs["abi_violations"] for obs in physical.values()):
        limitations.append("machine-execution-violated-abi")
    if any(obs.get("raw_callee_effects_available") for obs in observations.values()):
        limitations.append("raw-callee-effects-available-to-final-ir")

    confirmed = any("unicorn" in finding["values"] for finding in findings)
    classification = (
        "COUNTEREXAMPLE" if confirmed else
        "INCONCLUSIVE" if findings or limitations else
        "CROSS_BACKEND_AGREEMENT"
    )
    return {
        "version": 1,
        "function": capture["function"],
        "classification": classification,
        "case_sha256": digest,
        "entry": case["entry"],
        "decompile_options": capture.get("decompile_options", []),
        "provenance": capture.get("provenance", {}),
        "coverage": {
            "return": "void" if not expected.size else f"{expected.register}:{expected.size}",
            "declared_memory_bytes": sum(watched_regions(case).values()),
            "memory_backends": list(observations) if syscall or external else (list(returned) if len(returned) >= 2 else []),
            "physical_abi_backends": list(physical) if len(physical) == 2 else [],
            "boundary": (
                "external-call-prefix" if external else "syscall-prefix" if syscall else
                "function-return" if returned else None
            ),
            "full_return_comparison": len(returned) == 3 and not any(
                limitation.startswith("final-ir-return") for limitation in limitations
            ),
            "backend_status": statuses,
        },
        "backends": {
            "unicorn": unicorn["backend_version"],
            "raw-pcode": "Kuna SLEIGH/p-code",
            "final-ir": "Kuna final IR/p-code",
        },
        "assumptions": [case["abi"], case["cpu_model"], "raw-pcode-and-final-ir-share-sleigh-and-pcode-semantics"],
        "limits": {"pcode_budget": capture["pcode_budget"], "unicorn": {**DEFAULT_LIMITS, **case.get("limits", {})}},
        "findings": findings,
        "limitations": limitations,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", help="JSON envelope from kuna-verify oracle-case")
    parser.add_argument("unicorn", help="JSON observation from function_oracle.py")
    parser.add_argument("--return", dest="return_spec", required=True, help="void or REGISTER:BYTES")
    args = parser.parse_args()
    try:
        expected = Return.parse(args.return_spec)
        with open(args.capture, "rb") as file:
            capture = load_request(file)
        with open(args.unicorn, "rb") as file:
            result = compare(capture, load_request(file), expected)
    except (AttributeError, KeyError, TypeError, ValueError, OSError) as error:
        parser.exit(2, f"oracle comparison: {error}\n")
    print(json.dumps(result, sort_keys=True))
    if result["classification"] == "COUNTEREXAMPLE":
        parser.exit(1)


if __name__ == "__main__":
    main()
