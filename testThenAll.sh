# cargo run -- --input puzzles/solvable3std.txt --goal std

for i in {0..64}
do
    echo "it's $i"
    cat "testinit/puzzles copy $i.txt" | grep -a \# | grep -v COM
    cargo run --release -- --input "testinit/puzzles copy $i.txt" --goal custom "testgoals/puzzles copy $i.txt"
done
