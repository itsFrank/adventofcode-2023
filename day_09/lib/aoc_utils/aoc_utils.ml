module AocUtils = struct
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

  let sprint_list (to_string_fun : 'a -> string) (list : 'a list) : string =
    let content = List.fold_left (fun acc n -> acc ^ to_string_fun n ^ "; ") "" list in
    "[ " ^ content ^ " ]"
  ;;

  let resource_file file =
    let rec helper current_dir =
      if Sys.file_exists (Filename.concat current_dir "resources")
         && Sys.is_directory (Filename.concat current_dir "resources")
      then Some current_dir
      else (
        let parent_dir = Filename.dirname current_dir in
        if parent_dir <> current_dir
        then (* If there is a parent directory *)
          helper parent_dir
        else None)
    in
    let res_dir =
      match helper (Sys.getcwd ()) with
      | Some v -> v
      | None ->
        failwith (Format.sprintf "no resource dir found above '%s'" (Sys.getcwd ()))
    in
    Filename.concat res_dir ( Filename.concat "resources" file )
  ;;
end

let%test "test res dir" =
  let s = AocUtils.resource_file "hello.txt" in
  Alcotest.(check bool) "pass" true (String.ends_with ~suffix:"resources/hello.txt" s)
;;
