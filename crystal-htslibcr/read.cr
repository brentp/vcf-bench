require "htslib/hts/bcf"

fname = ARGV[0]
a = [] of Int32

f = HTS::Bcf.new(fname)
f.each do |r|
  an = r.info.get_int("AN")
  a << an[0] if an
end
f.close

sum = a.sum
avg = sum / a.size.to_f

puts "sum: #{sum}, avg: #{avg}"

