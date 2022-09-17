# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Fp=18446744069414584321](#fp=18446744069414584321)

## Benchmark Results

### Fp=18446744069414584321

|                                          | `Specialized`             | `Generic`                         |
|:-----------------------------------------|:--------------------------|:--------------------------------- |
| **`Addition`**                           | `3.80 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.08x slower**)    |
| **`Subtraction`**                        | `3.75 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.10x slower**)    |
| **`Negation`**                           | `3.90 ns` (✅ **1.00x**)   | `4.21 ns` (✅ **1.08x slower**)    |
| **`Double`**                             | `4.39 ns` (✅ **1.00x**)   | `4.13 ns` (✅ **1.06x faster**)    |
| **`Multiplication`**                     | `4.00 ns` (✅ **1.00x**)   | `6.14 ns` (❌ *1.53x slower*)      |
| **`Square`**                             | `4.89 ns` (✅ **1.00x**)   | `5.56 ns` (❌ *1.14x slower*)      |
| **`Inverse`**                            | `285.02 ns` (✅ **1.00x**) | `543.15 ns` (❌ *1.91x slower*)    |
| **`Sum of products of size 2`**          | `7.35 ns` (✅ **1.00x**)   | `17.81 ns` (❌ *2.42x slower*)     |
| **`Naive sum of products of size 2`**    | `8.67 ns` (✅ **1.00x**)   | `15.43 ns` (❌ *1.78x slower*)     |
| **`Serialize Compressed`**               | `3.96 ns` (✅ **1.00x**)   | `4.68 ns` (❌ *1.18x slower*)      |
| **`Serialize Uncompressed`**             | `3.96 ns` (✅ **1.00x**)   | `4.73 ns` (❌ *1.19x slower*)      |
| **`Deserialize Compressed`**             | `5.25 ns` (✅ **1.00x**)   | `8.88 ns` (❌ *1.69x slower*)      |
| **`Deserialize Compressed Unchecked`**   | `5.25 ns` (✅ **1.00x**)   | `8.76 ns` (❌ *1.67x slower*)      |
| **`Deserialize Uncompressed`**           | `4.98 ns` (✅ **1.00x**)   | `8.80 ns` (❌ *1.77x slower*)      |
| **`Deserialize Uncompressed Unchecked`** | `4.99 ns` (✅ **1.00x**)   | `8.79 ns` (❌ *1.76x slower*)      |
| **`Square Root for QR`**                 | `2.77 us` (✅ **1.00x**)   | `4.41 us` (❌ *1.59x slower*)      |
| **`Legendre for QR`**                    | `596.07 ns` (✅ **1.00x**) | `1.11 us` (❌ *1.87x slower*)      |
| **`From BigInt`**                        | `4.33 ns` (✅ **1.00x**)   | `5.31 ns` (❌ *1.23x slower*)      |
| **`Into BigInt`**                        | `3.93 ns` (✅ **1.00x**)   | `4.72 ns` (❌ *1.20x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

