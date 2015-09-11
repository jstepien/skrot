#!/usr/bin/env ruby
require 'open3'
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
    enc = Open3.popen2 "./measure.py -c #{n_enc} #{model}" do |inp, out|
      inp.write ex
      inp.close
      out.read
    end
    comp = File.read(tmpfile)
    len = comp.length
    decomp = `cat #{tmpfile} | ../unskr #{model}`
    raise ex unless decomp == ex
    dec = Open3.popen2 "./measure.py -d #{n_dec} #{model}" do |inp, out|
      inp.write comp
      inp.close
      out.read
    end
    result = [
      ex.length,
      len,
      len * 1.0 / ex.length,
      enc.to_f * 1e3,
      dec.to_f * 1e3
    ]
    puts result.join(",")
  end
ensure
  `rm -f #{tmpfile}`
end
