#!/usr/bin/env python3

import sys

import pandas as pd

result = sys.argv[1] if len(sys.argv) > 1 else "./result.txt"
df = pd.read_csv(result, sep=" ", header=None)

print("Min:", df[0].min())
print("Avg cost:", df[0].mean())
print("Standard deviation (cost):", df[0].std())
print(f"Avg time: {df[1].mean()}s")
print("Standard deviation (time):", df[1].std())
print("Count:", len(df))
