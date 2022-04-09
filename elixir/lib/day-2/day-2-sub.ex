defmodule SubmarineRunner do

  defmodule Submarine do
    defstruct x: 0, y: 0, aim: 0

    def handleCommandPart1(direction, value, submarine) do
      case direction do
        "forward" -> %Submarine{submarine | x: submarine.x + value}
        "down" -> %Submarine{submarine | y: submarine.y + value}
        "up" -> %Submarine{submarine | y: submarine.y - value}
        _ -> IO.puts("Unknown direction: " <> direction <> " doing nothing")
        submarine
      end
    end

    def handleCommandPart2(direction, value, submarine) do
      case direction do
        "forward" -> %Submarine{submarine | x: submarine.x + value, y: submarine.y + (submarine.aim * value)}
        "down" -> %Submarine{submarine | aim: submarine.aim + value}
        "up" -> %Submarine{submarine | aim: submarine.aim - value}
        _ -> IO.puts("Unknown direction: " <> direction <> " doing nothing")
        submarine
      end
    end
  end

  def swim() do
    {:ok, file} = File.read("./input.txt")
    split_file = file |> String.split("\n", trim: true)

    atlas_v2 = iterate_command(split_file, %Submarine{})
    atlas_v2.x * atlas_v2.y
  end

  def iterate_command([head | tail], submarine) do
    [direction, value_str] = String.split(head, " ")
    {value_int, ""} = Integer.parse(value_str)
    submarine = Submarine.handleCommandPart2(direction, value_int, submarine)
    iterate_command(tail, submarine)
  end

  def iterate_command([], submarine) do
    submarine
  end
end
