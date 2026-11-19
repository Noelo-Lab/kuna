// Fixture for `swiftnames`: accessor, witness and runtime-helper names on
// a stripped arm64 Mach-O (see the fixtures README for the recipe).
protocol Shape { var id: Int { get set }; var area: Int { get }; func scaled(by k: Int) -> Int; var label: String { get set } }
enum Token { case word(String), number(Int), pair(String, [Int]) }
class Board {
    var width: Int = 3
    var title: String = "board"
    let depth: Int
    lazy var cells: [Int] = [width, depth]
    var span: Int { width + depth }
    var doubled: Int { get { width * 2 } set { width = newValue / 2 } }
    init(depth: Int) { self.depth = depth }
    func describe() -> String { "\(title) \(width)" }
}
class Tile: Board, Shape {
    var tag: Token = .number(1)
    var id: Int = 7
    var area: Int { width * depth }
    func scaled(by k: Int) -> Int { area * k }
    var label: String { get { title } set { title = newValue } }
    override func describe() -> String { "tile \(super.describe())" }
}
struct Dot: Shape {
    var id: Int = 2
    var x: Int
    var label: String
    var area: Int { x }
    func scaled(by k: Int) -> Int { x * k + 1 }
}
struct Span { var lo: Int; var hi: Int; var open: Bool }
@inline(never) func spans(_ n: Int) -> [Span] { [Span(lo: n, hi: n + 1, open: n > 3)] }
struct Big: Shape {
    var id: Int = 3
    var a: Int, b: Int, c: Int, d: Int
    var label: String
    var area: Int { a * b + c * d }
    func scaled(by k: Int) -> Int { area * k }
}
@inline(never) func total(_ shapes: [Shape]) -> Int { shapes.reduce(0) { $0 + $1.scaled(by: 2) + $1.label.count } }
@inline(never) func render(_ t: Token) -> String {
    switch t { case .word(let s): return s; case .number(let n): return "\(n)"; case .pair(let s, let a): return "\(s)\(a.count)" }
}
@inline(never) func keep(_ t: Token) -> [Token] { [t, t] }
@inline(never) func boxed(_ n: Int) -> Shape { if n > 2 { return Big(a: n, b: 2, c: 3, d: 4, label: "b") }; if n > 1 { return Dot(x: n, label: "d") }; return Tile(depth: n) }
let n = CommandLine.arguments.count
let t = Tile(depth: n)
t.width = n + 1; t.title = "t\(n)"; t.cells.append(n); t.doubled = 8; t.label = "L"
var s: Shape = boxed(n)
s.label = "m"
print(t.span, t.doubled, t.cells, t.describe(), total([t, Dot(x: n, label: "q"), s]), render(t.tag), keep(.pair("p", [n])).count, spans(n).count, t.id, Board(depth: 2).describe())
