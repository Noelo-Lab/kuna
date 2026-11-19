// Fixture for Swift concurrency (`swiftcall`): free and member `async`
// functions, their resume partials, a checked continuation, `async let`,
// a task group, an unstructured task and an actor hop.
public actor Counter {
    public var total: Int = 0
    public init() {}
    @inline(never) public func add(_ k: Int) -> Int { total &+= k; return total }
}
open class Loader {
    public var base: Int = 3
    public init() {}
    @inline(never) open func fetch(_ k: Int) async throws -> Int {
        try await Task.sleep(nanoseconds: 1)
        return base &+ k
    }
}
@inline(never) public func resumeLater(_ k: Int) async -> Int {
    await withCheckedContinuation { (c: CheckedContinuation<Int, Never>) in c.resume(returning: k &* 2) }
}
@inline(never) public func pair(_ a: Int, _ b: Int) async -> Int {
    async let x = resumeLater(a)
    async let y = resumeLater(b)
    return await x &+ y
}
@inline(never) public func fan(_ n: Int) async -> Int {
    await withTaskGroup(of: Int.self) { group in
        for i in 0..<n { group.addTask { await resumeLater(i) } }
        var s = 0
        for await v in group { s &+= v }
        return s
    }
}
@inline(never) public func hop(_ c: Counter, _ k: Int) async -> Int { await c.add(k) }
@inline(never) public func spawn(_ k: Int) async -> Int {
    let t = Task { await resumeLater(k) }
    return await t.value
}
@main struct Main {
    static func main() async {
        let n = CommandLine.arguments.count
        let c = Counter()
        let l = Loader()
        let a = await pair(n, 2)
        let b = await fan(n)
        let d = await hop(c, n)
        let e = await spawn(n)
        let f = (try? await l.fetch(n)) ?? 0
        print(a, b, d, e, f)
    }
}
