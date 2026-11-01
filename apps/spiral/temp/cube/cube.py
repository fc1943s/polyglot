kernels_main = r"""
"""
from cube_auto import *
kernels = kernels_aux + kernels_main
import os
try: # CuPy when a CUDA device answers, otherwise numpy under the same name (SPIRAL_CUDA=0 forces numpy).
    if os.environ.get('SPIRAL_CUDA', '1') == '0': raise ImportError('SPIRAL_CUDA=0')
    import cupy as cp
    if cp.cuda.runtime.getDeviceCount() < 1: raise RuntimeError('no CUDA device')
    cuda = True
except Exception:
    import numpy as cp
    cuda = False
from dataclasses import dataclass
from typing import NamedTuple, Union, Callable, Tuple
i8 = int; i16 = int; i32 = int; i64 = int; u8 = int; u16 = int; u32 = int; u64 = int; f32 = float; f64 = float; char = str; string = str

import math
def spiral_array_index(array, index):
    value = array[index]
    return value.item() if isinstance(array, cp.ndarray) and array.dtype.kind != 'O' else value
import sys
@dataclass
class Mut0:
    v0 : i32
@dataclass
class Mut1:
    v0 : f64
    v1 : f64
    v2 : f64
@dataclass
class Mut2:
    v0 : i32
@dataclass
class Mut3:
    v0 : f64
def method0(v0 : Mut0) -> bool:
    v1 = v0.v0
    del v0
    v2 = v1 < 7040
    del v1
    return v2
def method1(v0 : Mut0) -> bool:
    v1 = v0.v0
    del v0
    v2 = v1 < 60
    del v1
    return v2
def method4(v0 : f64, v1 : Mut3) -> bool:
    v2 = v1.v0
    del v1
    v3 = v2 < v0
    del v0, v2
    return v3
def method5(v0 : f64, v1 : Mut3) -> bool:
    v2 = v1.v0
    del v1
    v3 = v2 < v0
    del v0, v2
    return v3
def method6(v0 : cp.ndarray, v1 : cp.ndarray, v2 : f64, v3 : f64, v4 : f64, v5 : f64, v6 : f64, v7 : f64, v8 : f64, v9 : i32) -> None:
    v10 = math.sin(v2)
    v11 = v7 * v10
    v12 = math.sin(v3)
    v13 = v11 * v12
    v14 = math.cos(v4)
    v15 = v13 * v14
    v16 = math.cos(v2)
    del v2
    v17 = v8 * v16
    v18 = v17 * v12
    v19 = v18 * v14
    v20 = v15 - v19
    del v15, v19
    v21 = v7 * v16
    del v7, v16
    v22 = math.sin(v4)
    del v4
    v23 = v21 * v22
    v24 = v20 + v23
    del v20, v23
    v25 = v8 * v10
    del v8, v10
    v26 = v25 * v22
    v27 = v24 + v26
    del v24, v26
    v28 = math.cos(v3)
    del v3
    v29 = v6 * v28
    v30 = v29 * v14
    v31 = v27 + v30
    del v27, v30
    v32 = v21 * v14
    del v21
    v33 = v25 * v14
    del v14, v25
    v34 = v32 + v33
    del v32, v33
    v35 = v13 * v22
    del v13
    v36 = v34 - v35
    del v34, v35
    v37 = v18 * v22
    del v18
    v38 = v36 + v37
    del v36, v37
    v39 = v29 * v22
    del v22, v29
    v40 = v38 - v39
    del v38, v39
    v41 = v17 * v28
    del v17
    v42 = v11 * v28
    del v11, v28
    v43 = v41 - v42
    del v41, v42
    v44 = v6 * v12
    del v6, v12
    v45 = v43 + v44
    del v43, v44
    v46 = v45 + 100.0
    del v45
    v47 = 1.0 / v46
    del v46
    v48 = 80.0 + v5
    del v5
    v49 = 40.0 * v47
    v50 = v49 * v31
    del v31
    v51 = v50 * 2.0
    del v50
    v52 = v48 + v51
    del v48, v51
    v53 = i32(v52)
    del v52
    v54 = v49 * v40
    del v40, v49
    v55 = 22.0 + v54
    del v54
    v56 = i32(v55)
    del v55
    v57 = v56 * 160
    del v56
    v58 = v53 + v57
    del v53, v57
    v59 = v58 >= 0
    if v59:
        v60 = v58 < 7040
        v61 = v60
    else:
        v61 = False
    del v59
    if v61:
        del v61
        v62 = spiral_array_index(v0, v58)
        v63 = v47 > v62
        del v62
        if v63:
            del v63
            v0[v58] = v47
            del v0, v47
            v1[v58] = v9
            del v1, v9, v58
            return 
        else:
            del v0, v1, v9, v47, v58, v63
            return 
    else:
        del v0, v1, v9, v47, v58, v61
        return 
def method3(v0 : cp.ndarray, v1 : cp.ndarray, v2 : f64, v3 : f64, v4 : f64, v5 : f64, v6 : f64) -> None:
    v7 = -v5
    v8 = Mut3(v7)
    while method4(v5, v8):
        v10 = Mut3(v7)
        while method5(v5, v10):
            v12 = v8.v0
            v13 = v10.v0
            v14 = 59
            method6(v0, v1, v2, v3, v4, v6, v12, v13, v7, v14)
            del v14
            v15 = 92
            method6(v0, v1, v2, v3, v4, v6, v5, v13, v12, v15)
            del v15
            v16 = -v12
            v17 = 47
            method6(v0, v1, v2, v3, v4, v6, v7, v13, v16, v17)
            del v17
            v18 = 61
            method6(v0, v1, v2, v3, v4, v6, v16, v13, v5, v18)
            del v16, v18
            v19 = -v13
            v20 = 62
            method6(v0, v1, v2, v3, v4, v6, v12, v7, v19, v20)
            del v19, v20
            v21 = 60
            method6(v0, v1, v2, v3, v4, v6, v12, v5, v13, v21)
            del v12, v13, v21
            v22 = v10.v0
            v23 = v22 + 0.6
            del v22
            v10.v0 = v23
            del v23
        del v10
        v24 = v8.v0
        v25 = v24 + 0.6
        del v24
        v8.v0 = v25
        del v25
    del v0, v1, v2, v3, v4, v5, v6, v7, v8
    return 
def method7(v0 : Mut0) -> bool:
    v1 = v0.v0
    del v0
    v2 = v1 < 44
    del v1
    return v2
def method8(v0 : Mut0) -> bool:
    v1 = v0.v0
    del v0
    v2 = v1 < 160
    del v1
    return v2
def method2(v0 : cp.ndarray, v1 : cp.ndarray, v2 : f64, v3 : f64, v4 : f64, v5 : i32) -> i32:
    v6 = Mut0(0)
    while method0(v6):
        v8 = v6.v0
        v0[v8] = 0.0
        v1[v8] = 46
        v9 = v8 + 1
        del v8
        v6.v0 = v9
        del v9
    del v6
    v10 = 20.0
    v11 = -40.0
    method3(v0, v1, v2, v3, v4, v10, v11)
    del v10, v11
    v12 = 10.0
    v13 = 10.0
    method3(v0, v1, v2, v3, v4, v12, v13)
    del v12, v13
    v14 = 5.0
    v15 = 40.0
    method3(v0, v1, v2, v3, v4, v14, v15)
    del v0, v2, v3, v4, v14, v15
    print("u001b[H", end='')
    v16 = Mut2(v5)
    del v5
    v17 = Mut0(0)
    while method7(v17):
        v19 = v17.v0
        v20 = Mut0(0)
        while method8(v20):
            v22 = v20.v0
            v23 = v19 * 160
            v24 = v22 + v23
            del v23
            v25 = spiral_array_index(v1, v24)
            del v24
            v26 = v25 == 59
            if v26:
                v27 = ";"
                v44 = v27
            else:
                v28 = v25 == 92
                if v28:
                    del v28
                    v29 = "\\"
                    v44 = v29
                else:
                    del v28
                    v30 = v25 == 47
                    if v30:
                        del v30
                        v31 = "/"
                        v44 = v31
                    else:
                        del v30
                        v32 = v25 == 61
                        if v32:
                            del v32
                            v33 = "="
                            v44 = v33
                        else:
                            del v32
                            v34 = v25 == 62
                            if v34:
                                del v34
                                v35 = ">"
                                v44 = v35
                            else:
                                del v34
                                v36 = v25 == 60
                                if v36:
                                    del v36
                                    v37 = "<"
                                    v44 = v37
                                else:
                                    del v36
                                    v38 = "."
                                    v44 = v38
            del v26
            print(v44, end='')
            del v44
            v45 = v16.v0
            v46 = v45 * 31
            del v45
            v47 = v46 + v25
            del v25, v46
            v48 = v47 % 1000003
            del v47
            v16.v0 = v48
            del v48
            v49 = v22 + 1
            del v22
            v20.v0 = v49
            del v49
        del v20
        print("\n", end='')
        v50 = v19 + 1
        del v19
        v17.v0 = v50
        del v50
    del v1
    del v17
    v51 = v16.v0
    del v16
    return v51
def main():
    v0 = cp.empty(7040,dtype=cp.float64)
    v1 = Mut0(0)
    while method0(v1):
        v3 = v1.v0
        v0[v3] = 0.0
        v4 = v3 + 1
        del v3
        v1.v0 = v4
        del v4
    del v1
    v5 = cp.empty(7040,dtype=cp.int32)
    v6 = Mut0(0)
    while method0(v6):
        v8 = v6.v0
        v5[v8] = 46
        v9 = v8 + 1
        del v8
        v6.v0 = v9
        del v9
    del v6
    v10 = Mut1(0.0, 0.0, 0.0)
    v11 = Mut2(0)
    v12 = Mut0(0)
    while method1(v12):
        v14 = v12.v0
        v15, v16, v17 = v10.v0, v10.v1, v10.v2
        v18 = v11.v0
        v19 = method2(v0, v5, v15, v16, v17, v18)
        del v15, v16, v17, v18
        v11.v0 = v19
        del v19
        v20, v21, v22 = v10.v0, v10.v1, v10.v2
        v23 = v20 + 0.05
        del v20
        v24 = v21 + 0.05
        del v21
        v25 = v22 + 0.01
        del v22
        v10.v0 = v23
        v10.v1 = v24
        v10.v2 = v25
        del v23, v24, v25
        v26 = v14 + 1
        del v14
        v12.v0 = v26
        del v26
    del v0, v5, v10
    del v12
    v27 = v11.v0
    del v11
    print("cube: ", 60, " frames, checksum ", v27, "\n", sep='', end='')
    del v27
    return 0

if __name__ == '__main__': sys.exit(main())
