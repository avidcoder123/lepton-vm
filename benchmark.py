import os
import timeit

print("Building:")
os.system("cargo b --release")
os.system("cd bench && rustc native.rs -O && cd ..")

print("Benchmarking Python:")
py = timeit.timeit(lambda: os.system("python bench/bench.py > /dev/null"), number = 100)
print(round(py/100, 5))

print("Benchmarking LeptonVM:")
levim = timeit.timeit(lambda: os.system("./target/release/tauvm bench/loop.levim > /dev/null"), number = 100)
print(round(levim/100, 5))

print("Benchmarking Native(Rust):")
rs = timeit.timeit(lambda: os.system("./bench/native > /dev/null"), number = 100)
print(round(rs/100, 5))

print("")
print("LeptonVM is about", round(py/levim, 2), "times faster than Python and", round(levim/rs, 2), "times slower than native.")