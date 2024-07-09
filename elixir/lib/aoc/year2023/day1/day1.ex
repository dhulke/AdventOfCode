defmodule Aoc.Year2023.Day1.Day1 do
  @moduledoc """
  Although part2's implementation works for part1, it's less performant and
  so I left both implementations for reference
  """

  alias Aoc.TextUtil
  alias Aoc.RegexUtil

  @digits %{
    "one" => "1",
    "1" => "1",
    "two" => "2",
    "2" => "2",
    "three" => "3",
    "3" => "3",
    "four" => "4",
    "4" => "4",
    "five" => "5",
    "5" => "5",
    "six" => "6",
    "6" => "6",
    "seven" => "7",
    "7" => "7",
    "eight" => "8",
    "8" => "8",
    "nine" => "9",
    "9" => "9"
  }

  def part1(input) do
    input
    |> TextUtil.lines()
    |> Enum.map(fn line ->
      line
      |> digits()
      |> first_and_last_digits()
      |> String.to_integer()
    end)
    |> Enum.sum()
  end

  defp first_and_last_digits(digits) do
    if byte_size(digits) === 1 do
      digits <> digits
    else
      <<first::binary-size(1), _::binary-size(byte_size(digits) - 2), last::binary-size(1)>> =
        digits

      first <> last
    end
  end

  defp digits(line) do
    line
    |> String.graphemes()
    |> Enum.filter(&TextUtil.is_digit/1)
    |> Enum.join("")
  end

  def part2(input) do
    input
    |> TextUtil.lines()
    |> Enum.map(&first_and_last_digit_as_integer/1)
    |> Enum.sum()
  end

  defp first_and_last_digit_as_integer(line) do
    Map.keys(@digits)
    |> Enum.map(&indexes_of(&1, line))
    |> List.flatten()
    |> min_max()
    |> Tuple.to_list()
    |> Enum.map(&@digits[elem(&1, 1)])
    |> Enum.join()
    |> String.to_integer()
  end

  defp indexes_of(pattern, subject) do
    ~r/#{pattern}/
    |> RegexUtil.scan_slice_index(subject)
    |> Enum.map(&{elem(&1, 0), pattern})
  end

  defp min_max(indexes) do
    Enum.reduce(indexes, nil, fn
      {index, pattern}, nil ->
        {{index, pattern}, {index, pattern}}

      {index, pattern}, {{min_index, min_pattern}, {max_index, max_pattern}} ->
        cond do
          index > max_index ->
            {{min_index, min_pattern}, {index, pattern}}

          index < min_index ->
            {{index, pattern}, {max_index, max_pattern}}

          true ->
            {{min_index, min_pattern}, {max_index, max_pattern}}
        end
    end)
  end
end
