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

touch $directory/input.txt

# create the files
cat >$directory/challenge.md <<EOF
# Day $day

Link: https://adventofcode.com/2024/day/$day
EOF

cat >$directory/main.rs <<EOF
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day$day");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    Ok(())
}
EOF
