import hts/vcf
import os
import math

proc main() =

  var ivcf:VCF
  if not ivcf.open(paramStr(1)):
    quit "couldn't open input vcf"

  #ivcf.set_samples(@["^"])
  var an = newSeq[int32](1)
  var li = newSeqOfCap[int64](65536)
  for v in ivcf:
    if v.info.get("AN", an) != Status.OK:
      continue
    li.add(an[0])

  echo sum(li).float64 / li.len.float64


when isMainModule:
  main()
