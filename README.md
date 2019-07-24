rkv-fuzz
-----
afl-based fuzzing setup for rkv

howto
-----
* build: `RUSTFLAGS="-Clink-arg=-fuse-ld=gold" cargo afl build`
* fuzz: `RUSTFLAGS="-Clink-arg=-fuse-ld=gold" cargo afl fuzz -i in -o out target/debug/rkv-fuzz`
* re-run on specific input: `RUSTFLAGS="-Clink-arg=-fuse-ld=gold" cargo afl run < path/to/input/file`

hints
-----
afl may require setting the core pattern as root like so: `echo core > /proc/sys/kernel/core_pattern`
