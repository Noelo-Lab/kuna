
struct Shape { virtual ~Shape(); virtual double area() const = 0; virtual int sides() const { return 0; } };
struct Circle : Shape { double r; Circle(double x): r(x) {} double area() const override { return 3.0 * r * r; } };
struct Square : Shape { double s; Square(double x): s(x) {} double area() const override { return s * s; } int sides() const override { return 4; } };
Shape::~Shape() {}
extern "C" Shape *make(int k, double v) { if (k) return new Circle(v); return new Square(v); }
extern "C" double area_of(Shape *s) { return s->area(); }
