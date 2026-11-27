import struct
import unittest

from function_oracle import Case, UnicornBackend

CODE = 0x400000
SENTINEL = 0x600000
STACK = 0x700000
RSP = STACK + 0x808
DATA = 0x800000


def request(code, **registers):
    return {
        "version": 1,
        "architecture": "x86-64",
        "abi": "sysv-amd64",
        "entry": hex(CODE),
        "return_address": hex(SENTINEL),
        "registers": {"RSP": hex(RSP), **{r: hex(v) for r, v in registers.items()}},
        "memory": [
            {"address": hex(CODE), "size": 4096, "permissions": "rx", "bytes": code},
            {"address": hex(STACK), "size": 4096, "permissions": "rw",
             "bytes": (bytes(RSP - STACK) + struct.pack("<Q", SENTINEL)).hex()},
            {"address": hex(DATA), "size": 4096, "permissions": "rw", "observe": True},
        ],
    }


class FunctionOracleTests(unittest.TestCase):
    def execute(self, spec):
        return UnicornBackend().run(Case.parse(spec))

    def test_arguments_return_and_memory(self):
        result = self.execute(request("4889f84801f0488902c3", RDI=7, RSI=12, RDX=DATA))
        self.assertEqual(result["status"], "returned")
        self.assertEqual(result["registers"]["RAX"], "0x13")
        self.assertEqual(result["abi_violations"], [])
        self.assertEqual(result["instructions"], 4)
        self.assertEqual(bytes.fromhex(result["memory"][0]["bytes"])[:8], struct.pack("<Q", 19))

    def test_internal_call_and_root_return(self):
        result = self.execute(request("4883ec08e8050000004883c408c3488d4701c3", RDI=5))
        self.assertEqual(result["status"], "returned")
        self.assertEqual(result["registers"]["RAX"], "0x6")
        self.assertEqual(result["abi_violations"], [])

    def test_indirect_return_is_not_assumed_from_the_final_pc(self):
        result = self.execute(request("e9fbff1f00"))
        self.assertEqual(result["pc"], hex(SENTINEL))
        self.assertEqual(result["reason"], "non-return-transfer-to-sentinel")

    def test_abi_preservation_and_stack_cleanup(self):
        result = self.execute(request("bb01000000c20800", RBX=2))
        self.assertEqual(result["status"], "returned")
        self.assertEqual(result["abi_violations"], ["RBX", "RSP"])

    def test_vector_write_uses_all_sixteen_bytes(self):
        value = int.from_bytes(bytes(range(16)), "little")
        result = self.execute(request("f30f7f02c3", RDX=DATA, XMM0=value))
        self.assertEqual(result["status"], "returned")
        self.assertEqual(bytes.fromhex(result["memory"][0]["bytes"])[:16], bytes(range(16)))

    def test_x87_state_is_initialized_and_reported(self):
        result = self.execute(request("d9e8c3"))
        self.assertEqual(result["status"], "returned")
        top = (int(result["registers"]["FPSW"], 16) >> 11) & 7
        self.assertEqual(int(result["registers"][f"FP{top}"], 16), (0x3fff << 64) | (1 << 63))

    def test_instruction_limit_is_incomplete(self):
        spec = request("ebfe")
        spec["limits"] = {"instructions": 10}
        result = self.execute(spec)
        self.assertEqual((result["status"], result["reason"], result["instructions"]), ("incomplete", "instruction-limit", 10))

    def test_timestamp_requires_an_environment_model(self):
        result = self.execute(request("0f31c3"))
        self.assertEqual((result["status"], result["reason"]), ("incomplete", "timestamp-counter"))
        self.assertEqual(result["event"]["address"], hex(CODE))

    def test_trap_and_unmapped_data_access_are_distinct(self):
        self.assertEqual(self.execute(request("0f0b"))["status"], "trap")
        result = self.execute(request("488b07c3", RDI=0x810000))
        self.assertEqual(result["status"], "fault")
        self.assertEqual(result["event"]["address"], "0x810000")

    def test_external_entry_records_arguments_without_executing_the_target(self):
        spec = request("e8fbff0f00c3", RDI=5, RDX=11)
        spec["external_entries"] = ["0x500000"]
        result = self.execute(spec)
        event = result["event"]
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(event["target"], "0x500000")
        self.assertEqual(event["from"], hex(CODE))
        self.assertEqual(event["registers"]["RDI"], "0x5")
        self.assertEqual(event["registers"]["RDX"], "0xb")

    def test_syscall_stops_before_an_environment_effect(self):
        result = self.execute(request("0f05c3", RAX=1, R10=4))
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(result["event"]["kind"], "system-call")
        self.assertEqual(result["event"]["registers"]["RAX"], "0x1")
        self.assertEqual(result["event"]["registers"]["R10"], "0x4")

    def test_rejects_conflicting_mappings(self):
        spec = request("c3")
        spec["memory"].append(spec["memory"][0].copy())
        with self.assertRaisesRegex(ValueError, "overlap"):
            Case.parse(spec)


if __name__ == "__main__":
    unittest.main()
