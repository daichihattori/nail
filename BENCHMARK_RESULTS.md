# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 12 ns | 43 ns | 24 ns | - | - |
| **nail** | 1 ns | 2 ns | 4 ns | 7 ns | 16 ns |
| **num-bigint** | 39 ns | 40 ns | 21 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (7 ns)
- **1024-bit**: Fastest is **nail** (16 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 14 ns | 35 ns | 44 ns | - | - |
| **nail** | 1 ns | 2 ns | 6 ns | 38 ns | 173 ns |
| **num-bigint** | 38 ns | 33 ns | 47 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (38 ns)
- **1024-bit**: Fastest is **nail** (173 ns)


