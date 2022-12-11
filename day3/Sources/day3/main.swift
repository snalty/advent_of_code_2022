let scores = Array("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

func main() {
    var input: [String] = [];
    while let line = readLine() { input += [line] };
    part1(input: input);
    part2(input: input);
}

func part1(input: [String]) {
    var score = 0;
    for line in input {
        let midpoint = line.count / 2;
        let a = Set(Array(line.prefix(midpoint)));
        let b = Set(Array(line.suffix(midpoint)));
        let matches = a.intersection(b);
        matches.forEach { char in score += scores.firstIndex(of: char)! + 1 };
    }
    print(score);
}

func part2(input: [String]) {
    var groups: [Set<Character>] = []
    var score = 0;
    var i = 0;
    for line in input {
        if i <= 2 {
            groups.append(Set(Array(line)));
            i += 1;
        }
        if i > 2  {
            let matches = groups[0].intersection(groups[1]).intersection(groups[2])
            matches.forEach { char in score += scores.firstIndex(of: char)! + 1 };
            groups.removeAll();
            i = 0;
        }
    }
    print(score)
}

main()