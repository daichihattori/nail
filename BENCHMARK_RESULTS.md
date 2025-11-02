# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 14 ns | 46 ns | 30 ns | - | - |
| **nail** | 1 ns | 2 ns | 4 ns | 8 ns | 20 ns |
| **num-bigint** | 45 ns | 46 ns | 28 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **nail** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (8 ns)
- **1024-bit**: Fastest is **nail** (20 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 14 ns | 42 ns | 55 ns | - | - |
| **nail** | 1 ns | 2 ns | 7 ns | 37 ns | 214 ns |
| **num-bigint** | 46 ns | 39 ns | 58 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (37 ns)
- **1024-bit**: Fastest is **nail** (214 ns)


