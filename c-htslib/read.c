#include <stdio.h>
#include "htslib/vcf.h"
#include "htslib/hts.h"

int main(int argc, char *argv[]) {

  char *fname = argv[1];

  htsFile *fp = hts_open(fname, "r");
  bcf_hdr_t *hdr = bcf_hdr_read(fp);

  bcf1_t *line = bcf_init();
  int *values = 0;
  int count = 0;

  double s = 0;
  int n = 0;

  while (bcf_read(fp, hdr, line) == 0) {
    if (bcf_get_info_int32(hdr, line, "AN", &values, &count) > 0) {
      s += *values;
      n += 1;
    }
  }

  bcf_hdr_destroy(hdr);
  hts_close(fp);
  bcf_destroy(line);
  free(values);

  fprintf(stderr, "%.2f\n", s / (double)n);
}
