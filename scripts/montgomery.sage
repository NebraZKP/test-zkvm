#!/usr/bin/env sage --python3

Q = 21888242871839275222246405745257275088696311157297823662689037894645226208583

N = Q
R_BITS=254
R = 1 << R_BITS
assert(R & (R-1) == 0)

R_MASK = R - 1

(g, Np, Rp) = xgcd(N, R)
if Np < 0:
    Np = -Np
else:
    Np = R - Np

assert(g == 1)
assert(Np * N % R == R - 1)

def to_montgomery(v: int) -> int:
    return (v * R) % N

def reduce(T: int) -> int:
    assert(T < N*R)
    t0 = T & R_MASK
    assert(t0 < R)
    t0Np = t0 * Np
    t = t0Np & R_MASK
    assert(t < R)

    T_plus_tN = T + t*N
    assert(T_plus_tN & R_MASK == 0)
    T_plus_tN_over_R = T_plus_tN >> R_BITS

    assert(0 <= T_plus_tN_over_R < 2*N)
    if T_plus_tN_over_R > N:
        return T_plus_tN_over_R - N
    return T_plus_tN_over_R

def from_montgomery(m: int) -> int:
    return reduce(m)
