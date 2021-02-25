import os

data = ['a', 'b', 'c', 'd', 'e', 'f']

for c in data:
  os.system(f"cargo run {c}.out < {c}.txt")