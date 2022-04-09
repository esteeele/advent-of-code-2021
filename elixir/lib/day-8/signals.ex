defmodule Signals do
  defmodule SignalBoardLetters do
    defstruct top: '',
              top_left: '',
              top_right: '',
              middle: '',
              bottom_left: '',
              bottom_right: '',
              bottom: ''
  end

  #this works ... but omg I've not used this language properly
  def work_out_signals() do
    {:ok, file} = File.read("./input.txt")
    inputs = file |> String.split("\n", trim: true)

    Enum.reduce(inputs, 0, fn input_line, acc ->
      [inputs, outputs] = String.split(input_line, "|", trim: true)
      encoded_inputs = String.split(inputs, " ", trim: true)

      # build up which letters are in which position logically starting with 1
      one_letters = Enum.find(encoded_inputs, fn number -> String.length(number) == 2 end)
      ones_chars = String.to_charlist(one_letters)
      seven_letters = Enum.find(encoded_inputs, fn number -> String.length(number) == 3 end)

      top_long_letter = find_diff(String.to_charlist(seven_letters), ones_chars)

      fours_letters =
        String.to_charlist(Enum.find(encoded_inputs, fn number -> String.length(number) == 4 end))

      nine_six_and_zero = Enum.filter(encoded_inputs, fn number -> String.length(number) == 6 end)

      nine =
        String.to_charlist(
          hd(Enum.filter(nine_six_and_zero, fn number -> contains_all(number, fours_letters) end))
        )

      six =
        String.to_charlist(
          hd(Enum.filter(nine_six_and_zero, fn number -> !contains_all(number, ones_chars) end))
        )

      zero =
        String.to_charlist(
          hd(
            Enum.filter(nine_six_and_zero, fn number ->
              String.to_charlist(number) != nine and String.to_charlist(number) != six
            end)
          )
        )

      bottom_right_letter = Enum.filter(ones_chars, fn number -> number in six end)
      top_right_letter = Enum.filter(ones_chars, fn char -> char not in bottom_right_letter end)

      diff_nine_and_six = find_diff(nine, six)

      bottom_left_letter =
        Enum.filter(diff_nine_and_six, fn number -> number not in top_right_letter end)

      bottom_letter = find_diff(nine, fours_letters ++ top_long_letter)

      middle_letter = find_diff(zero, six ++ top_right_letter)
      top_left_letter = find_diff(fours_letters, ones_chars ++ middle_letter)

      all_known_letters = [
        top_long_letter,
        bottom_right_letter,
        top_right_letter,
        top_left_letter,
        bottom_left_letter,
        bottom_letter
      ]

      IO.inspect(all_known_letters)
      two_three_five = Enum.filter(encoded_inputs, fn number -> String.length(number) == 5 end)
      IO.inspect(two_three_five)

      three =
        String.to_charlist(hd(Enum.filter(two_three_five, fn letters ->
          !String.contains?(letters, List.to_string(top_left_letter)) and
          !String.contains?(letters, List.to_string(bottom_left_letter))
        end)))

      two =
        String.to_charlist(hd(Enum.filter(two_three_five, fn letters ->
          !String.contains?(letters, List.to_string(top_left_letter)) and
          !String.contains?(letters, List.to_string(bottom_right_letter))
        end)))

      five =
        String.to_charlist(hd(Enum.filter(two_three_five, fn letters ->
          !String.contains?(letters, List.to_string(top_right_letter)) and
          !String.contains?(letters, List.to_string(bottom_left_letter))
        end)))

      IO.inspect([one_letters, two, three, fours_letters, five, six, seven_letters, nine])

      encoded_numbers = String.split(outputs, " ", trim: true)

      number_occurances =
        Enum.reduce(encoded_numbers, "", fn encoded_number, encoded_numbers_acc ->
          # based on these known values find where each letter could be ...
          encoded_number_len = String.length(encoded_number)
          encoded_num_chars = String.to_charlist(encoded_number)

          case encoded_number_len do
            2 ->
              encoded_numbers_acc <> "1"

            3 ->
              encoded_numbers_acc <> "7"

            4 ->
              encoded_numbers_acc <> "4"

            5 ->
              sorted_word = Enum.sort(encoded_num_chars)
              sorted_three = Enum.sort(three)
              sorted_two = Enum.sort(two)
              sorted_five = Enum.sort(five)
              case sorted_word do
                ^sorted_three -> encoded_numbers_acc <> "3"
                ^sorted_two -> encoded_numbers_acc <> "2"
                ^sorted_five -> encoded_numbers_acc <> "5"
              end

            6 ->
              sorted_word = Enum.sort(encoded_num_chars)
              sorted_nine = Enum.sort(nine)
              sorted_six = Enum.sort(six)
              sorted_zero = Enum.sort(zero)
              case sorted_word do
                ^sorted_nine -> encoded_numbers_acc <> "9"
                ^sorted_six -> encoded_numbers_acc <> "6"
                ^sorted_zero -> encoded_numbers_acc <> "0"
              end

            7 ->
              encoded_numbers_acc <> "8"
          end
        end)

      acc + String.to_integer(number_occurances, 10)
    end)
  end

  defp contains_all(input, comparator) do
    input = String.to_charlist(input)

    length(
      Enum.filter(comparator, fn char ->
        char not in input
      end)
    ) == 0
  end

  defp find_diff(string_one, string_two) do
    # never going to work ...
    Enum.filter(string_one, fn char -> char not in string_two end) ++
      Enum.filter(string_two, fn char -> char not in string_one end)
  end

  def part1(inputs) do
    Enum.reduce(inputs, 0, fn input_line, acc ->
      [_inputs, outputs] = String.split(input_line, "|", trim: true)
      encoded_numbers = String.split(outputs, " ", trim: true)

      number_occurances =
        Enum.reduce(encoded_numbers, 0, fn encoded_number, encoded_numbers_acc ->
          length_number = String.length(encoded_number)

          # based on these known values find where each letter could be ...
          case length_number do
            2 -> encoded_numbers_acc + 1
            3 -> encoded_numbers_acc + 1
            4 -> encoded_numbers_acc + 1
            7 -> encoded_numbers_acc + 1
            _ -> encoded_numbers_acc
          end
        end)

      acc + number_occurances
    end)
  end
end
