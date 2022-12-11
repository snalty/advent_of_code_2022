using System;
using System.Collections.Generic;
using System.Drawing;

class Day9 {
    static void Main(string[] args){
        // Display the number of command line arguments.
        List<Tuple<Char, Int16>> input = new List<Tuple<Char, Int16>>();
        
        string line;
        while (!string.IsNullOrEmpty(line = Console.ReadLine())) {
            var command = line.Split(' ');
            input.Add(
                new Tuple<Char, Int16>(char.Parse(command[0]), short.Parse(command[1]))
                );
        }

        var head = new Point(0, 0);
        var tail = new Point(0, 0);

        var tail_positions = new HashSet<Point>();

        foreach (var (instruction, distance) in input) {
            Console.WriteLine(instruction.ToString() + " " + distance);
            for (var i = 0; i < distance; i++) {
                switch(instruction) {
                    case 'R':
                        head.X += 1;
                        break;
                    case 'L':
                        head.X -= 1;
                        break;
                    case 'U':
                        head.Y += 1;
                        break;
                    case 'D':
                        head.Y -= 1;
                        break;
                }

                // Console.WriteLine("Head:" + head);

                var delta = Point.Subtract(head, (Size)tail);

                // Handle touching points
                // Directly 2 in any direction
                Console.WriteLine(delta);
                if (delta.X == 0 || delta.Y == 0) {
                    if (delta.X == 2) { tail.X += 1; }
                    if (delta.X == -2) { tail.X -= 1; }
                    if (delta.Y == 2) { tail.Y += 1; }
                    if (delta.Y == -2) { tail.Y -= 1; }
                } else if (Math.Abs(delta.X) > 1 | Math.Abs(delta.Y) > 1) {

                    if  (delta.X > 0) { tail.X += 1; }
                    if  (delta.X < 0) { tail.X -= 1; }
                    if  (delta.Y > 0) { tail.Y += 1; }
                    if  (delta.Y < 0) { tail.Y -= 1; }

                }
                // Console.WriteLine("Tail: " + tail);
                PrintGrid(head, tail);
                tail_positions.Add(tail);
            }
        }
    
        Console.WriteLine(tail_positions.Count);
    }

    static void PrintGrid(Point head, Point tail) {
        var x_max = 5;
        var y_max = 5;

        for (var v = y_max; v >= 0; v--) {
            for (var h = 0; h <= x_max; h++) {
                if (h == head.X && v == head.Y) {
                    Console.Write("H");
                } else if (h == tail.X && v == tail.Y) {
                    Console.Write("T");
                } else {
                    Console.Write(".");
                }
            }
            Console.WriteLine("");
        }
        Console.WriteLine("");
    }
}
