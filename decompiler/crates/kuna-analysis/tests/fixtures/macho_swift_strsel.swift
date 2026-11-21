// Swift string literals chosen by branches and conditional selects: each
// literal's two words (count and object) must be paired along one path.
enum Field {
    case items, date, provider, odometer
}

extension Field: Hashable {
    @inline(never)
    func hash(into h: inout Hasher) {
        let s: String
        switch self {
        case .items: s = "serviceItems"
        case .date: s = "serviceDate"
        case .provider: s = "serviceProvider"
        case .odometer: s = "odoMeter"
        }
        h.combine(s)
    }
}

@inline(never)
func pick(_ a: Bool, _ b: Bool) -> Int {
    var h = Hasher()
    h.combine(a ? "serviceItems" : "serviceDate")
    h.combine(b ? "shape" : "animal")
    return h.finalize()
}

final class Box {
    var label: String = "none"
}

@inline(never)
func tag(_ b: Box, _ on: Bool) {
    b.label = on ? "enabled" : "disabled"
}

@inline(never)
func blank(_ b: Box) {
    b.label = ""
}

let all: [Field] = [.items, .date, .provider, .odometer]
var total = 0
for f in all {
    total &+= f.hashValue
}
total &+= pick(total & 1 == 0, total & 2 == 0)
let box = Box()
tag(box, total & 4 == 0)
print(box.label)
blank(box)
print(box.label, total != 0)
