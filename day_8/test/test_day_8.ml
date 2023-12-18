open Alcotest
open Qbuffer
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

module LoopMap = Map.Make (String)

(* reverse all the subsets between elements with value 0 *)
let fix_loop_stats list =
  let split_lists =
    List.fold_left
      (fun acc n ->
        match n with
        | 0 -> List.append [ []; [ 0 ] ] acc
        | n ->
          let new_head = List.hd acc |> List.append [ n ] in
          List.append [ new_head ] (List.tl acc))
      [ [] ]
      list
  in
  List.rev split_lists |> List.flatten
;;

(* the idea is to get a circulat buffer of distances to next 'Z' for the loop of *)
(* each start node and then increment the buffer ssimultaneously my the *)
(* biggest number until they all say 0 *)
let get_loop_stats key steps map =
  let rec helper key stats visited_map steps steps_original map count =
    let stats = List.append stats [ count ] in
    let key_step_str = key ^ string_of_int (String.length steps) in
    let visited_map = LoopMap.add key_step_str (List.length stats - 1) visited_map in
    let step, steps = pop_step steps steps_original in
    let key = next_key key step map in
    let key_step_str = key ^ string_of_int (String.length steps) in
    let entry = LoopMap.find_opt key_step_str visited_map in
    match entry with
    | Some idx -> fix_loop_stats stats, idx
    | None ->
      let count =
        match key.[2] with
        | 'Z' -> 0
        | _ -> count + 1
      in
      helper key stats visited_map steps steps_original map count
  in
  helper key [] LoopMap.empty steps steps map 1
;;

let rec gcd a = function
  | 0 -> a
  | b -> gcd b (a mod b)
;;

let lcm a b = a * b / gcd a b

let do_part2 qbufs =
  (* worked, but way too slow *)
  (* let rec helper qbufs count = *)
  (*   let max_dist = *)
  (*     List.fold_left *)
  (*       (fun acc qbuf -> *)
  (*         match Qbuffer.front qbuf with *)
  (*         | f when f > acc -> f *)
  (*         | _ -> acc) *)
  (*       0 *)
  (*       qbufs *)
  (*   in *)
  (*   match max_dist with *)
  (*   | 0 -> count *)
  (*   | _ -> *)
  (*     let qbufs = List.map (fun qbuf -> Qbuffer.shiftn max_dist qbuf) qbufs in *)
  (*     helper qbufs (count + max_dist) *)
  (* in *)
  (* helper qbufs 0 *)
  List.fold_left (fun acc qbuf -> lcm acc (Qbuffer.loop_len qbuf)) 1 qbufs
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

let test_part2_loop_stats () =
  let steps, map = parse_input (read_lines "resources/sample3.txt") in
  let start_keys = get_start_keys map in
  let loop_stats, loop_idx = get_loop_stats (List.hd start_keys) steps map in
  print_endline (List.hd start_keys);
  check int "loop idx[0]" 1 loop_idx;
  check (list int) "keys[0]" [ 2; 1; 0 ] loop_stats;
  let loop_stats, loop_idx = get_loop_stats (List.hd (List.tl start_keys)) steps map in
  print_endline (List.hd (List.tl start_keys));
  check int "loop idx[1]" 1 loop_idx;
  check (list int) "keys[1]" [ 3; 2; 1; 0; 2; 1; 0 ] loop_stats
;;

let test_part3_qbufs () =
  let helper qbufs shift =
    let qbufs = List.map (fun qbuf -> Qbuffer.shiftn shift qbuf) qbufs in
    List.map (fun qbuf -> Qbuffer.front qbuf) qbufs, qbufs
  in
  let steps, map = parse_input (read_lines "resources/sample3.txt") in
  let start_keys = get_start_keys map in
  let qbufs =
    List.map
      (fun key ->
        let stats, loop_idx = get_loop_stats key steps map in
        Qbuffer.create (Array.of_list stats) loop_idx)
      start_keys
  in
  let front, qbufs = helper qbufs 0 in
  Alcotest.(check (list int)) "step 0" [ 2; 3 ] front;
  let front, qbufs = helper qbufs 3 in
  Alcotest.(check (list int)) "step 1" [ 1; 0 ] front;
  let front, qbufs = helper qbufs 1 in
  Alcotest.(check (list int)) "step 2" [ 0; 2 ] front;
  let front, _ = helper qbufs 2 in
  Alcotest.(check (list int)) "step 3" [ 0; 0 ] front
;;

let test_part2_sample3 () =
  let steps, map = parse_input (read_lines "resources/sample3.txt") in
  let start_keys = get_start_keys map in
  let qbufs =
    List.map
      (fun key ->
        let stats, loop_idx = get_loop_stats key steps map in
        Qbuffer.create (Array.of_list stats) loop_idx)
      start_keys
  in
  Alcotest.(check int) "trying" 6 (do_part2 qbufs)
;;

let test_part2_main () =
  let steps, map = parse_input (read_lines "resources/main.txt") in
  let start_keys = get_start_keys map in
  let qbufs =
    List.map
      (fun key ->
        let stats, loop_idx = get_loop_stats key steps map in
        Printf.printf "%d, %d\n" loop_idx (List.length stats);
        Qbuffer.create (Array.of_list stats) loop_idx)
      start_keys
  in
  Alcotest.(check int) "trying" 0 (do_part2 qbufs)
;;

let test_lcm () =
  let input = [ 2, 3; 3, 3; 6, 5; 8, 3 ]
  and exp = [ 6; 3; 30; 24 ] in
  List.iteri
    (fun i ((a, b), e) ->
      let check_name = Printf.sprintf "LCM [%d]" i in
      Alcotest.(check int) check_name e (lcm a b))
    (List.combine input exp)
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
        ; test_case "part2 loop stats" `Quick test_part2_loop_stats
        ; test_case "part2 loop stats" `Quick test_part3_qbufs
        ; test_case "lcm test" `Quick test_lcm
        ; test_case "part2 main" `Quick test_part2_main
        ] )
    ]
;;
