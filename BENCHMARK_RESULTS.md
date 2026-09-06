# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 7 ns | 26 ns | 19 ns | - | - |
| **nail** | 1 ns | 1 ns | 2 ns | 4 ns | 9 ns |
| **num-bigint** | 27 ns | 27 ns | 17 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (4 ns)
- **1024-bit**: Fastest is **nail** (9 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 7 ns | 20 ns | 26 ns | - | - |
| **nail** | 1 ns | 1 ns | 3 ns | 16 ns | 81 ns |
| **num-bigint** | 25 ns | 22 ns | 30 ns | - | - |
| **rug-gmp** | 1 ns | 1 ns | 1 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (1 ns)
- **256-bit**: Fastest is **rug-gmp** (1 ns)
- **512-bit**: Fastest is **nail** (16 ns)
- **1024-bit**: Fastest is **nail** (81 ns)


