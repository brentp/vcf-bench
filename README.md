
# task

the timed task will be to:
iterate over each row in a VCF, extract a value from the INFO field (an
integer), add it to a vector and report the mean at the end.

# Summary

Individual runs are below. `rust-htslib` was compiled with --release and `hts-nim` was compiled with -d:danger and uses libdeflate.

### BCF

| Tool  | Time   | File |
|-------|--------|------|
| cyvcf2 | 8.15s | BCF  |
| rust-htslib | 5.8s | BCF |
| hts-nim | 3.5s | BCF |

### VCF

| Tool  | Time   | File |
|-------|--------|------|
| pyvcf | 16m49s | VCF  |
| cyvcf2 | 29s   | VCF  |
| rust-htslib | 22s | VCF |
| hts-nim | 18s | VCF |


# testing data

```
wget -O - http://ftp.1000genomes.ebi.ac.uk/vol1/ftp/data_collections/1000G_2504_high_coverage/working/20201028_3202_raw_GT_with_annot/20201028_CCDG_14151_B01_GRM_WGS_2020-08-05_chr1.recalibrated_variants.vcf.gz \
    | zcat - \
    | head -30000 \
    | bgzip -c > 1kg.chr1.subset.vcf.gz
```

# Timings

## python pyvcf

```
$ time python python-pyvcf/read.py 1kg.chr1.subset.vcf.gz 
5992.641098163443

real    16m49.068s
user    16m44.487s
sys     0m0.924s
```

## python cyvcf2


### BCF

```
$ time python python-cyvcf2/read.py 1kg.chr1.subset.bcf    
5992.64

real    0m8.154s
user    0m8.168s
sys     0m0.268s
```

### VCF

```
$ time python python-cyvcf2/read.py 1kg.chr1.subset.vcf.gz 
5992.64
real    0m29.703s
user    0m29.715s
sys     0m0.260s
```

### VCF with subset samples

```
$ time python python-cyvcf2/read.py 1kg.chr1.subset.vcf.gz 
5992.64

real    0m9.927s
user    0m9.938s
```

## rust-htslib


### BCF

```
$ time rust-htslib/target/release/rust-rhtslib ./1kg.chr1.subset.bcf    
sum: 158906864, avg:5992

real    0m5.888s
user    0m5.799s
sys     0m0.088s
```

### VCF
```
$ time rust-htslib/target/release/rust-rhtslib ./1kg.chr1.subset.vcf.gz 
sum: 158906864, avg:5992

real    0m22.122s
user    0m22.002s
sys     0m0.116s
```

## hts-nim

### BCF

```
$ time ./nim-hts-nim/read ./1kg.chr1.subset.bcf 
5992.641098163443

real    0m3.579s
user    0m3.482s
sys     0m0.097s
```

### VCF

```
$ time ./nim-hts-nim/read ./1kg.chr1.subset.vcf.gz 
5992.641098163443
real    0m18.346s
user    0m18.276s
sys     0m0.068s
```
