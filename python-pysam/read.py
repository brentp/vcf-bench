from pysam import VariantFile
import sys


li = []
for v in VariantFile(sys.argv[1]):
    try:
        li.append(v.info["AN"])
    except KeyError:
        pass

print(sum(li) / len(li))

