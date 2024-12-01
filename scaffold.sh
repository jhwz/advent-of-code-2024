# get the day as the first argument
day=$1
if [ -z $day ]; then
    # figure out the next day
    day=$(ls src/bin | sort -n | tail -n 1 | sed 's/day//')
    day=$((day + 1))
fi

directory=src/bin/day$day
# See if the directory already exists
if [ -d $directory ]; then
    echo "Directory $directory already exists"
    exit 1
fi

# create the directory
mkdir $directory

# create the files
cat >$directory/challenge.md <<EOF
# Day $day

Link: https://adventofcode.com/2024/day/$day
EOF

cat >$directory/main.rs <<EOF
fn main() {
    println!("Hello, world!");
}
EOF
