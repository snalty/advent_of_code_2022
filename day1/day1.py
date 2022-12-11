import sys

def part1():
    file = sys.stdin.readlines()
    elves = []
    elf = 0
    elves.append((elf, []))
    for line in file:
        line = line.strip()
        if line == "":
            elf += 1
            elves.append((elf, []))
            continue 
        elves[elf][1].append(int(line))  
    elves.sort(key=lambda x: sum(x[1]))
    print(elves[-1], sum(elves[-1][1]))
    return elves

def part2(elves):
    answer = 0
    for elf in elves[-3:]:
        answer += sum(elf[1])
    print(answer)

if __name__ == "__main__":
    elves = part1()
    part2(elves) 
