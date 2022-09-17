# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [Fp=18446744069414584321](#fp=18446744069414584321)

## Benchmark Results

### Fp=18446744069414584321

|                                          | `Generic`   | `Specialized`                     |
| :--------------------------------------- | :---------- | :-------------------------------- |
| **`Sum of products of size 2`**          | `18.04 ns`  | `7.34 ns` (🚀 **2.46x faster**)   |
| **`Inverse`**                            | `556.74 ns` | `283.87 ns` (🚀 **1.96x faster**) |
| **`Legendre for QR`**                    | `1.12 us`   | `596.15 ns` (🚀 **1.88x faster**) |
| **`Naive sum of products of size 2`**    | `15.41 ns`  | `8.68 ns` (🚀 **1.78x faster**)   |
| **`Deserialize Compressed`**             | `8.82 ns`   | `4.99 ns` (🚀 **1.77x faster**)   |
| **`Deserialize Compressed Unchecked`**   | `8.80 ns`   | `4.97 ns` (🚀 **1.77x faster**)   |
| **`Deserialize Uncompressed`**           | `8.86 ns`   | `5.16 ns` (🚀 **1.72x faster**)   |
| **`Deserialize Uncompressed Unchecked`** | `8.81 ns`   | `5.15 ns` (🚀 **1.71x faster**)   |
| **`Square Root for QR`**                 | `4.43 us`   | `2.77 us` (🚀 **1.60x faster**)   |
| **`Multiplication`**                     | `6.15 ns`   | `4.03 ns` (🚀 **1.53x faster**)   |
| **`From BigInt`**                        | `5.32 ns`   | `4.30 ns` (✅ **1.24x faster**)   |
| **`Serialize Uncompressed`**             | `4.72 ns`   | `3.95 ns` (✅ **1.20x faster**)   |
| **`Into BigInt`**                        | `4.72 ns`   | `3.92 ns` (✅ **1.20x faster**)   |
| **`Serialize Compressed`**               | `4.72 ns`   | `3.96 ns` (✅ **1.19x faster**)   |
| **`Square`**                             | `5.60 ns`   | `4.88 ns` (✅ **1.15x faster**)   |
| **`Subtraction`**                        | `4.09 ns`   | `3.77 ns` (✅ **1.09x faster**)   |
| **`Addition`**                           | `4.11 ns`   | `3.79 ns` (✅ **1.08x faster**)   |
| **`Negation`**                           | `4.21 ns`   | `3.90 ns` (✅ **1.08x faster**)   |
| **`Double`**                             | `4.13 ns`   | `4.32 ns` (❌ **1.04x slower**)   |

---

Made with [criterion-table](https://github.com/nu11ptr/criterion-table)
