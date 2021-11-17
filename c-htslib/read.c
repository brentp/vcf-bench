#include <stdio.h>
#include "htslib/vcf.h"
#include "htslib/hts.h"
#include <stdint.h>
#include "kvec.h"

int main(int argc, char *argv[]) {

  char *fname = argv[1];

  kvec_t(int32_t) array;
  kv_init(array);

  htsFile *fp = hts_open(fname, "r");
  bcf_hdr_t *hdr = bcf_hdr_read(fp);

  bcf1_t *line = bcf_init();
  int *values = 0;
  int count = 0;


  while (bcf_read(fp, hdr, line) == 0) {
    if (bcf_get_info_int32(hdr, line, "AN", &values, &count) > 0) {
      kv_push(int32_t, array, *values);
    }
  }
  double s = 0;
  for(int i=0; i<kv_size(array);i++) {
    s += kv_A(array, i);
  }

  bcf_hdr_destroy(hdr);
  hts_close(fp);
  bcf_destroy(line);
  free(values);
  kv_destroy(array);

  fprintf(stderr, "%.2f\n", s / (double)kv_size(array));
}
