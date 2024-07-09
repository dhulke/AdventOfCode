defmodule AocTest.Year2023.Day1.Day1 do
  use ExUnit.Case

  test "Part 1 - Example" do
    sample_input = """
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    """

    assert Aoc.Year2023.Day1.Day1.part1(sample_input) === 142
  end

  test "Part 1 - Real Payload" do
    contents = File.read!(Path.join(__DIR__, "part1_payload.txt"))

    assert Aoc.Year2023.Day1.Day1.part1(contents) === 54697
  end

  test "Part 2 - Example" do
    sample_input = """
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    """

    assert Aoc.Year2023.Day1.Day1.part2(sample_input) === 281
  end

  test "Part 2 - Edge Case" do
    sample_input = """
    twone
    """

    assert Aoc.Year2023.Day1.Day1.part2(sample_input) === 21
  end

  test "Part 2 - Real Payload" do
    contents = File.read!(Path.join(__DIR__, "part1_payload.txt"))

    assert Aoc.Year2023.Day1.Day1.part2(contents) === 54885
  end
end
