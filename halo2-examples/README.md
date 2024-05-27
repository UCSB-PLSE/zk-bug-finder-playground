# halo2-examples
Super simple example programs demonstrating basic halo2 arithmetic operations.

### To run
`cargo run --bin <binary>`

### Available binaries
- `addition`: `f(u, v) = u + v`
- `arithmetic`: `f(u, v) = u^2 + 3uv + v + 5`
    - Adapted from: https://github.com/Crazy-Cryptographic-Buddies/example-halo2-pse/tree/main
- `ite`: `f(a, b, c) = if a == b then {c} else {a - b}`
    - Adapted from: https://github.com/enricobottazzi/halo2-intro/tree/master