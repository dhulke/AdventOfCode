defmodule Aoc.Year2023.Day3.Day3 do
  @moduledoc """
  It's important to note that the same part number adjacent to two symbols is considered
  two separate part numbers. To represent this concept I've introduced the idea of a
  partial_part_number_id which is the part number with its coordinates and a part_number_id
  which is the part number, its coordinates and the coordinates of the symbol it's a part number for.

  In this challenge, we first create a lookup table with partial_part_number_ids and at a later
  stage we transform these partial_part_number_ids into part_number_ids in part_number_ids_adjacent_to/3.
  """

  alias Aoc.TextUtil
  alias Aoc.RegexUtil

  def part1(input) do
    partial_part_number_id_coordinates = partial_part_number_id_coordinates(input)

    input
    |> TextUtil.lines_with_index()
    |> Enum.map(&part_numbers_adjacent_to_symbols(partial_part_number_id_coordinates, &1))
    |> List.flatten()
    |> Enum.sum()
  end

  def part2(input) do
    partial_part_number_id_coordinates = partial_part_number_id_coordinates(input)

    input
    |> TextUtil.lines_with_index()
    |> Enum.map(&gear_ratios(partial_part_number_id_coordinates, &1))
    |> List.flatten()
    |> Enum.sum()
  end

  defp partial_part_number_id_coordinates(input) do
    input
    |> TextUtil.lines_with_index()
    |> Enum.map(&partial_part_number_id_coordinates_for_line/1)
    |> Enum.reduce(%{}, &Map.merge/2)
  end

  defp partial_part_number_id_coordinates_for_line({line, line_index}) do
    RegexUtil.scan_slice_index(~r/\d+/, line)
    |> Enum.map(fn {index, length} ->
      {String.to_integer(String.slice(line, index, length)), index, length}
    end)
    |> Enum.map(fn {part_number, index, length} ->
      Enum.map(index..(index + length - 1), &{{&1, line_index}, {part_number, index, length}})
    end)
    |> List.flatten()
    |> Map.new()
  end

  defp part_numbers_adjacent_to_symbols(partial_part_number_id_coordinates, {line, line_index}) do
    line
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.reject(fn {grapheme, _} ->
      TextUtil.is_digit(grapheme) or grapheme === "."
    end)
    |> Enum.map(fn {_, sym_col_index} ->
      part_number_ids_adjacent_to(partial_part_number_id_coordinates, sym_col_index, line_index)
    end)
    |> List.flatten()
    |> Enum.map(fn {part_number, _, _, _, _} -> part_number end)
  end

  defp part_number_ids_adjacent_to(partial_part_number_id_coordinates, sym_col_index, sym_line_index) do
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
    |> Enum.map(&Map.get(partial_part_number_id_coordinates, &1))
    |> Enum.reject(&is_nil/1)
    |> Enum.map(fn x -> x |> Tuple.append(sym_col_index) |> Tuple.append(sym_line_index) end)
    |> Enum.uniq()
  end

  defp gear_ratios(partial_part_number_id_coordinates, {line, line_index}) do
    line
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.filter(fn {grapheme, _} -> grapheme === "*" end)
    |> Enum.map(fn {_, sym_col_index} ->
      part_number_ids_adjacent_to(partial_part_number_id_coordinates, sym_col_index, line_index)
    end)
    |> Enum.filter(fn adjacent_numbers -> length(adjacent_numbers) === 2 end)
    |> Enum.map(fn [{part_number_1, _, _, _, _}, {part_number_2, _, _, _, _}] ->
      part_number_1 * part_number_2
    end)
  end
end
