static int Sum(int[] a) { int s = 0; foreach (var x in a) s += x; return s; }
System.Console.WriteLine(Sum(new[] {1, 2, 3}));
