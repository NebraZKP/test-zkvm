
def u256_from_u64s(u64s: [int]) -> int:
    assert(len(u64s) == 4)
    return u64s[0] + (u64s[1] << 64) + (u64s[2] << 128) + (u64s[3] << 192)

def u256_to_u64s(v: int) -> [int]:
    return [
        v          & 0xffffffffffffffff,
        (v >> 64)  & 0xffffffffffffffff,
        (v >> 128) & 0xffffffffffffffff,
        (v >> 192) & 0xffffffffffffffff,
    ]

def u256_from_u32s(u32s: [int]) -> int:
    assert(len(u32s) == 8)
    return u32s[0] + (u32s[1] << 32) + (u32s[2] << 64) + (u32s[3] << 96) + \
        ( u32s[4] << 128) + (u32s[5] << 160) + (u32s[6] << 192) + (u32s[7] << 224)

def u256_to_u32s(v: int) -> [int]:
    return [
        v          & 0xffffffff,
        (v >> 32)  & 0xffffffff,
        (v >> 64)  & 0xffffffff,
        (v >> 96)  & 0xffffffff,
        (v >> 128) & 0xffffffff,
        (v >> 160) & 0xffffffff,
        (v >> 192) & 0xffffffff,
        (v >> 224) & 0xffffffff,
    ]
