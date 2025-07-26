# 1. load message bits
A = []
with open("message_bits.txt", 'r') as f:
    for line in f:
        row = [int(bit) for bit in line.strip()]
        A.append(row)

assert len(A) == 256
assert all(len(row) == 256 for row in A)

# 2: set up BLS12-381 scalar field
P = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
F = FiniteField(P)

# 3. build matrix GA
GA = Matrix(F, A)        # initially rows are messages
GAT = GA.transpose()     # then transpose so each column is a message instead

# 4. invert matrix GA
GAinv = GAT.inverse()

# 5. load target bits
with open("target_bits.txt", "r") as f:
    line = f.readline().strip()
    bits = [int(b) for b in line]
    assert len(bits) == 256
    gbits = vector(F, bits)

# 6. multiply inverse * target
gsolution = GAinv * gbits  # coefficients in F

# 7. format for rust (arkworks style)
with open("coeffs.rs", "w") as f:
    f.write("let coeffs = vec![\n")
    for ci in gsolution:
        f.write(f'    Fq::from_str("{int(ci)}").unwrap(),\n')
    f.write("];\n")

print("Done. Wrote Rust-compatible coefficients to coeffs.rs.")
