open Alcotest
module StringMap = Map.Make (String)

let read_lines filename : string list =
  let in_channel = open_in filename in
  let rec read_lines_helper acc =
    try
      let line = input_line in_channel in
      read_lines_helper (line :: acc)
    with
    | End_of_file -> acc
  in
  let lines = read_lines_helper [] in
  close_in in_channel;
  List.rev lines
;;

let rec pop_n list n : string list =
  match list, n with
  | _, 0 -> list
  | _ :: tl, _ -> pop_n tl (n - 1)
  | [], _ -> []
;;

let rec make_map_from_lines lines map =
  match lines with
  | [] -> map
  | next :: tl ->
    let key = String.sub next 0 3
    and left = String.sub next 7 3
    and right = String.sub next 12 3 in
    make_map_from_lines tl (StringMap.add key (left, right) map)
;;

let parse_input lines =
  let steps =
    match lines with
    | [] -> ""
    | first :: _ -> first
  in
  let mapping = pop_n lines 2 in
  let map = make_map_from_lines mapping StringMap.empty in
  steps, map
;;

(* part 1 *)
let pop_step steps_current steps_original =
  let steps_current =
    match String.length steps_current with
    | 0 -> steps_original
    | _ -> steps_current
  in
  let step = steps_current.[0]
  and steps_next = String.sub steps_current 1 (String.length steps_current - 1) in
  step, steps_next
;;

let next_key key step map =
  let l, r = StringMap.find key map in
  match step with
  | 'L' -> l
  | 'R' -> r
  | _ -> failwith "bad instruction"
;;

let rec do_part1 key steps_current steps_original map count =
  match key with
  | "ZZZ" -> count
  | _ ->
    let step, steps_next = pop_step steps_current steps_original in
    let next = next_key key step map in
    do_part1 next steps_next steps_original map (count + 1)
;;

(* part 2 *)
let get_start_keys map =
  StringMap.filter
    (fun key _ ->
      match key.[2] with
      | 'A' -> true
      | _ -> false)
    map
  |> StringMap.bindings
  |> List.map (fun (k, _) -> k)
;;

(* the idea is to get a circulat buffer of distances to next 'Z' for the loop of *)
(* each start node and then increment the buffer ssimultaneously my the *)
(* biggest number until they all say 0 *)
let rec get_loop_stats stats key start_key steps_current steps_original map count =
  if key == start_key
  then (* TODO: clean up stats *)
    stats
  else (
    let count =
      match key.[2] with
      | 'Z' -> 0
      | _ -> count
    in
    let stats = List.append stats [ count ] in
    let step, steps_next = pop_step steps_current steps_original in
    let key = next_key key step map in
    get_loop_stats stats key start_key steps_next steps_original map (count + 1))
;;

let rec do_part2 keys steps_current steps_original map count =
  let step, steps_next = pop_step steps_current steps_original in
  let next_keys = List.map (fun key -> next_key key step map) keys in
  match List.fold_left (fun acc key -> acc && key.[2] == 'Z') true next_keys with
  | true -> count + 1
  | false -> do_part2 next_keys steps_next steps_original map count + 1
;;

let test_parse () =
  let steps, map = parse_input (read_lines "resources/sample1.txt") in
  check string "steps expectes" "RL" steps;
  check int "maps" 7 (StringMap.cardinal map);
  let exp =
    [ "AAA", ("BBB", "CCC")
    ; "BBB", ("DDD", "EEE")
    ; "CCC", ("ZZZ", "GGG")
    ; "DDD", ("DDD", "DDD")
    ; "EEE", ("EEE", "EEE")
    ; "GGG", ("GGG", "GGG")
    ; "ZZZ", ("ZZZ", "ZZZ")
    ]
  in
  let exp_iter = ref exp in
  StringMap.iter
    (fun key (l, r) ->
      match !exp_iter with
      | [] -> failwith "exp should not be empty"
      | (key_exp, (l_exp, r_exp)) :: tl ->
        check string "match keys" key_exp key;
        check string "match left" l_exp l;
        check string "match keys" r_exp r;
        exp_iter := tl)
    map
;;

let test_part1_sample1 () =
  let steps, map = parse_input (read_lines "resources/sample1.txt") in
  let count = do_part1 "AAA" steps steps map 0 in
  check int "steps" 2 count
;;

let test_part1_sample2 () =
  let steps, map = parse_input (read_lines "resources/sample2.txt") in
  let count = do_part1 "AAA" steps steps map 0 in
  check int "steps" 6 count
;;

let test_part1_main () =
  let steps, map = parse_input (read_lines "resources/main.txt") in
  let count = do_part1 "AAA" steps steps map 0 in
  check int "steps" 14257 count
;;

let test_part2_initial_nodes () =
  let _, map = parse_input (read_lines "resources/sample3.txt") in
  let nodes = get_start_keys map
  and exp = [ "11A"; "22A" ] in
  let exp_iter = ref exp in
  List.iter
    (fun n ->
      match !exp_iter with
      | s :: tl ->
        check string "node" s n;
        exp_iter := tl
      | _ -> failwith "should not happern")
    nodes
;;

let test_part2_sample3 () =
  let steps, map = parse_input (read_lines "resources/sample3.txt") in
  let count = do_part2 (get_start_keys map) steps steps map 0 in
  check int "count" 6 count
;;

let test_part2_main () =
  let steps, map = parse_input (read_lines "resources/main.txt") in
  let count = do_part2 (get_start_keys map) steps steps map 0 in
  check int "count" 1 count
;;

let () =
  run
    "Day8"
    [ ( "hello test"
      , [ test_case "parse input" `Quick test_parse
        ; test_case "part1 sample 1" `Quick test_part1_sample1
        ; test_case "part1 sample 2" `Quick test_part1_sample2
        ; test_case "part1 main" `Quick test_part1_main
        ; test_case "part2 start nodes" `Quick test_part2_initial_nodes
        ; test_case "part2 sample3" `Quick test_part2_sample3
        ; test_case "part2 sample3" `Quick test_part2_main
        ] )
    ]
;;
