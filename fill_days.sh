for day in 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25; do
    cp template.rs src/lib/day$day.rs
    sed -i "s/day3/day$day/g" src/lib/day$day.rs
    sed -i "s/test3.txt/test$day.txt/g" src/lib/day$day.rs
    touch test$day.txt
done