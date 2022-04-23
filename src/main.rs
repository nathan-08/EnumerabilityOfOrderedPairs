// Proof of enumerability of N^2
//
// This program demonstrates the enumerability of the set N^2
// (the set of ordered pairs of natural numbers)
//
// If a set is enumerable then there exists a function f: N -> N^2
// which is a bijection (one-to-one and onto).
//
// Define f recursively as follows:
// f(n) = 
//  if n == 0 then (0,0)
//  else successor(f(n-1))
//
//  where successor is a function mapping N^2 -> N^2
//  defined in the OrderedPairGenerator below.


struct OrderedPairGenerator {
    p: (u32, u32),
}
impl OrderedPairGenerator {
    fn new() -> OrderedPairGenerator {
        OrderedPairGenerator{ p: (0,0) }
    }
    fn newp(p: (u32, u32)) -> OrderedPairGenerator {
        OrderedPairGenerator{ p }
    }
}
impl Iterator for OrderedPairGenerator {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        match self.p {
            (_, 0) => self.p = (0, self.p.0+1),
            _ => self.p = (self.p.0+1, self.p.1-1),
        };
        Some(self.p)
    }
}


fn main() {
    OrderedPairGenerator::new()
        .take(10)
        .for_each(|p|println!("{:?}", p));
}

