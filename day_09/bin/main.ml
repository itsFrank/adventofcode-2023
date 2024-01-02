open Aoc_utils
open Day_9

let () =
  let p1_input = AocUtils.resource_file "main.txt" |> Day9.parse_input in
  let p1_result = Day9.do_part1 p1_input in
  print_endline (Printf.sprintf "\nPart 1: %d" p1_result);
  
  let p2_input = AocUtils.resource_file "main.txt" |> Day9.parse_input in
  let p2_result = Day9.do_part2 p2_input in
  print_endline (Printf.sprintf "\nPart 2: %d" p2_result)

;;
