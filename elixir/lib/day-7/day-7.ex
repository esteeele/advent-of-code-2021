defmodule Crabs do
  def align_crabs do
    #iterate from max to min in the crabs position then calc fuel for each
    #can keep a running min to avoid pointlessly calculating all values

    {:ok, file} = File.read("./input.txt")
    input = file |> String.split("\n", trim: true) |> hd |> String.split(",", trim: true)
    crab_positions = Enum.map(input, fn str_num -> Integer.parse(str_num, 10) |> elem(0) end)

    max_crab_pos = Enum.max(crab_positions)
    min_crab_pos = Enum.min(crab_positions)
    big_integer_value = 100000000000000 #elixir doesn't really have a max integer value so just defining a big one

    calc_absolute_difference = fn crab_position, target_position -> abs(target_position - crab_position) end
    calc_fuel_increasing_with_distance = fn crab_position, target_position ->
      diff = abs(target_position - crab_position)
      Integer.floor_div(diff * (diff + 1), 2) #triangular numbers
    end

    #find minimum fuel usage for crabs Part 1
    part1_res = Enum.reduce(Enum.to_list(min_crab_pos..max_crab_pos), big_integer_value,
      fn position, local_min ->
        calc_fuel_for_position(position, local_min, crab_positions, calc_absolute_difference)
    end)

    #Part2 diff increases with distance
    part2_res = Enum.reduce(Enum.to_list(min_crab_pos..max_crab_pos), big_integer_value,
      fn position, local_min ->
        calc_fuel_for_position(position, local_min, crab_positions, calc_fuel_increasing_with_distance)
    end)

    %{
      part1: part1_res,
      part2: part2_res
    }
  end

  defp calc_fuel_for_position(target_position, local_min, crabs, diff_function) do
    Enum.reduce_while(crabs, 0, fn crab_pos, acc ->
      fuel_used = diff_function.(crab_pos, target_position) + acc

      if (fuel_used > local_min) do
        {:halt, local_min} #v. handy :)
      else
        {:cont, fuel_used}
      end
     end)
  end
end
