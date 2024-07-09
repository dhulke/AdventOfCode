defmodule Aoc.Year2023.Day3.Day3 do
  @moduledoc false

  alias Aoc.TextUtil

  def part1(input) do
    number_coordinates =
      input
      |> TextUtil.lines()
      |> Enum.with_index()
      |> Enum.map(&number_coordinates_to_map/1)
      |> Enum.reduce(%{}, &Map.merge/2)

    input
    |> TextUtil.lines()
    |> Enum.with_index()
    |> Enum.map(&numbers_adjacent_to_symbols(number_coordinates, &1))
    |> List.flatten()
    |> Enum.uniq()
    |> Stream.map(fn {_, _, number, _, _} -> number end)
    |> Stream.map(&String.to_integer/1)
    |> Enum.sum()
  end

  def part2(input) do
    number_coordinates =
      input
      |> TextUtil.lines()
      |> Enum.with_index()
      |> Enum.map(&number_coordinates_to_map/1)
      |> Enum.reduce(%{}, &Map.merge/2)

    input
    |> TextUtil.lines()
    |> Enum.with_index()
    |> Enum.map(&gear_ratios(number_coordinates, &1))
    |> List.flatten()
    |> Enum.sum()
  end

  defp gear_ratios(numbers_coordinates, {line, line_index}) do
    line
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.filter(fn {grapheme, _} -> grapheme === "*" end)
    |> Enum.map(fn {_, col_index} ->
      lookup_numbers_adjacent_to(numbers_coordinates, col_index, line_index)
    end)
    |> Enum.filter(fn adjacent_numbers -> length(adjacent_numbers) === 2 end)
    |> Enum.map(fn [{_, _, number1, _, _}, {_, _, number2, _, _}] ->
      String.to_integer(number1) * String.to_integer(number2)
    end)
  end

  defp numbers_adjacent_to_symbols(numbers_coordinates, {line, line_index}) do
    line
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.filter(fn {grapheme, _} ->
      !TextUtil.is_digit(grapheme) and grapheme !== "."
    end)
    |> Enum.map(fn {_, col_index} ->
      lookup_numbers_adjacent_to(numbers_coordinates, col_index, line_index)
    end)
    |> List.flatten()
  end

  defp lookup_numbers_adjacent_to(number_coordinates, sym_col_index, sym_line_index) do
    [
      {sym_col_index - 1, sym_line_index - 1},
      {sym_col_index, sym_line_index - 1},
      {sym_col_index + 1, sym_line_index - 1},
      {sym_col_index - 1, sym_line_index},
      {sym_col_index, sym_line_index},
      {sym_col_index + 1, sym_line_index},
      {sym_col_index - 1, sym_line_index + 1},
      {sym_col_index, sym_line_index + 1},
      {sym_col_index + 1, sym_line_index + 1}
    ]
    |> Enum.map(&Map.get(number_coordinates, &1))
    |> Enum.reject(&is_nil/1)
    |> Enum.map(fn x -> x |> Tuple.append(sym_col_index) |> Tuple.append(sym_line_index) end)
    |> Enum.uniq()
  end

  defp number_coordinates_to_map({line, line_index}) do
    Regex.scan(~r/\d+/, line, return: :index)
    |> Enum.map(&hd/1)
    |> Enum.map(&distribute_range_to_indexes(&1, line))
    |> List.flatten()
    |> index_column_line_to_map(line_index)
  end

  defp distribute_range_to_indexes({index, length}, line) do
    number = String.slice(line, index, length)
    Enum.map(index..(index + length - 1), &{&1, {index, length, number}})
  end

  defp index_column_line_to_map(indexes_numbers, line_index) do
    indexes_numbers
    |> Enum.map(fn {col_index, number_slice_index} ->
      {{col_index, line_index}, number_slice_index}
    end)
    |> Map.new()
  end
end
