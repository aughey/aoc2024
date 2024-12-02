day=`echo $1 | sed 's/[^0-9]//g'`
if [ -z "$day" ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

codspeed=~/Advent/rust-runner

rm $codspeed/input.txt
cp input/2024/day$1.txt $codspeed/input.txt
cd $codspeed
rm Cargo.lock

sed -i "s/day[0-9]*/day$day/g" bench.rs
cargo codspeed build && cargo codspeed run