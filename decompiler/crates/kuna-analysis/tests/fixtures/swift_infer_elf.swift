// Functions the Swift metadata does not bind (`tests/stages/kuna-swiftinfer.xml`).
public final class Box {
    public var value: Int
    public init(_ v: Int) { value = v }
}

struct Failure: Error {}

// An escaping closure: its partial-apply forwarder reads the context in r13.
@inline(never)
public func makeAdder(_ b: Box) -> () -> Int {
    return { b.value &+ 1 }
}

// A two-word result whose halves the caller reads apart.
@inline(never)
public func split(_ x: Int) -> (Int, Int) {
    return (x &+ 1, x &* 3)
}

public func usePair(_ x: Int) -> Int {
    let (a, b) = split(x)
    return a &- b
}

@inline(never)
public func mayThrow(_ x: Int) throws -> Int {
    if x == 0 { throw Failure() }
    return x &* 2
}

@inline(never)
public func bump(_ x: Int) -> Int {
    return x &* 5 &+ 1
}

// Hands its own error register on to mayThrow and tests it after.
@inline(never)
public func forward(_ x: Int) throws -> Int {
    let y = try mayThrow(x)
    return bump(y)
}

@main
struct Main {
    static func main() {
        let f = makeAdder(Box(Int(CommandLine.arguments.count)))
        print(f(), usePair(CommandLine.arguments.count))
        print((try? forward(CommandLine.arguments.count - 1)) ?? -1)
    }
}
