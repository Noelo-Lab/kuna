// Fixture for a Swift function that returns its String in the register
// pair, one literal per branch (`swiftcall`, the returned pair). Stripped:
// the witness is found through the conformance, not a symbol.
enum Key: CustomStringConvertible {
    case cursor, next, goal
    var description: String {
        switch self {
        case .cursor: return "endCursor"
        case .next: return "hasNextPage"
        case .goal: return "goal_id"
        }
    }
}
@inline(never) func show(_ k: Key) -> String { String(describing: k) }
public func run(_ i: Int) -> String { show(i == 0 ? .cursor : i == 1 ? .next : .goal) }
