import java.util.Scanner;
import java.util.ArrayList;
import java.util.stream.IntStream;
import java.util.stream.Collectors;
import java.util.Set;
import java.util.Collections;

public class Main {
    public static void main(String[] args) {
        Scanner stdin = new Scanner(System.in);
        ArrayList<String> input = new ArrayList<>();
        while (stdin.hasNextLine()) {
            input.add(stdin.nextLine());
        }
        part1(input);
    }

    public static void part1(ArrayList<String> input) {
        Integer count_1 = 0;
        Integer count_2 = 0;
        for (String line : input) {
            String[] range_strings = line.split(",");
            ArrayList<Set> ranges = new ArrayList<>();
            for (String range : range_strings) {
                String[] min_max = range.split("-");
                ranges.add(
                    IntStream.rangeClosed(Integer.parseInt(min_max[0]), Integer.parseInt(min_max[1]))
                        .boxed().collect(Collectors.toSet())
                );

            };
            if (ranges.get(0).containsAll(ranges.get(1)) | ranges.get(1).containsAll(ranges.get(0))) {
                count_1 += 1;
            }
            if (!Collections.disjoint(ranges.get(0), ranges.get(1))) {
                count_2 += 1;
            }
        };

        System.out.println(count_1);
        System.out.println(count_2);
    };
}
