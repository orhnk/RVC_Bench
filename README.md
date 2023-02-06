# Benchmarking

Benchmarking by its nature is not determenistic (Because of system traffic is unstable) But it gives us a brief understanding of which languages are faster than other languages. Like I said it wouldn't be right commentary saying that V is faster than Zig (etc)

BUT! I could not stand alone with my ego and now I am making a benchmark on "fast" languages.

"Benchmarking is prone to error. You can see the difference between Python and V but can't with C"

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
| Rust | 1.703 | -> 1.70 / 1.71 / 1.70
| V    | 2.013 | -> 1.98 / 2.00 / 2.06
| C    | 2.196 | -> 2.20 / 2.21 / 2.18
----------------
```

**More verbose Result (more detailed) can be found in Rust/src/main.rscomment section (top)**

**Versions**
Rustup: 1.66.0
V: 0.3.3
GCC: 12.2.1

# Try

you can try all examples inside languages folder.
I used cargo tool to make it simpler to constuct files:
just do:

```
cargo run --example safe --release
cargo run --example unsafe --release
```
