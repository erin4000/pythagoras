# Efficient ordered generation of primitive Pythagorean triples

Efficient generation of primitive pythagorean triples in ascending order.

A **Pythagorean triple** is a triple of positive integers $(a,b,c)$ such that $a^2+b^2=c^2$. A **primitive** Pythagorean triple is a triple such that $(a,b,c)$ have no common factor between them.

The function `pythagorean_triples()` returns an infinite iterator that enumerates all primitive triples $a<b<c$ in order of increasing $c$, followed by increasing $a$. For example, $(20,21,29)$ will come before $(12,35,37)$ as 29 < 37, while $(16, 63, 65)$ will come before $(33, 56, 65)$ as $16 < 33$.

Example usage:

```rs
// Prints the following:
// 3² + 4² = 5²
// 5² + 12² = 13²
// 8² + 15² = 17²
// 7² + 24² = 25²
// 20² + 21² = 29²
// 12² + 35² = 37²
// 9² + 40² = 41²
// 28² + 45² = 53²
// 11² + 60² = 61²
// 16² + 63² = 65²
// 33² + 56² = 65²
// 48² + 55² = 73²
// 13² + 84² = 85²
// 36² + 77² = 85²
// 39² + 80² = 89²
// 65² + 72² = 97²
for (a, b, c) in pythagorean_triples().take_while(|(_,_,c)| *c <= 100) {
    println!("{}² + {}² = {}²", a, b, c);
}

// For any primitive triple (a,b,c), a*b*c is a multiple of 60:
assert!(pythagorean_triples().take(1_000).all(|(a,b,c)| a*b*c % 60 == 0));
```

## Implementation

Triples are generated efficiently by using a [tree of primitive Pythagorean triples](https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples): an infinite ternary tree guaranteed to contain every primitive triple once. Though there is no clear pattern for obtaining the triples in order of increasing hypotenuse, the tree does satisfy the property that the hypotenuse of any triple is strictly less than the hypotenuse of any of its three children.

This suggests a straightfoward algorithm: explore the tree, placing triples in a priority queue, and each time the next triple is removed, adding its three child elements to the priority queue. This is efficient ($O(\log n)$ using a heap data structure) and uses memory linear in the number of triples polled (every poll results in one element being removed from the heap and three being added).