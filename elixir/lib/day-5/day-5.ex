defmodule VentParser do
  def parse_input() do
    {:ok, file} = File.read("./input.txt")
    split_file = file |> String.split("\n", trim: true)

    coords_instances =
      List.foldl(split_file, %{}, fn line, instance_map ->
        coords_to_add = convert_line_to_coords(line)

        List.foldl(coords_to_add, instance_map, fn coord, instance_map ->
          Map.update(
            instance_map,
            coord,
            1,
            fn existing_value -> existing_value + 1 end
          )
        end)
      end)

    thermal_vents = Enum.filter(coords_instances, fn {_, v} -> v >= 2 end)
    length(thermal_vents)
  end

  @spec convert_line_to_coords(binary) :: list
  def convert_line_to_coords(line) do
    input_coords = String.split(line, "->", trim: true)
    [first_coord_str, last_coord_str] = input_coords
    {x1, y1} = convert_str_input_to_tuple(first_coord_str)
    {x2, y2} = convert_str_input_to_tuple(last_coord_str)

    # find all coords between these two if in line
    cond do
      x1 == x2 -> Enum.to_list(y1..y2) |> Enum.map(fn val -> {x1, val} end)
      y1 == y2 -> Enum.to_list(x1..x2) |> Enum.map(fn val -> {val, y1} end)
      abs(Integer.floor_div((y2 - y1), (x2-x1))) == 1 -> Enum.zip(Enum.to_list(x1..x2), Enum.to_list(y1..y2)) #calc abs of gradient for diagonals
      true -> []
    end
  end

  def convert_str_input_to_tuple(str_coords) do
    coords =
      String.split(str_coords, ",", trim: true) |> Enum.map(fn val -> String.trim(val, " ") end)

    [x, y] = coords
    {Integer.parse(x, 10) |> elem(0), Integer.parse(y, 10) |> elem(0)}
  end
end
