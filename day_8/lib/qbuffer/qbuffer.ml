(* Circular buffer that loops at an arbitrary index, i.e. circle with tail start, kinda Q-shaped *)
module Qbuffer = struct
  type t =
    { buf : int array
    ; loop_idx : int
    ; idx : int
    }

  let create buf loop_idx = { buf; loop_idx; idx = 0 }
  let front qbuf = qbuf.buf.(qbuf.idx)
  let loop_len qbuf = Array.length qbuf.buf - qbuf.loop_idx
  let full_len qbuf = Array.length qbuf.buf

  let shift qbuf =
    match qbuf.idx + 1 with
    | i when i < Array.length qbuf.buf -> { qbuf with idx = i }
    | i when i = Array.length qbuf.buf -> { qbuf with idx = qbuf.loop_idx }
    | _ -> failwith "Bad index value"
  ;;

  let shiftn n qbuf =
    print_endline
      (Printf.sprintf "{idx: %d, loop_idx: %d}, n: %d" qbuf.idx qbuf.loop_idx n);
    match qbuf.idx + n with
    | i when i < Array.length qbuf.buf -> { qbuf with idx = i }
    | i when i = Array.length qbuf.buf -> { qbuf with idx = qbuf.loop_idx }
    | i ->
      let loop_len = Array.length qbuf.buf - qbuf.loop_idx in
      print_endline (Printf.sprintf "   i: %d, loop_len: %d" i loop_len);
      let i = (i - qbuf.loop_idx) mod loop_len in
      { qbuf with idx = i + qbuf.loop_idx }
  ;;
end

let%test "qbuffer shift" =
  let qbuf = ref (Qbuffer.create [| 1; 2; 3; 4; 5 |] 2)
  and exp_sequence = [ 1; 2; 3; 4; 5; 3; 4; 5; 3; 4; 5; 3; 4; 5 ] in
  List.iteri
    (fun i exp ->
      Alcotest.(check int) (Printf.sprintf "iter [ %d ]" i) exp (Qbuffer.front !qbuf);
      qbuf := Qbuffer.shift !qbuf)
    exp_sequence
;;

let%test "qbuffer shiftn" =
  let qbuf = Qbuffer.create [| 1; 2; 3; 4; 5; 6 |] 3 in
  Alcotest.(check int) "shift before loop" 3 (Qbuffer.shiftn 2 qbuf |> Qbuffer.front);
  Alcotest.(check int) "shift at loop" 4 (Qbuffer.shiftn 3 qbuf |> Qbuffer.front);
  Alcotest.(check int) "shift past loop" 5 (Qbuffer.shiftn 4 qbuf |> Qbuffer.front);
  Alcotest.(check int) "shift to end" 6 (Qbuffer.shiftn 5 qbuf |> Qbuffer.front);
  Alcotest.(check int) "shift to end+1" 4 (Qbuffer.shiftn 6 qbuf |> Qbuffer.front);
  Alcotest.(check int) "shift to end+2" 5 (Qbuffer.shiftn 7 qbuf |> Qbuffer.front);
  Alcotest.(check int) "shift multiple loops" 5 (Qbuffer.shiftn 22 qbuf |> Qbuffer.front)
;;

let%test "qbuffer shift no tail" =
  let qbuf = ref (Qbuffer.create [| 1; 2; 3 |] 0)
  and exp_sequence = [ 1; 2; 3; 1; 2; 3; 1; 2; 3; 1; 2; 3; 1; 2; 3 ] in
  List.iteri
    (fun i exp ->
      Alcotest.(check int) (Printf.sprintf "iter [ %d ]" i) exp (Qbuffer.front !qbuf);
      qbuf := Qbuffer.shift !qbuf)
    exp_sequence
;;

let%test "qbuffer shiftn no tail" =
  let qbuf = ref (Qbuffer.create [| 1; 2; 3 |] 0)
  and exp_sequence = [ 1; 3; 2; 1; 3; 2; 1; 3; 2; 1; 3; 2; 1; 3; 2; 1; 3; 2 ] in
  List.iteri
    (fun i exp ->
      print_endline (string_of_int (Qbuffer.front !qbuf));
      Alcotest.(check int) (Printf.sprintf "iter [ %d ]" i) exp (Qbuffer.front !qbuf);
      qbuf := Qbuffer.shiftn 2 !qbuf)
    exp_sequence
;;

let%test "edge case" =
  let qbuf = Qbuffer.create [| 2; 1; 0 |] 1 |> Qbuffer.shiftn 2 in
  Alcotest.(check int) (Printf.sprintf "first") 0 (Qbuffer.front qbuf);
  let qbuf = Qbuffer.shiftn 2 qbuf in
  Alcotest.(check int) (Printf.sprintf "second") 0 (Qbuffer.front qbuf)
;;
