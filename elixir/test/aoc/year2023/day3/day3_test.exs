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

  test "Part 1 - Real Payload" do
    contents = File.read!(Path.join(__DIR__, "part3_payload.txt"))

    assert Aoc.Year2023.Day3.Day3.part1(contents) == 530849
  end

  test "Part 2 - Example" do
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

    assert Aoc.Year2023.Day3.Day3.part2(sample_input) == 467835
  end

  test "Part 2 - Real Payload" do
    contents = File.read!(Path.join(__DIR__, "part3_payload.txt"))

    assert Aoc.Year2023.Day3.Day3.part2(contents) == 84900879
  end
end
