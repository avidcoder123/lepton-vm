import os
import timeit

print("Building:")
os.system("cargo b --release")

print("Benchmarking Python:")
py = timeit.timeit(lambda: os.system("python bench/bench.py > /dev/null"), number = 100)
print(py/100)

print("Benchmarking LeptonVM:")
levim = timeit.timeit(lambda: os.system("./target/release/tauvm > /dev/null"), number = 100)
print(levim/100)

print("")
print("LeptonVM is about", int(py/levim), "times faster than Python.")