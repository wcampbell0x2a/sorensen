# Sorensen Compression

# build
```
cargo b --release
```

# run
```
cargo r --release
```

# benchmark
`target-cpu=native` gives around a 5% performance boost.

```
RUSTFLAGS="-C target-cpu=native" cargo bench
```

## example benchmark (Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz)
```
01 bytes                time:   [4.2423 us 4.2974 us 4.3527 us]
02 bytes                time:   [274.52 us 275.55 us 276.68 us]
03 bytes                time:   [20.334 ms 20.414 ms 20.500 ms]
04 bytes                time:   [1.6147 s  1.6206 s  1.6267 s]
05 bytes                time:   [127.16 s  131.32 s  135.23 s]
```

## Previous.... _art_
* [Kfeavel/sorensen](https://github.com/Kfeavel/sorensen)
* [aedrax/SorensenCompression](https://github.com/aedrax/SorensenCompression)
