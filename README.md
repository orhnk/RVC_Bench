# Benchmarking

Benchmarking by its nature is not determenistic (Because of system traffic is unstable) But it gives us a brief understanding of which languages are faster than other languages. Like I said it wouldn't be right commentary saying that V is faster than Zig (etc)

BUT! I could not stand alone with my ego and now I am making a benchmark on "fast" languages.

"Benchmarking is prone to error. You can see the difference between Python and V but can't with Zig and C"

Ok. After a bit of self insultion. Lets dive into what are these languages? (briefly ofc)

**V**

Born in July 2019 and it is currently in v0.3.3 which means it is not even stable yet. (v1.0.0)
As you have to understood that I am willing to compare to other stable languages. I know I know...
That is a bit UNFARE?!?!? nobody asked.

V transpiles to human-readable C and claims to be fast as C.
Main goal of this language is to be: Fast, Safe, Simple
Wow! Till the last one I would say, Rust babe.

**C**

C has born in 1972 and it is known as "Mother" of all other languages. It was the superrior choice for operating system kernels (Mac, Windows, Linux) C allows you to acces low level hardware with human readable syntax.

**Rust**

Rust was born in 2006 and is relatively new language. It is a game changer now. Because it delivers safety, flexibility and performance at the same time. BUT!!! If V will succeed on their commitments, V will be a good language too. (now compiler and other tools are full of bugs)

**-> To se what differs in these compiers, We have to enlarge the projects to see what high level optimizations that these compilers can deliver? <-**

# Results

**Quick Sort**

Performance:

```
----------------
| Lang | Speed |
| V    | 1.476 | -> 1.48 / 1.47 / 1.48
| Rust | 1.580 | -> 1.59 / 1.58 / 1.57
| C    | 1.660 | -> 1.70 / 1.61 / 1.67
----------------
```

**More verbose and old result's are located in Rust/src/safe.rs top comments.**

**Versions**
Rustup: 1.67.0
V: 0.3.3
GCC: 12.2.1

# Try

you can try all examples inside languages folder.

just do:

**Rust**

```
cargo run --example safe --release
cargo run --example unsafe --release
```

**V**

```
v quick_sort.v -prod -no-bound-checking
```

**C**

```
gcc quick_sort.c -o quick -Ofast; ./quick
```
