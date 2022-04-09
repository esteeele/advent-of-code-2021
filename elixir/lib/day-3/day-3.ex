defmodule Diagnostics do
  defmodule CountHolder do
    defstruct num0: 0, num1: 0

    def put_value(value, count) do
      case value do
        0 -> %CountHolder{count | num0: count.num0 + 1}
        1 -> %CountHolder{count | num1: count.num1 + 1}
        _ -> count
      end
    end
  end

  def calc_diagnostics() do
    {:ok, file} = File.read("./input.txt")
    split_file = file |> String.split("\n", trim: true)
    [head | _tail] = split_file
    row_length = String.length(head)

    accumulator = build_accumulator(row_length, [])
    final_count = calculate_overall_counts_for_each_position(split_file, accumulator)
    IO.inspect(final_count)

    gamma_str = List.foldl(final_count, "", fn x, acc -> sum_gamma(x, acc) end)
    epsilon_str = List.foldl(final_count, "", fn x, acc -> sum_epsilon(x, acc) end)

    # part 1 answer
    IO.puts(compute_multiplication(gamma_str, epsilon_str))

    # part2 filter list down based on overall counts for each position in row
    find_o2 = fn count -> if (count.num1 >= count.num0) do 1 else 0 end end
    find_co2 = fn count -> if (count.num1 >= count.num0) do 0 else 1 end end

    o2_rating = calc_gas_rating(0, split_file, find_o2)
    co2_rating = calc_gas_rating(0, split_file, find_co2)
    IO.puts(compute_multiplication(hd(o2_rating), hd(co2_rating)))
  end

  def compute_multiplication(binary_string_a, binary_string_b) do
    a_decimal = Integer.parse(binary_string_a, 2) |> elem(0)
    b_decimal = Integer.parse(binary_string_b, 2) |> elem(0)
    a_decimal * b_decimal
  end

  # TODO: definitely something functional possible here - yes there is see calc_gas_rating for how to do this better
  def sum_gamma(count_holder, acc) do
    if count_holder.num1 > count_holder.num0 do
      acc <> "1"
    else
      acc <> "0"
    end
  end

  def sum_epsilon(count_holder, acc) do
    if count_holder.num1 < count_holder.num0 do
      acc <> "1"
    else
      acc <> "0"
    end
  end

  def build_accumulator(iter, accumulator) do
    if iter <= 0 do
      accumulator
    else
      build_accumulator(iter - 1, accumulator ++ [%CountHolder{}])
    end
  end

  def calculate_overall_counts_for_each_position([head | tail], currentCount) do
    values_list = row_to_binary_num_list(head)
    currentCount = update_counts_with_values_in_row(values_list, currentCount)
    calculate_overall_counts_for_each_position(tail, currentCount)
  end

  def calculate_overall_counts_for_each_position([], currentCount) do
    currentCount
  end

  # count list has updated values cycled through it continuously until the end of the input
  # while iterating through each bit in the input
  # sort of like a queue maintained with size row_length
  def update_counts_with_values_in_row([binary_head | binary_tail], current_count) do
    [count_head | count_tail] = current_count
    updated_count_head = CountHolder.put_value(binary_head, count_head)
    # put new head at end of list
    update_counts_with_values_in_row(binary_tail, count_tail ++ [updated_count_head])
  end

  def update_counts_with_values_in_row([], current_count) do
    current_count
  end

  # part 2
  def calc_gas_rating(index, diagnostic_data, comparison_func) do
    # recalculate counts on filtered list
    count_head =
      List.foldl(diagnostic_data, %CountHolder{}, fn binary_string, count_holder ->
        count_holder = update_count_for_index(binary_string, index, count_holder)
      end)

    val_to_filter_on = comparison_func.(count_head)
    reduced_list = reduce_list(diagnostic_data, index, val_to_filter_on)

    case reduced_list do
      [_last_item] -> reduced_list
      [] -> "Failed to reduce list to single item!"
      _ -> calc_gas_rating(index + 1, reduced_list, comparison_func)
    end
  end

  defp reduce_list(input_list, index, expected_value) do
    Enum.filter(input_list, fn binary_nums -> apply_filter(binary_nums, index, expected_value) end)
  end

  defp apply_filter(binary_nums, index, expected_value) do
    binary_nums_list = row_to_binary_num_list(binary_nums)
    bit = List.pop_at(binary_nums_list, index) |> elem(0)
    bit == expected_value
  end

  defp update_count_for_index(binary_string, index, count_holder) do
    value = List.pop_at(String.to_charlist(binary_string), index) |> elem(0)
    CountHolder.put_value(value - 48, count_holder)
  end

  defp row_to_binary_num_list(row) do
    Enum.map(String.to_charlist(row), fn num -> num - 48 end)
  end
end
