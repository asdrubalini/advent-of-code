# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Input Parse ](#input-parse-)
    - [Part I ](#part-i-)
    - [Part II ](#part-ii-)

## Benchmark Results

### Input Parse 

|        | ` One`                   | ` Two`                          | ` Three`                         | ` Four`                          |
|:-------|:-------------------------|:--------------------------------|:---------------------------------|:-------------------------------- |
|        | `38.64 us` (✅ **1.00x**) | `73.62 us` (❌ *1.91x slower*)   | `197.97 us` (❌ *5.12x slower*)   | `61.92 us` (❌ *1.60x slower*)    |

### Part I 

|        | ` One`                    | ` Two`                         | ` Three`                         | ` Four`                         |
|:-------|:--------------------------|:-------------------------------|:---------------------------------|:------------------------------- |
|        | `646.20 ns` (✅ **1.00x**) | `2.16 us` (❌ *3.34x slower*)   | `44.42 us` (❌ *68.74x slower*)   | `2.52 us` (❌ *3.90x slower*)    |

### Part II 

|        | ` One`                  | ` Two`                         | ` Three`                          | ` Four`                            |
|:-------|:------------------------|:-------------------------------|:----------------------------------|:---------------------------------- |
|        | `4.34 us` (✅ **1.00x**) | `3.69 us` (✅ **1.17x faster**) | `341.12 us` (❌ *78.67x slower*)   | `240.93 ns` (🚀 **18.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

