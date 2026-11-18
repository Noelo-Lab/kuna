// Fixture for the Swift calling-convention arm (`swiftcall`): callers of a
// throwing Swift function (`swift_call_spill_helper.c`) that spill the
// error register after the call and test the value they reload.
enum Oops: Error { case bad }
@_silgen_name("risky") public func risky(_ x: Int) throws -> Int { if x > 3 { throw Oops.bad }; return x &* 2 }
@_silgen_name("spill_test") func spill_test(_ n: Int) -> Int
@_silgen_name("spill_moved") func spill_moved(_ n: Int) -> Int
print(spill_test(CommandLine.arguments.count), spill_moved(CommandLine.arguments.count))
