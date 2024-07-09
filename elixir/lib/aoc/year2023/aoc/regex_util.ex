defmodule Aoc.RegexUtil do
  @moduledoc false

  @doc "Scans subject and returns a list of matched indexes and length of matches"
  def scan_slice_index(pattern, subject) do
    pattern
    |> Regex.scan(subject, return: :index)
    |> Enum.map(&hd/1)
  end

end
