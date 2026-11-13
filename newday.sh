
day=${1:=0}
year=${2:=2015}
foldername="$year/day$(printf "%02d" $day)"

cargo new "$foldername"
cp "template/template.rs" "$foldername/src/main.rs"
cd "$foldername"

curl https://adventofcode.com/$year/day/$day/input > ./src/input.txt
