// Fixture for the Swift calling-convention arm (`swiftcall`) on indirect
// calls: a C function pointer a Swift function loads from a field of the
// object it is given and calls, beside a class method it dispatches through
// the object's isa.
public typealias Hook = @convention(c) (Int) -> Int
public final class Holder { public var hook: Hook; public init(hook: Hook) { self.hook = hook } }
open class Shape { public init() {}; open func area() -> Int { 1 } }
final class Square: Shape { override func area() -> Int { 4 } }
@inline(never) public func callHook(_ h: Holder, _ x: Int) -> Int { h.hook(x) &+ 1 }
@inline(never) public func measure(_ s: Shape) -> Int { s.area() &+ 2 }
print(callHook(Holder(hook: { $0 &* 5 }), CommandLine.arguments.count), measure(Square()))
