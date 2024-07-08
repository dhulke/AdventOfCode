defmodule Aoc.Year2023.Day2.Day2 do
  @moduledoc false

  @max_per_game %{red: 12, green: 13, blue: 14}

  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Stream.filter(&valid_game?/1)
    |> Stream.map(&game_id/1)
    |> Enum.sum()
  end

  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Stream.map(&min_valid_set_of_cubes/1)
    |> Stream.map(&(&1.red * &1.green * &1.blue))
    |> Enum.sum()
  end

  defp min_valid_set_of_cubes(line) do
    [_, games] = String.split(line, ":")

    games
    |> String.split(";")
    |> Enum.map(&extract_game/1)
    |> max_cubes_of_all_games()
  end

  defp valid_game?(line) do
    %{red: red, green: green, blue: blue} = min_valid_set_of_cubes(line)

    red <= @max_per_game.red and green <= @max_per_game.green and blue <= @max_per_game.blue
  end

  defp extract_game(game_line) do
    game_line
    |> String.split(",", trim: true)
    |> Enum.map(&String.trim/1)
    |> Enum.reduce(
      %{red: 0, green: 0, blue: 0},
      fn cube_line, acc ->
        case String.split(cube_line, " ") do
          [n, "red"] -> Map.put(acc, :red, String.to_integer(n))
          [n, "green"] -> Map.put(acc, :green, String.to_integer(n))
          [n, "blue"] -> Map.put(acc, :blue, String.to_integer(n))
        end
      end
    )
  end

  defp max_cubes_of_all_games(games) do
    Enum.reduce(
      games,
      %{red: 0, green: 0, blue: 0},
      fn %{red: red, green: green, blue: blue}, acc ->
        %{acc | red: max(red, acc.red), green: max(green, acc.green), blue: max(blue, acc.blue)}
      end
    )
  end

  defp game_id(line) do
    ["Game " <> id, _] = String.split(line, ":")
    String.to_integer(id)
  end
end
