type Mut0 = {mutable l0 : int32}
and Mut1 = {mutable l0 : float; mutable l1 : float; mutable l2 : float}
and Mut2 = {mutable l0 : int32}
and Mut3 = {mutable l0 : float}
let rec method0 (v0 : Mut0) : bool =
    let v1 : int32 = v0.l0
    let v2 : bool = v1 < 7040
    v2
and method1 (v0 : Mut0) : bool =
    let v1 : int32 = v0.l0
    let v2 : bool = v1 < 60
    v2
and method4 (v0 : float, v1 : Mut3) : bool =
    let v2 : float = v1.l0
    let v3 : bool = v2 < v0
    v3
and method5 (v0 : float, v1 : Mut3) : bool =
    let v2 : float = v1.l0
    let v3 : bool = v2 < v0
    v3
and method6 (v0 : (float []), v1 : (int32 []), v2 : float, v3 : float, v4 : float, v5 : float, v6 : float, v7 : float, v8 : float, v9 : int32) : unit =
    let v10 : float = sin v2
    let v11 : float = v7 * v10
    let v12 : float = sin v3
    let v13 : float = v11 * v12
    let v14 : float = cos v4
    let v15 : float = v13 * v14
    let v16 : float = cos v2
    let v17 : float = v8 * v16
    let v18 : float = v17 * v12
    let v19 : float = v18 * v14
    let v20 : float = v15 - v19
    let v21 : float = v7 * v16
    let v22 : float = sin v4
    let v23 : float = v21 * v22
    let v24 : float = v20 + v23
    let v25 : float = v8 * v10
    let v26 : float = v25 * v22
    let v27 : float = v24 + v26
    let v28 : float = cos v3
    let v29 : float = v6 * v28
    let v30 : float = v29 * v14
    let v31 : float = v27 + v30
    let v32 : float = v21 * v14
    let v33 : float = v25 * v14
    let v34 : float = v32 + v33
    let v35 : float = v13 * v22
    let v36 : float = v34 - v35
    let v37 : float = v18 * v22
    let v38 : float = v36 + v37
    let v39 : float = v29 * v22
    let v40 : float = v38 - v39
    let v41 : float = v17 * v28
    let v42 : float = v11 * v28
    let v43 : float = v41 - v42
    let v44 : float = v6 * v12
    let v45 : float = v43 + v44
    let v46 : float = v45 + 100.0
    let v47 : float = 1.0 / v46
    let v48 : float = 80.0 + v5
    let v49 : float = 40.0 * v47
    let v50 : float = v49 * v31
    let v51 : float = v50 * 2.0
    let v52 : float = v48 + v51
    let v53 : int32 = int32 v52
    let v54 : float = v49 * v40
    let v55 : float = 22.0 + v54
    let v56 : int32 = int32 v55
    let v57 : int32 = v56 * 160
    let v58 : int32 = v53 + v57
    let v59 : bool = v58 >= 0
    let v61 : bool =
        if v59 then
            let v60 : bool = v58 < 7040
            v60
        else
            false
    if v61 then
        let v62 : float = v0.[int v58]
        let v63 : bool = v47 > v62
        if v63 then
            v0.[int v58] <- v47
            v1.[int v58] <- v9
            ()
and method3 (v0 : (float []), v1 : (int32 []), v2 : float, v3 : float, v4 : float, v5 : float, v6 : float) : unit =
    let v7 : float =  -v5
    let v8 : Mut3 = {l0 = v7} : Mut3
    while method4(v5, v8) do
        let v10 : Mut3 = {l0 = v7} : Mut3
        while method5(v5, v10) do
            let v12 : float = v8.l0
            let v13 : float = v10.l0
            let v14 : int32 = 59
            method6(v0, v1, v2, v3, v4, v6, v12, v13, v7, v14)
            let v15 : int32 = 92
            method6(v0, v1, v2, v3, v4, v6, v5, v13, v12, v15)
            let v16 : float =  -v12
            let v17 : int32 = 47
            method6(v0, v1, v2, v3, v4, v6, v7, v13, v16, v17)
            let v18 : int32 = 61
            method6(v0, v1, v2, v3, v4, v6, v16, v13, v5, v18)
            let v19 : float =  -v13
            let v20 : int32 = 62
            method6(v0, v1, v2, v3, v4, v6, v12, v7, v19, v20)
            let v21 : int32 = 60
            method6(v0, v1, v2, v3, v4, v6, v12, v5, v13, v21)
            let v22 : float = v10.l0
            let v23 : float = v22 + 0.6
            v10.l0 <- v23
            ()
        let v24 : float = v8.l0
        let v25 : float = v24 + 0.6
        v8.l0 <- v25
        ()
    ()
and method7 (v0 : Mut0) : bool =
    let v1 : int32 = v0.l0
    let v2 : bool = v1 < 44
    v2
and method8 (v0 : Mut0) : bool =
    let v1 : int32 = v0.l0
    let v2 : bool = v1 < 160
    v2
and method2 (v0 : (float []), v1 : (int32 []), v2 : float, v3 : float, v4 : float, v5 : int32) : int32 =
    let v6 : Mut0 = {l0 = 0} : Mut0
    while method0(v6) do
        let v8 : int32 = v6.l0
        v0.[int v8] <- 0.0
        v1.[int v8] <- 46
        let v9 : int32 = v8 + 1
        v6.l0 <- v9
        ()
    let v10 : float = 20.0
    let v11 : float = -40.0
    method3(v0, v1, v2, v3, v4, v10, v11)
    let v12 : float = 10.0
    let v13 : float = 10.0
    method3(v0, v1, v2, v3, v4, v12, v13)
    let v14 : float = 5.0
    let v15 : float = 40.0
    method3(v0, v1, v2, v3, v4, v14, v15)
    System.Console.Write("u001b[H")
    let v16 : Mut2 = {l0 = v5} : Mut2
    let v17 : Mut0 = {l0 = 0} : Mut0
    while method7(v17) do
        let v19 : int32 = v17.l0
        let v20 : Mut0 = {l0 = 0} : Mut0
        while method8(v20) do
            let v22 : int32 = v20.l0
            let v23 : int32 = v19 * 160
            let v24 : int32 = v22 + v23
            let v25 : int32 = v1.[int v24]
            let v26 : bool = v25 = 59
            let v44 : string =
                if v26 then
                    let v27 : string = ";"
                    v27
                else
                    let v28 : bool = v25 = 92
                    if v28 then
                        let v29 : string = "\\"
                        v29
                    else
                        let v30 : bool = v25 = 47
                        if v30 then
                            let v31 : string = "/"
                            v31
                        else
                            let v32 : bool = v25 = 61
                            if v32 then
                                let v33 : string = "="
                                v33
                            else
                                let v34 : bool = v25 = 62
                                if v34 then
                                    let v35 : string = ">"
                                    v35
                                else
                                    let v36 : bool = v25 = 60
                                    if v36 then
                                        let v37 : string = "<"
                                        v37
                                    else
                                        let v38 : string = "."
                                        v38
            System.Console.Write(v44)
            let v45 : int32 = v16.l0
            let v46 : int32 = v45 * 31
            let v47 : int32 = v46 + v25
            let v48 : int32 = v47 % 1000003
            v16.l0 <- v48
            let v49 : int32 = v22 + 1
            v20.l0 <- v49
            ()
        System.Console.Write("\n")
        let v50 : int32 = v19 + 1
        v17.l0 <- v50
        ()
    let v51 : int32 = v16.l0
    v51
let v0 : (float []) = Array.zeroCreate<float> (7040)
let v1 : Mut0 = {l0 = 0} : Mut0
while method0(v1) do
    let v3 : int32 = v1.l0
    v0.[int v3] <- 0.0
    let v4 : int32 = v3 + 1
    v1.l0 <- v4
    ()
let v5 : (int32 []) = Array.zeroCreate<int32> (7040)
let v6 : Mut0 = {l0 = 0} : Mut0
while method0(v6) do
    let v8 : int32 = v6.l0
    v5.[int v8] <- 46
    let v9 : int32 = v8 + 1
    v6.l0 <- v9
    ()
let v10 : Mut1 = {l0 = 0.0; l1 = 0.0; l2 = 0.0} : Mut1
let v11 : Mut2 = {l0 = 0} : Mut2
let v12 : Mut0 = {l0 = 0} : Mut0
while method1(v12) do
    let v14 : int32 = v12.l0
    let struct (v15 : float, v16 : float, v17 : float) = v10.l0, v10.l1, v10.l2
    let v18 : int32 = v11.l0
    let v19 : int32 = method2(v0, v5, v15, v16, v17, v18)
    v11.l0 <- v19
    let struct (v20 : float, v21 : float, v22 : float) = v10.l0, v10.l1, v10.l2
    let v23 : float = v20 + 0.05
    let v24 : float = v21 + 0.05
    let v25 : float = v22 + 0.01
    v10.l0 <- v23
    v10.l1 <- v24
    v10.l2 <- v25
    let v26 : int32 = v14 + 1
    v12.l0 <- v26
    ()
let v27 : int32 = v11.l0
System.Console.Write("cube: " + string (60) + " frames, checksum " + string (v27) + "\n")
0
