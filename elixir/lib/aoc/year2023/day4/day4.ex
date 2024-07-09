defmodule Aoc.Year2023.Day4.Day4 do
  @moduledoc false

  alias Aoc.TextUtil

  def part1(input) do
    input
    |> card_wins_map()
    |> Map.values()
    |> Enum.map(&trunc(:math.pow(2, &1 - 1)))
    |> Enum.sum()
  end

  def part2(input) do
    card_wins_map = card_wins_map(input)
    total_cards = map_size(card_wins_map)

    card_wins_map
    |> Enum.map(fn {card_id, wins} ->
      total_scratch_cards(card_wins_map, total_cards, card_id, wins)
    end)
    |> Enum.sum()
    |> Kernel.+(total_cards)
  end

  defp total_scratch_cards(_, _, _, 0), do: 0
  defp total_scratch_cards(card_wins_map, total_cards, card_id, wins) do
    next_card_id = min(card_id + 1, total_cards)
    last_card_id = min(card_id + wins, total_cards)
    cards_won = last_card_id - next_card_id + 1

    next_card_id..last_card_id
    |> Enum.map(&total_scratch_cards(card_wins_map, total_cards, &1, card_wins_map[&1]))
    |> Enum.sum()
    |> Kernel.+(cards_won)
  end

  defp card_wins_map(input) do
    input
    |> TextUtil.lines()
    |> Enum.map(&String.split(&1, ": "))
    |> Enum.map(fn [card_string | [contents]] ->
      {card_id(card_string), String.split(contents, " | ")}
    end)
    |> Enum.map(fn {card_id, [winning | [selected]]} ->
      {card_id, intersection_size(to_map_set(winning), to_map_set(selected))}
    end)
    |> Map.new()
  end

  defp card_id(card_string) do
    [_ | [id]] = String.split(card_string)
    String.to_integer(id)
  end

  defp to_map_set(string_list) do
    MapSet.new(String.split(string_list))
  end

  defp intersection_size(map_set_a, map_set_b) do
    MapSet.size(MapSet.intersection(map_set_a, map_set_b))
  end
end
