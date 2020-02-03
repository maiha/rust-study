# crystal build -o cr-cat cr-cat.cr --release

file = ARGV.shift? || raise "args(1) not found"

File.each_line(file) do |line|
  puts line
end

