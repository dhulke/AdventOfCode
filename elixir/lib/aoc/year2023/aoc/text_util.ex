defmodule Aoc.TextUtil do
  @moduledoc false

  def lines(input) do
    String.split(input, "\n", trim: true)
  end

  def is_digit(str) do
    case Integer.parse(str) do
      {_, ""} -> true
      _ -> false
    end
  end
end
