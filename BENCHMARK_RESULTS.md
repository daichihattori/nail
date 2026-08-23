# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 14 ns | 46 ns | 23 ns | - | - |
| **nail** | 1 ns | 2 ns | 4 ns | 7 ns | 16 ns |
| **num-bigint** | 44 ns | 44 ns | 24 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (7 ns)
- **1024-bit**: Fastest is **nail** (16 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 15 ns | 37 ns | 48 ns | - | - |
| **nail** | 1 ns | 2 ns | 6 ns | 40 ns | 191 ns |
| **num-bigint** | 42 ns | 38 ns | 54 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (40 ns)
- **1024-bit**: Fastest is **nail** (191 ns)


