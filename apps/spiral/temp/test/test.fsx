let rec closure1 () () : unit =
    let v0 : (string -> unit) = System.Console.WriteLine
    let v1 : string = "test"
    v0 v1
and closure0 () () : int32 =
    let v3 : unit = ()
    let v4 : (unit -> unit) = closure1()
    let v5 : unit = (fun () -> v4 (); v3) ()
    0
let v0 : (unit -> int32) = closure0()
()
