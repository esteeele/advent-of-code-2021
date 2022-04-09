defmodule Lanternfish do
  def calc_result do
    {:ok, file} = File.read("./input.txt")
    input = file |> String.split("\n", trim: true) |> hd |> String.split(",", trim: true)
    initial_timers = Enum.map(input, fn str_num -> Integer.parse(str_num, 10) |> elem(0) end)

    #build a map with timer => number of fish at that timer to make this efficient
    hashed_input =
      List.foldl(initial_timers, %{}, fn fish_timer, instance_map ->
        Map.update(instance_map, fish_timer, 1,
          fn existing_value -> existing_value + 1 end
        )
      end)

    hashed_populations =
      List.foldl(Enum.to_list(1..256), hashed_input, fn _i, lanternfish ->
        decrease_timer_map(lanternfish)
      end)

    Enum.reduce(hashed_populations, 0, fn {_k, number_fish}, acc -> acc + number_fish end)
  end

  defp decrease_timer_map(current_pop) do
    # handle 'birth' separately i.e. timer = 0 -> pull out of map then process rest and re-update
    {birthing_fish, other_fish} = Map.pop(current_pop, 0, 0)

    #decrement counter on 'normal' fish
    updated_fish =
      List.foldl(Enum.to_list(other_fish), %{}, fn {timer, number_fish}, accumulator ->
        Map.put(accumulator, timer - 1, number_fish)
      end)

    #handle 'birthed' fish -> add new fish to counter 8, increase fish at 6 as well
    updated_fish =
      Map.update(updated_fish, 6, birthing_fish, fn existing_value ->
        existing_value + birthing_fish
      end)

    Map.put(updated_fish, 8, birthing_fish)
  end
end
