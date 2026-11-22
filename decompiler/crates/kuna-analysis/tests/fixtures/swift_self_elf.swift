// swiftc -O -module-name selffix -parse-as-library swift_self_elf.swift
@inline(never) public func opaque() -> Int { return Int(bitPattern: UInt(clock_tick())) }
@_silgen_name("clock") func clock_tick() -> Int

public final class Box { public var v = 0 }

public func makeReader(_ b: Box) -> () -> Int {
    return { opaque() &+ b.v }
}

@inline(never) public func label(_ n: Int) -> String {
    var s = String(n)
    s.append(" s")
    return s
}

@main struct Main {
    static func main() {
        let r = makeReader(Box())
        print(r(), label(CommandLine.arguments.count))
    }
}
