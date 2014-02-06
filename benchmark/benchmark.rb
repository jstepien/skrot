#!/usr/bin/env ruby
require 'benchmark'
require 'tempfile'

corpus = ARGV[0]
puts "original,encoded,ratio,t_enc,t_dec"
tmpfile = Dir::Tmpname.make_tmpname "tmp", nil
begin
  while ex = STDIN.gets do
    enc = Benchmark.measure do
      `echo -n "#{ex.gsub '"', '\\"'}" | ../skr #{corpus} > #{tmpfile}`
    end
    len = `wc -c #{tmpfile}`.to_i
    dec = Benchmark.measure do
      decomp = `cat #{tmpfile} | ../unskr #{corpus}`
      raise ex unless decomp == ex
    end
    result = [ex.length, len, len * 1.0 / ex.length, enc.real, dec.real]
    puts result.join(",")
  end
ensure
  `rm -f #{tmpfile}`
end
