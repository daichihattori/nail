# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 11 ns | 36 ns | 24 ns | - | - |
| **nail** | 1 ns | 2 ns | 3 ns | 6 ns | 15 ns |
| **num-bigint** | 36 ns | 36 ns | 21 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (6 ns)
- **1024-bit**: Fastest is **nail** (15 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 11 ns | 32 ns | 42 ns | - | - |
| **nail** | 1 ns | 2 ns | 5 ns | 30 ns | 164 ns |
| **num-bigint** | 34 ns | 32 ns | 44 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **nail** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (30 ns)
- **1024-bit**: Fastest is **nail** (164 ns)


