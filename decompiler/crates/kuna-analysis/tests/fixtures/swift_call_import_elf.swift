// A throwing function of this module that calls throwing functions of
// another image (the standard library's `Decoder` and
// `KeyedDecodingContainer`) and hands its own error register on to them.
public struct Pair: Decodable {
    public var a: Int
    public var b: Int
    enum CodingKeys: String, CodingKey { case a, b }

    public init(from decoder: Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        a = try c.decode(Int.self, forKey: .a)
        b = (try? c.decode(Int.self, forKey: .b)) ?? 7
    }
}
