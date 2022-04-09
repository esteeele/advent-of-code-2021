defmodule Squidgame do
  defmodule BoardValue do
    defstruct value: -1, marked: false, board: -1, row_index: -1, column_index: -1

    def init_value(value, board, row, col) do
      %BoardValue{value: value, board: board, row_index: row, column_index: col}
    end

    def mark_value(board_value) do
      %BoardValue{board_value | marked: true}
    end
  end

  @row_size 5

  def play() do
    {:ok, file} = File.read("./input.txt")
    split_file = file |> String.split("\n", trim: true)
    numbers_called = convert_string_to_list_of_integers(hd(split_file), ",")

    # a big list of all the values in the input
    board_input =
      List.foldl(tl(split_file), [], fn value, acc ->
        acc ++ convert_string_to_list_of_integers(value, " ")
      end)

    board = parse_inital_input(board_input, 0, [])

    # loop through numbers called look in each bingo table then mark as read,
    # after each number check if any board is complete
    fastest_scoring_board =
      iterate_through_numbers_called_first_winner(numbers_called, board, %{})

    IO.inspect(fastest_scoring_board)

    # part 2
    # should probably generate one results map for both but lazy

    number_boards = Integer.floor_div(length(board_input) - 1, @row_size * @row_size)

    slowest_scoring_board =
      iterate_through_numbers_called_last_winner(
        numbers_called,
        board,
        %{},
        0..number_boards
      )

    IO.inspect(slowest_scoring_board)
  end

  def iterate_through_numbers_called_first_winner([number_called | tail], board, results_map) do
    matching_items = find_matching_board_values(number_called, board)
    results_map = update_results_map(matching_items, results_map)
    updated_board = update_board_for_number(matching_items, board, number_called)
    winning_runs = find_winning_runs(results_map)

    case winning_runs do
      [] ->
        iterate_through_numbers_called_first_winner(tail, updated_board, results_map)

      _ ->
        winning_run_board = hd(winning_runs)
        calculate_final_board_score(number_called, updated_board, winning_run_board)
    end
  end

  def iterate_through_numbers_called_last_winner(
        [number_called | tail],
        board,
        results_map,
        boards_remaining
      ) do
    matching_items = find_matching_board_values(number_called, board)
    results_map = update_results_map(matching_items, results_map)
    updated_board = update_board_for_number(matching_items, board, number_called)
    new_winning_runs = find_winning_runs(results_map)

    not_complete_boards =
      Enum.filter(boards_remaining, fn board -> board not in new_winning_runs end)

    case not_complete_boards do
      [] ->
        IO.puts(number_called)
        # assume 2 boards didn't finish at same time
        calculate_final_board_score(
          number_called,
          updated_board,
          hd(boards_remaining)
        )

      _ ->
        iterate_through_numbers_called_last_winner(
          tail,
          updated_board,
          results_map,
          not_complete_boards
        )
    end
  end

  def find_winning_runs(results_map) do
    Enum.filter(results_map, fn {_, v} -> v >= @row_size end)
    |> Enum.map(fn {{board, winning_run}, v} -> board end)
  end

  def update_board_for_number(matching_items, board, number_called) do
    # bit weird - because can't update in place need to either
    # a) find indexes of matching items then replace
    # b) filter out matches then re-add to list
    list_without_matches =
      Enum.filter(board, fn board_value -> board_value.value != number_called end)

    list_without_matches ++ matching_items
  end

  def find_matching_board_values(number_called, board) do
    Enum.filter(board, fn board_value -> board_value.value == number_called end)
    |> Enum.map(fn matching_item -> BoardValue.mark_value(matching_item) end)
  end

  def update_results_map(matching_items, results_map) do
    # go through list again and build map with number complete items per board
    # key = {board, row and column id} (todo: struct?) then when one of these sets gets to 5 the game is over

    List.foldl(matching_items, results_map, fn item, results_map ->
      update_row_results =
        Map.update(
          results_map,
          {item.board, "row" <> Integer.to_string(item.row_index)},
          1,
          fn existing_value -> existing_value + 1 end
        )

      Map.update(
        update_row_results,
        {item.board, "col" <> Integer.to_string(item.column_index)},
        1,
        fn existing_value -> existing_value + 1 end
      )
    end)
  end

  def calculate_final_board_score(last_number, board, board_index) do
    unmarked_values_in_board =
      Enum.filter(board, fn board_value ->
        board_value.board == board_index and !board_value.marked
      end)
      |> Enum.map(fn item -> item.value end)

    IO.inspect(unmarked_values_in_board)

    Enum.sum(unmarked_values_in_board) * last_number
  end

  def parse_inital_input([head_value | tail], count, board) do
    board_number = Integer.floor_div(count, @row_size * @row_size)
    row_and_column = get_row_and_col(count)
    row = elem(row_and_column, 0)
    column = elem(row_and_column, 1)
    board_value = BoardValue.init_value(head_value, board_number, row, column)

    parse_inital_input(tail, count + 1, board ++ [board_value])
  end

  def parse_inital_input([], count, board) do
    board
  end

  def convert_string_to_list_of_integers(string, seperarator) do
    String.split(string, seperarator, trim: true)
    |> Enum.map(fn num -> Integer.parse(num, 10) |> elem(0) end)
  end

  def get_row_and_col(count) do
    mod_count = Integer.mod(count, @row_size * @row_size)

    row_index =
      Enum.to_list(1..@row_size) |> Enum.filter(fn index -> index * @row_size > mod_count end)

    row = hd(row_index) - 1
    column = Integer.mod(mod_count, @row_size)
    {row, column}
  end
end
