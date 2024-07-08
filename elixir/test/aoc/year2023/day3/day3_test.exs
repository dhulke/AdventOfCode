defmodule AocTest.Year2023.Day3.Day3 do
  use ExUnit.Case

  test "Part 1 - Example" do
    sample_input = """
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    """

    assert Aoc.Year2023.Day3.Day3.part1(sample_input) == 4361
  end

#  test "Part 1 - Real Payload" do
#    contents = File.read!(Path.join(__DIR__, "part3_payload.txt"))
#
#    assert Aoc.Year2023.Day3.Day3.part1(contents) == 2256
#  end

#  test "Part 2 - Example" do
#    sample_input = """
#    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
#    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
#    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
#    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
#    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
#    """
#
#    assert Aoc.Year2023.Day2.Day2.part2(sample_input) == 2286
#  end
#
#  test "Part 2 - Real Payload" do
#    contents = File.read!(Path.join(__DIR__, "part2_payload.txt"))
#
#    assert Aoc.Year2023.Day2.Day2.part2(contents) == 74229
#  end
end
