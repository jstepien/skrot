#!/usr/bin/env ruby
require 'benchmark'
require 'tempfile'

model = ARGV[0]
puts "original,encoded,ratio,t_enc,t_dec"
tmpfile = Dir::Tmpname.make_tmpname "tmp", nil
n_enc = 32
n_dec = 64
begin
  while ex = STDIN.gets do
    IO.popen "sh -c '../skr #{model} > #{tmpfile}'", "w" do |io|
      io.write ex
    end
    enc = Benchmark.measure do
      IO.popen "./measure -c #{n_enc} #{model}", "w" do |io|
        io.write ex
      end
    end
    comp = File.read(tmpfile)
    len = comp.length
    decomp = `cat #{tmpfile} | ../unskr #{model}`
    raise ex unless decomp == ex
    dec = Benchmark.measure do
      IO.popen "./measure -d #{n_dec} #{model}", "w" do |io|
        io.write comp
      end
    end
    result = [
      ex.length,
      len,
      len * 1.0 / ex.length,
      enc.real / n_enc * 1e3,
      dec.real / n_dec * 1e3
    ]
    puts result.join(",")
  end
ensure
  `rm -f #{tmpfile}`
end
