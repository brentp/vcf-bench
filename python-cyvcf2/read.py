from cyvcf2 import VCF
import sys

li = []

vcf = VCF(sys.argv[1], lazy=True)
#vcf.set_samples("HG00096")
for v in vcf:
    try:
        f = v.INFO["AN"]
        li.append(f)
    except KeyError:
        continue

s = sum(li)
print(f"{(s/len(li)):.2f}")
