# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 8 ns | 28 ns | 16 ns | - | - |
| **nail** | 1 ns | 1 ns | 2 ns | 4 ns | 10 ns |
| **num-bigint** | 29 ns | 29 ns | 15 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (4 ns)
- **1024-bit**: Fastest is **nail** (10 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 9 ns | 24 ns | 29 ns | - | - |
| **nail** | 1 ns | 1 ns | 4 ns | 28 ns | 146 ns |
| **num-bigint** | 27 ns | 25 ns | 31 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **rug-gmp** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (28 ns)
- **1024-bit**: Fastest is **nail** (146 ns)


