import random

r = random.randint(0, 2**64 - 1)

body = f"\t\tlet r = 0b{r:064b}_u64;\n"
for i in range(64):
    for j in range(i + 1, 65):
        l = j - i
        v = f"{r:064b}"[::-1][i:j][::-1]
        x = int(v, 2)
        z = f"0b{x:0{l}b}"
        body += f"\t\tassert_eq!(r.extract_bitfield::<{i:02}, {j:02}>(), {z:>66}_u64);\n"

template = f"""
#![cfg(test)]
use bitfields::*;

#[test]
fn test_extract_bitfield() {{
{body}
}}
"""

with open("./tests/extract_bitfield.rs", "w") as f:
    f.write(template)
    

body = f"\t\tlet r = 0b{r:064b}_u64;\n"
for i, b in enumerate(reversed( f"{r:064b}")):
    v = str(b != "0").lower()
    body += f"\t\tassert_eq!(r.extract_bit::<{i:02}>(), {v});\n"

template = f"""
#![cfg(test)]
use bitfields::*;

#[test]
fn test_extract_bit() {{
{body}
}}
"""

with open("./tests/extract_bit.rs", "w") as f:
    f.write(template)