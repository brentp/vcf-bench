import hts/vcf
import os

proc main() =

  var f = paramStr(1)

  var ivcf:VCF
  if not ivcf.open(f):
    quit "couldn't open input vcf"

  var an = newSeq[int32](1)
  var li = newSeqOfCap[int64](65536)
  for v in ivcf:

    if v.info.get("AN", an) != Status.OK:
      continue
    li.add(an[0])

  var s = 0'i64
  for v in li: s += v
  echo s.float64 / li.len.float64


when isMainModule:
  main()
