# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 11 ns | 38 ns | 24 ns | - | - |
| **nail** | 1 ns | 2 ns | 3 ns | 6 ns | 13 ns |
| **num-bigint** | 43 ns | 42 ns | 21 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (6 ns)
- **1024-bit**: Fastest is **nail** (13 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 11 ns | 31 ns | 42 ns | - | - |
| **nail** | 1 ns | 2 ns | 5 ns | 27 ns | 163 ns |
| **num-bigint** | 34 ns | 31 ns | 44 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (27 ns)
- **1024-bit**: Fastest is **nail** (163 ns)


