import std.stdio;
import std.algorithm : sum;
import dhtslib;

/// Ported from rust-htslib implementation

void main(string[] args)
{
	// open vcf reader
	auto vcfr = VCFReader(args[1]);
	long[] li;

	// loop over records and get the AN INFO field
	// and append it to our list
	foreach(rec; vcfr)
	{
		li ~= rec.getInfos["AN"].to!int;
	}

	// sum and print
	auto s = li.sum;
	stderr.writefln("sum: %d, avg: %f", s, double(s) / double(li.length));
}
