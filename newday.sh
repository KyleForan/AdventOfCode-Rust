
day=${1:=0}
year=${2:=2015}
foldername="$year/day$(printf "%02d" $day)"

printf "Creating project for %d day %d...\n" "$year" "$day"
mkdir -p "$year"
cargo new "$foldername"
cp "template/template.rs" "$foldername/src/main.rs"

printf "Moving to $foldername...\n"
cd "$foldername"

printf "Retrieving Input...\n"
curl "https://adventofcode.com/$year/day/$day/input" --cookie "session=$AOC" > ./src/input.txt 

