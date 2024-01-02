open Aoc_utils

module Day9 = struct
  let difference_list nums =
    let rec helper nums acc =
      match nums with
      | a :: (b :: _ as t) -> helper t (List.append [ b - a ] acc)
      | _ -> acc
    in
    helper nums []
  ;;

  let predict_next nums =
    let rec helper (nums, acc) =
      let diffs = difference_list nums in
      match List.fold_left (fun acc n -> acc + n) 0 diffs with
      | 0 -> List.append [ 0 ] acc
      | _ -> helper (List.rev diffs, List.append [ List.hd diffs ] acc)
    in
    let ends = helper (nums, [ List.rev nums |> List.hd ]) in
    List.fold_left (fun acc n -> acc + n) 0 ends
  ;;

  let predict_prev nums =
    let rec helper (nums, acc) =
      let diffs = difference_list nums in
      match List.fold_left (fun acc n -> n + acc) 0 diffs with
      | 0 -> List.append [ 0 ] acc
      | _ ->
        let diffs = List.rev diffs in
        helper (diffs, List.append [ List.hd diffs ] acc)
    in
    let ends = helper (nums, [ nums |> List.hd ]) in
    List.fold_left (fun acc n -> n - acc) 0 ends
  ;;

  let parse_input file_path =
    List.map
      (fun line -> String.split_on_char ' ' line |> List.map (fun s -> int_of_string s))
      (AocUtils.read_lines file_path)
  ;;

  let do_part1 input =
    input |> List.map (fun l -> predict_next l) |> List.fold_left (fun acc n -> acc + n) 0
  ;;

  let do_part2 input =
    input |> List.map (fun l -> predict_prev l) |> List.fold_left (fun acc n -> acc + n) 0
  ;;
end

let%test "differences test" =
  let nums_in = [ 1; 3; 6; 10; 15; 21 ] in
  let diffs = Day9.difference_list nums_in in
  Alcotest.(check (list int)) "diffs" [ 2; 3; 4; 5; 6 ] (List.rev diffs)
;;

let%test "predict next test" =
  Alcotest.(check int) "pred" 28 (Day9.predict_next [ 1; 3; 6; 10; 15; 21 ])
;;

let%test "predict prev test" =
  Alcotest.(check int) "pred" 5 (Day9.predict_prev [ 10; 13; 16; 21; 30; 45 ])
;;

let%test "parse test" =
  let input_file = AocUtils.resource_file "sample.txt" in
  let exp =
    [ [ 0; 3; 6; 9; 12; 15 ]; [ 1; 3; 6; 10; 15; 21 ]; [ 10; 13; 16; 21; 30; 45 ] ]
  and got = Day9.parse_input input_file in
  Alcotest.(check int) "len" 3 (List.length got);
  List.iteri
    (fun i (exp, got) ->
      Alcotest.(check (list int)) (Printf.sprintf "list[%d]" i) exp got)
    (List.combine exp got)
;;

let%test "part1 test" =
  let input = AocUtils.resource_file "sample.txt" |> Day9.parse_input in
  let result = Day9.do_part1 input in
  Alcotest.(check int) "sum" 114 result
;;

let%test "part2 test" =
  let input = AocUtils.resource_file "sample.txt" |> Day9.parse_input in
  let result = Day9.do_part2 input in
  Alcotest.(check int) "sum" 2 result
;;
