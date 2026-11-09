// Fixture for the Swift calling-convention arm (`swiftcall`): a class
// witness of a class-bound protocol, whose `self` is the reference itself,
// and C functions (`swift_call_native_helper.c`) that use the error
// register as an ordinary callee-saved variable.
public protocol ClassOnly: AnyObject { var v: Int { get }; func bump() }
public final class Impl: ClassOnly { public var v: Int = 1; public init() {}; public func bump() { v &+= 1 } }
@_silgen_name("native_counter") func native_counter(_ n: Int) -> Int
@_silgen_name("native_arm") func native_arm(_ n: Int, _ m: Int) -> Int
let box: ClassOnly = Impl()
box.bump()
print(native_counter(2), native_arm(3, 4), box.v)
