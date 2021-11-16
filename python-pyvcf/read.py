import vcf
import gzip
import sys


reader = vcf.Reader(open(sys.argv[1], 'rb'))
li = []

for rec in reader:

    try:
        li.append(rec.INFO["AN"])
    except KeyError:
        continue


print(sum(li) / len(li))



