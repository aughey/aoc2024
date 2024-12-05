for day in 5; do # 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25; do
    out="src/day$day.rs"
    cp template.rs $out
    sed -i "s/pub const DAY.*/pub const DAY: u32 = $day;/g" $out
    sed -i "s/day3/day$day/g" $out
    touch input/2024/day$day-test.txt
done