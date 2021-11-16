from cyvcf2 import VCF
import sys

li = []

vcf = VCF(sys.argv[1], lazy=True)
#vcf.set_samples("HG00096")
for v in vcf:
    try:
        li.append(v.INFO["AN"])
    except KeyError:
        continue

print(f"{(sum(li)/len(li)):.2f}")
