// Fixture for the Swift calling-convention arm (`swiftcall`): a mutating
// struct method whose signature has no declared form (its argument is an
// enum) called after a Swift function of the module, with a loop, that runs
// between the caller setting the swiftself register and the call.
public enum Step { case by(Int), none }
public final class Box { public var n = 0; public init() {} }
public struct Counter {
    public var a: Int
    public var b: Box
    @inline(never) public mutating func bump(_ s: Step) { if case .by(let k) = s { a &+= k &+ b.n } }
}
@inline(never) public func spin(_ x: Int) -> Int { var s = 0; var i = 0; while i < x { s &+= i &* 3; i &+= 1 }; return s }
@inline(never) public func once(_ c: inout Counter, _ x: Int) -> Int { let t = spin(x); c.bump(.by(t)); return t }
var c0 = Counter(a: CommandLine.arguments.count, b: Box())
print(once(&c0, 5)); print(c0.a)
