use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Boilerplate to create a custom sort.
#[derive(PartialEq, Eq)]
struct WrappedTriple(i64, i64, i64);

impl Ord for WrappedTriple {
    /// Ordering is reversed so that the binary heap acts as a min-heap
    /// rather than a max-heap. If `WrappedTriple` were used anywhere
    /// else, it would be better to define this the right way around
    /// and use `std::cmp::Reverse` in the binary heap.
    fn cmp(&self, other: &Self) -> Ordering {
        (other.2, other.0).cmp(&(self.2, self.0))
    }
}

impl PartialOrd for WrappedTriple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const INITIAL_PYTHAGOREAN_TRIPLE: WrappedTriple = WrappedTriple(3, 4, 5);
const PYTHAGOREAN_MATRICES:
    [((i64, i64, i64), (i64, i64, i64), (i64, i64, i64)); 3]
= [
    (( 1, -2, 2), ( 2, -1, 2), ( 2, -2, 3)),
    (( 1,  2, 2), ( 2,  1, 2), ( 2,  2, 3)),
    ((-1,  2, 2), (-2,  1, 2), (-2,  2, 3)),
];

/// Iterator over Pythagorean triples in ascending order.
pub struct PythagoreanTripleIter {
    min_heap: BinaryHeap<WrappedTriple>,
}

impl Iterator for PythagoreanTripleIter {
    type Item = (i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        // Cannot panic as `min_heap` is never empty; it is initialized
        // with one triple, then three more elements are added every
        // time a triple is removed.
        let WrappedTriple(j, k, l) = self.min_heap.pop().unwrap();
        for ((a, b, c), (d, e, f), (g, h, i)) in PYTHAGOREAN_MATRICES {
            let m = a*j + b*k + c*l;
            let n = d*j + e*k + f*l;
            let o = g*j + h*k + i*l;
            self.min_heap.push(WrappedTriple(m.min(n), m.max(n), o));
        }

        Some((j, k, l))
    }
}

/// Produces all primitive Pythagorean triples in sorted order.
/// That is all coprime triples (a < b < c) such that a^2 + b^2 = c^2,
/// ordered by increasing c, then a, then b.
pub fn pythagorean_triples() -> PythagoreanTripleIter {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(INITIAL_PYTHAGOREAN_TRIPLE);
    PythagoreanTripleIter { min_heap }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ascending() {
        let mut triple_generator = pythagorean_triples();
        let mut previous = triple_generator.next().unwrap();
        for next@(x, _, z) in triple_generator.take(999_999) {
            let (a, _, c) = previous;
            assert!((c, a) < (z, x));

            previous = next;
        }
    }
}