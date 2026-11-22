@_silgen_name("kuna_pick") func kunaPick(_ x: Int32) -> Int32
@_silgen_name("kuna_alloc") func kunaAlloc() -> UnsafeMutableRawPointer

public class Box {
    var v: Int32 = 0
    @inline(never) public func get() -> Int32 { v }
}

@inline(never)
public func later(_ k: Int32) async -> Int32 {
    await Task.yield()
    return kunaPick(k)
}

@main
struct Main {
    static func main() async {
        if CommandLine.arguments.count > 5 { _ = kunaAlloc() }
        print(await later(Int32(CommandLine.arguments.count)) + Box().get())
    }
}
