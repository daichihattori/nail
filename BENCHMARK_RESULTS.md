# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 7 ns | 26 ns | 20 ns | - | - |
| **nail** | 1 ns | 1 ns | 2 ns | 4 ns | 10 ns |
| **num-bigint** | 28 ns | 27 ns | 17 ns | - | - |
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
| **malachite** | 8 ns | 22 ns | 27 ns | - | - |
| **nail** | 1 ns | 1 ns | 3 ns | 17 ns | 88 ns |
| **num-bigint** | 29 ns | 23 ns | 31 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (17 ns)
- **1024-bit**: Fastest is **nail** (88 ns)


