// This program demonstrates the enumerability of the set N^2
// (the set of ordered pairs of natural numbers)
//
// If a set S is enumerable, then there exists a bijective function f: N -> S
//
// Define f recursively as follows:
// f(n) = 
//  if n == 0 then (0,0)
//  else successor(f(n-1))
//
//  where successor is a function mapping N^2 -> N^2,
//  defined in OrderedPairGenerator::next below.


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

// Alternatively, we can define two functions fst(n) and snd(n),
// which give the first and second parts of the nth ordered pair.
fn fst(n: u32) -> u32 {
    let mut i = 0;
    let mut inc = 1;
    while i < n {
        i += inc;
        inc += 1;
    }
    if i == n { 0 }
    else {
        i -= inc-1;
        n - i
    }
}
fn snd(n: u32) -> u32 {
    let mut i = 0;
    let mut inc = 2;
    while i < n {
        i += inc;
        inc += 1;
    }
    i - n
}

fn main() {
    OrderedPairGenerator::new()
        .take(10)
        .for_each(|p|println!("{:?}", p));

    println!();
    for n in 0..10 {
        println!("({}, {})", fst(n), snd(n));
    }
}
/*
Output:
(0, 1) // generator skips (0,0)
(1, 0)
(0, 2)
(1, 1)
(2, 0)
(0, 3)
(1, 2)
(2, 1)
(3, 0)
(0, 4)

(0, 0)
(0, 1)
(1, 0)
(0, 2)
(1, 1)
(2, 0)
(0, 3)
(1, 2)
(2, 1)
(3, 0)
*/

