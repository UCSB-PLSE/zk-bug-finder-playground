# halo2-examples
Super simple example programs demonstrating basic halo2 arithmetic operations.

### To run
`cargo run --bin <binary>`

### Available binaries
- `addition`: Computes `f(u, v) = u + v`
- `arithmetic`: Computes `f(u, v) = u^2 + 3uv + v + 5`
    - Adapted from: https://github.com/Crazy-Cryptographic-Buddies/example-halo2-pse/tree/main
- `iszero`: Computes `f(v) = if v == 0 then 0 else {1 / v}`
- `ite`: Computes `f(a, b, c) = if a == b then {c} else {a - b}`
    - Adapted from: https://github.com/enricobottazzi/halo2-intro/tree/master