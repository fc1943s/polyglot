kernels_aux = r"""
// The types of these two will be replaced during compilation by the Spiral code generator. 
// It matches on `using default_int = ` and `;` with the inner part being replaced so the form should be kept as is. 
// The two statements need to begin at the start of a line.
using default_int = int;
using default_uint = unsigned int;

#ifndef __NVRTC__
// NVRTC has these includes by default so they need to be left out if it is used as the compiler.
#include <new>
#include <assert.h>
#include <stdio.h>
#endif

// For error checking on the host.
#define gpuErrchk(ans) { gpuAssert((ans), __FILE__, __LINE__); }
template <typename T> inline __device__ void destroy(T& obj) { obj.~T(); }
inline void gpuAssert(cudaError error, const char *file, int line, bool abort=true) {
    if (error != cudaSuccess) {
        fprintf(stderr, "GPUassert: %s %s %d\n", cudaGetErrorString(error), file, line);
        if (abort) exit(error);
    }
}

template <typename el>
struct sptr // Shared pointer for the Spiral datatypes. They have to have the refc field inside them to work.
{
    el* base;

    __device__ sptr() : base(nullptr) {}
    __device__ sptr(el* ptr) : base(ptr) { this->base->refc++; }

    __device__ ~sptr()
    {
        if (this->base != nullptr && --this->base->refc == 0)
        {
            delete this->base;
            this->base = nullptr;
        }
    }

    __device__ sptr(sptr& x)
    {
        this->base = x.base;
        this->base->refc++;
    }

    __device__ sptr(sptr&& x)
    {
        this->base = x.base;
        x.base = nullptr;
    }

    __device__ sptr& operator=(sptr& x)
    {
        if (this->base != x.base)
        {
            delete this->base;
            this->base = x.base;
            this->base->refc++;
        }
        return *this;
    }

    __device__ sptr& operator=(sptr&& x)
    {
        if (this->base != x.base)
        {
            delete this->base;
            this->base = x.base;
            x.base = nullptr;
        }
        return *this;
    }
};

template <typename el>
struct csptr : public sptr<el>
{ // Shared pointer for closures specifically.
    using sptr<el>::sptr;
    template <typename... Args>
    __device__ auto operator()(Args... args) -> decltype(this->base->operator()(args...))
    {
        return this->base->operator()(args...);
    }
};

template <typename el, default_int max_length>
struct static_array
{
    el ptr[max_length];
    __device__ el& operator[](default_int i) {
        assert("The index has to be in range." && 0 <= i && i < max_length);
        return this->ptr[i];
    }
};

template <typename el, default_int max_length>
struct static_array_list
{
    default_int length{ 0 };
    el ptr[max_length];

    __device__ el& operator[](default_int i) {
        assert("The index has to be in range." && 0 <= i && i < this->length);
        return this->ptr[i];
    }
    __device__ void push(el& x) {
        ptr[this->length++] = x;
        assert("The array after pushing should not be greater than max length." && this->length <= max_length);
    }
    __device__ void push(el&& x) {
        ptr[this->length++] = std::move(x);
        assert("The array after pushing should not be greater than max length." && this->length <= max_length);
    }
    __device__ el pop() {
        assert("The array before popping should be greater than 0." && 0 < this->length);
        auto x = ptr[--this->length];
        ptr[this->length].~el();
        new (&ptr[this->length]) el();
        return x;
    }
    // Should be used only during initialization.
    __device__ void unsafe_set_length(default_int i) {
        assert("The new length should be in range." && 0 <= i && i <= max_length);
        this->length = i;
    }
};

template <typename el, default_int max_length>
struct dynamic_array_base
{
    int refc{ 0 };
    el* ptr;

    __device__ dynamic_array_base() : ptr(new el[max_length]) {}
    __device__ ~dynamic_array_base() { delete[] this->ptr; }

    __device__ el& operator[](default_int i) {
        assert("The index has to be in range." && 0 <= i && i < this->length);
        return this->ptr[i];
    }
};

template <typename el, default_int max_length>
struct dynamic_array
{
    sptr<dynamic_array_base<el, max_length>> ptr;

    __device__ dynamic_array() = default;
    __device__ dynamic_array(bool t) : ptr(new dynamic_array_base<el, max_length>()) {}
    __device__ el& operator[](default_int i) {
        return this->ptr.base->operator[](i);
    }
};

template <typename el, default_int max_length>
struct dynamic_array_list_base
{
    int refc{ 0 };
    default_int length{ 0 };
    el* ptr;

    __device__ dynamic_array_list_base() : ptr(new el[max_length]) {}
    __device__ dynamic_array_list_base(default_int l) : ptr(new el[max_length]) { this->unsafe_set_length(l); }
    __device__ ~dynamic_array_list_base() { delete[] this->ptr; }

    __device__ el& operator[](default_int i) {
        assert("The index has to be in range." && 0 <= i && i < this->length);
        return this->ptr[i];
    }
    __device__ void push(el& x) {
        ptr[this->length++] = x;
        assert("The array after pushing should not be greater than max length." && this->length <= max_length);
    }
    __device__ void push(el&& x) {
        ptr[this->length++] = std::move(x);
        assert("The array after pushing should not be greater than max length." && this->length <= max_length);
    }
    __device__ el pop() {
        assert("The array before popping should be greater than 0." && 0 < this->length);
        auto x = ptr[--this->length];
        ptr[this->length].~el();
        new (&ptr[this->length]) el();
        return x;
    }
    // Should be used only during initialization.
    __device__ void unsafe_set_length(default_int i) {
        assert("The new length should be in range." && 0 <= i && i <= max_length);
        this->length = i;
    }
};

template <typename el, default_int max_length>
struct dynamic_array_list
{
    sptr<dynamic_array_list_base<el, max_length>> ptr;

    __device__ dynamic_array_list() = default;
    __device__ dynamic_array_list(default_int l) : ptr(new dynamic_array_list_base<el, max_length>(l)) {}

    __device__ el& operator[](default_int i) {
        return this->ptr.base->operator[](i);
    }
    __device__ void push(el& x) {
        this->ptr.base->push(x);
    }
    __device__ void push(el&& x) {
        this->ptr.base->push(std::move(x));
    }
    __device__ el pop() {
        return this->ptr.base->pop();
    }
    // Should be used only during initialization.
    __device__ void unsafe_set_length(default_int i) {
        this->ptr.base->unsafe_set_length(i);
    }
    __device__ default_int length_() {
        return this->ptr.base->length;
    }
};
"""
class static_array():
    def __init__(self, length):
        self.ptr = []
        for _ in range(length):
            self.ptr.append(None)

    def __getitem__(self, index):
        assert 0 <= index < len(self.ptr), "The get index needs to be in range."
        return self.ptr[index]
    
    def __setitem__(self, index, value):
        assert 0 <= index < len(self.ptr), "The set index needs to be in range."
        self.ptr[index] = value

class static_array_list(static_array):
    def __init__(self, length):
        super().__init__(length)
        self.length = 0

    def __getitem__(self, index):
        assert 0 <= index < self.length, "The get index needs to be in range."
        return self.ptr[index]
    
    def __setitem__(self, index, value):
        assert 0 <= index < self.length, "The set index needs to be in range."
        self.ptr[index] = value

    def push(self,value):
        assert (self.length < len(self.ptr)), "The length before pushing has to be less than the maximum length of the array."
        self.ptr[self.length] = value
        self.length += 1

    def pop(self):
        assert (0 < self.length), "The length before popping has to be greater than 0."
        self.length -= 1
        return self.ptr[self.length]

    def unsafe_set_length(self,i):
        assert 0 <= i <= len(self.ptr), "The new length has to be in range."
        self.length = i

class dynamic_array(static_array): 
    pass

class dynamic_array_list(static_array_list):
    def length_(self): return self.length


kernels_main = r"""
"""
from cube_auto import *
kernels = kernels_aux + kernels_main
import cupy as cp
import numpy as np
from dataclasses import dataclass
from typing import NamedTuple, Union, Callable, Tuple
i8 = int; i16 = int; i32 = int; i64 = int; u8 = int; u16 = int; u32 = int; u64 = int; f32 = float; f64 = float; char = str; string = str
cuda = False

import os
import math
import io
class CustomStringIO(io.StringIO):
    def __init__(self, init=''):
        super().__init__()
        if init != '': self.write(init)
    def __str__(self): return self.getvalue()
    def __repr__(self): return self.getvalue()
import sys
import asyncio

# fwd_dcls
# types
class US0_0(NamedTuple): # (0, Some)
    v0 : string
    tag = 0
class US0_1(NamedTuple): # (1, None)
    tag = 1
US0 = Union[US0_0, US0_1]
@dataclass
class Mut0:
    v0 : f64
def Closure1(env_v0 : f64, env_v1 : f64):
    def inner() -> None:
        nonlocal env_v0, env_v1
        v0 = env_v0; v1 = env_v1
        v2 = Mut0(v0)
        del v0
        while method5(v1, v2):
            v4 = v2.v0
            yield v4 
            del v4
            v5 = v2.v0
            v6 = v5 + 0.6
            del v5
            v2.v0 = v6
            del v6
        del v1, v2
        return 
    return inner
class US1_0(NamedTuple): # (0, Some)
    v0 : i32
    v1 : f64
    v2 : char
    tag = 0
class US1_1(NamedTuple): # (1, None)
    tag = 1
US1 = Union[US1_0, US1_1]
def Closure3(env_v0 : f64, env_v1 : f64, env_v2 : f64, env_v3 : f64, env_v4 : f64, env_v5 : f64):
    def inner(v6 : f64) -> list:
        nonlocal env_v0, env_v1, env_v2, env_v3, env_v4, env_v5
        v0 = env_v0; v1 = env_v1; v2 = env_v2; v3 = env_v3; v4 = env_v4; v5 = env_v5
        v7 = -v3
        v8 = ';'
        v9 = method8(v5, v6, v7, v8, v0, v1, v2, v4)
        del v8
        v10 = '\\'
        v11 = method8(v3, v6, v5, v10, v0, v1, v2, v4)
        del v10
        v12 = -v5
        v13 = '/'
        v14 = method8(v7, v6, v12, v13, v0, v1, v2, v4)
        del v13
        v15 = '='
        v16 = method8(v12, v6, v3, v15, v0, v1, v2, v4)
        del v12, v15
        v17 = -v6
        v18 = '>'
        v19 = method8(v5, v7, v17, v18, v0, v1, v2, v4)
        del v7, v17, v18
        v20 = '<'
        v21 = method8(v5, v3, v6, v20, v0, v1, v2, v4)
        del v0, v1, v2, v3, v4, v5, v20
        v22 = []
        v22.insert(0, v21)
        del v21
        v25 = v22 
        del v22
        v25.insert(0, v19)
        del v19
        v29 = v25 
        del v25
        v29.insert(0, v16)
        del v16
        v33 = v29 
        del v29
        v33.insert(0, v14)
        del v14
        v37 = v33 
        del v33
        v37.insert(0, v11)
        del v11
        v41 = v37 
        del v37
        v41.insert(0, v9)
        del v9
        v45 = v41 
        del v41
        return v45
    return inner
def Closure2(env_v0 : f64, env_v1 : f64, env_v2 : f64, env_v3 : f64, env_v4 : f64):
    def inner(v5 : f64) -> Callable[[f64], list]:
        nonlocal env_v0, env_v1, env_v2, env_v3, env_v4
        v0 = env_v0; v1 = env_v1; v2 = env_v2; v3 = env_v3; v4 = env_v4
        return Closure3(v0, v1, v2, v3, v4, v5)
    return inner
def Closure4():
    def inner(v0 : US1) -> Tuple[i32, f64, char]:
        match v0:
            case US1_1(): # None
                v10 = None
                return v10
            case US1_0(v1, v2, v3): # Some
                v6 = (v1, v2, v3) # some' 
                del v1, v2, v3
                return v6
            case t:
                raise Exception(f'Pattern matching miss. Got: {t}')
    return inner
def Closure5(env_v0 : list, env_v1 : Callable[[f64], Callable[[f64], list]], env_v2 : Callable[[US1], Tuple[i32, f64, char]]):
    def inner() -> None:
        nonlocal env_v0, env_v1, env_v2
        v0 = env_v0; v1 = env_v1; v2 = env_v2
        cube_range = v0 
        del v0
        get = v1 
        del v1
        box = v2 
        del v2
        for cube_x in cube_range:
            for cube_y in cube_range:
                x = get(cube_x)(cube_y)
                for i in x:
                    i_ = box(i)
                    if i_ is not None: yield i
        return 
    return inner
def Closure0(env_v0 : f64, env_v1 : f64, env_v2 : f64):
    def inner(v3 : f64, v4 : f64) -> list:
        nonlocal env_v0, env_v1, env_v2
        v0 = env_v0; v1 = env_v1; v2 = env_v2
        v5 = -v3
        v148 = Closure1(v5, v3)
        del v5
        v149 = list(v148())
        del v148
        v151 = method6(v149)
        del v149
        v152 = method7(v0, v1, v2, v3, v4)
        del v0, v1, v2
        v153 = method9()
        v161 = Closure5(v151, v152, v153)
        del v151, v152, v153
        v162 = list(v161())
        del v161
        return v162
    return inner
class US2_0(NamedTuple): # (0, Some)
    v0 : f64
    v1 : char
    tag = 0
class US2_1(NamedTuple): # (1, None)
    tag = 1
US2 = Union[US2_0, US2_1]
def Closure7(env_v0 : (cp if cuda else np).ndarray, env_v1 : i32):
    def inner(v2 : f64, v3 : char) -> None:
        nonlocal env_v0, env_v1
        v0 = env_v0; v1 = env_v1
        v6 = v0[v1]
        match v6:
            case US2_0(v8, _): # Some
                del v6
                v10 = v8 >= v2
                del v8
                if v10:
                    del v0, v1, v10
                    return 
                else:
                    del v10
                    v12 = US2_0(v2, v3)
                    v0[v1] = v12 
                    del v0, v1, v12
                    return 
            case t:
                del v6
                v14 = US2_0(v2, v3)
                v0[v1] = v14 
                del v0, v1, v14
                return 
    return inner
def Closure6(env_v0 : (cp if cuda else np).ndarray):
    def inner(v1 : i32) -> Callable[[f64, char], None]:
        nonlocal env_v0
        v0 = env_v0
        return Closure7(v0, v1)
    return inner
def Closure9(env_v0 : (cp if cuda else np).ndarray, env_v1 : io.StringIO, env_v2 : i32):
    def inner(v3 : i32) -> None:
        nonlocal env_v0, env_v1, env_v2
        v0 = env_v0; v1 = env_v1; v2 = env_v2
        v4 = v2 * 160
        del v2
        v5 = v3 + v4
        del v4
        v8 = v0[v5]
        del v0, v5
        match v8:
            case US2_1(): # None
                v13 = '.'
            case US2_0(_, v11): # Some
                v13 = v11
            case t:
                raise Exception(f'Pattern matching miss. Got: {t}')
        del v8
        v17 = str(v13)
        del v13
        v22 = v1.write(v17)
        del v1, v17, v22
        return 
    return inner
def Closure8(env_v0 : (cp if cuda else np).ndarray, env_v1 : io.StringIO):
    def inner(v2 : i32) -> None:
        nonlocal env_v0, env_v1
        v0 = env_v0; v1 = env_v1
        v4 = Closure9(v0, v1, v2)
        del v0
        for col in range(160): v4(col)
        del v4
        v17 = "\n"
        v18 = v1.write(v17)
        del v1, v17, v18
        return 
    return inner
# functions
def method0() -> string:
    v0 = "VSCODE_PID"
    return v0
def method1(v0 : string) -> string:
    v129 = os.environ
    v134 = v129.get(v0)
    del v0, v129
    v140 = v134 
    del v134
    v148 = None
    v152 = v140 == v148 
    del v148
    if v152:
        v156 = None
        v161 = v156
    else:
        v158 = v140 
        v159 = US0_0(v158)
        del v158
        v160 = v159 
        del v159
        v161 = v160
    del v140, v152
    v167 = US0_1()
    v168 = v161 or v167 
    del v161, v167
    match v168:
        case US0_1(): # None
            del v168
            v171 = ""
            return v171
        case US0_0(v170): # Some
            del v168
            return v170
        case t:
            raise Exception(f'Pattern matching miss. Got: {t}')
def method4(v0 : f64, v1 : f64, v2 : f64) -> Tuple[f64, f64, f64]:
    return v0, v1, v2
def method5(v0 : f64, v1 : Mut0) -> bool:
    v2 = v1.v0
    del v1
    v3 = v2 < v0
    del v0, v2
    return v3
def method6(v0 : list) -> list:
    return v0
def method8(v0 : f64, v1 : f64, v2 : f64, v3 : char, v4 : f64, v5 : f64, v6 : f64, v7 : f64) -> US1:
    v8 = math.sin(v4)
    v9 = v1 * v8
    v10 = math.sin(v5)
    v11 = v9 * v10
    v12 = math.cos(v6)
    v13 = v11 * v12
    v14 = math.cos(v4)
    del v4
    v15 = v2 * v14
    v16 = v15 * v10
    v17 = v16 * v12
    v18 = v13 - v17
    del v13, v17
    v19 = v1 * v14
    del v1, v14
    v20 = math.sin(v6)
    del v6
    v21 = v19 * v20
    v22 = v18 + v21
    del v18, v21
    v23 = v2 * v8
    del v2, v8
    v24 = v23 * v20
    v25 = v22 + v24
    del v22, v24
    v26 = math.cos(v5)
    del v5
    v27 = v0 * v26
    v28 = v27 * v12
    v29 = v25 + v28
    del v25, v28
    v30 = v19 * v12
    del v19
    v31 = v23 * v12
    del v12, v23
    v32 = v30 + v31
    del v30, v31
    v33 = v11 * v20
    del v11
    v34 = v32 - v33
    del v32, v33
    v35 = v16 * v20
    del v16
    v36 = v34 + v35
    del v34, v35
    v37 = v27 * v20
    del v20, v27
    v38 = v36 - v37
    del v36, v37
    v39 = v15 * v26
    del v15
    v40 = v9 * v26
    del v9, v26
    v41 = v39 - v40
    del v39, v40
    v42 = v0 * v10
    del v0, v10
    v43 = v41 + v42
    del v41, v42
    v44 = v43 + 100.0
    del v43
    v45 = 1.0 / v44
    del v44
    v49 = 160 
    v51 = v49 / 2.0
    del v49
    v52 = v51 + v7
    del v7, v51
    v53 = 40.0 * v45
    v54 = v53 * v29
    del v29
    v55 = v54 * 2.0
    del v54
    v56 = v52 + v55
    del v52, v55
    v67 = i32(v56)
    del v56
    v73 = 44 
    v75 = v73 / 2.0
    del v73
    v76 = v53 * v38
    del v38, v53
    v77 = v75 + v76
    del v75, v76
    v88 = i32(v77)
    del v77
    v91 = v88 * 160
    del v88
    v92 = v67 + v91
    del v67, v91
    v93 = v92 >= 0
    if v93:
        v94 = v92 < 7040
        v95 = v94
    else:
        v95 = False
    del v93
    if v95:
        del v95
        return US1_0(v92, v45, v3)
    else:
        del v3, v45, v92, v95
        return US1_1()
def method7(v0 : f64, v1 : f64, v2 : f64, v3 : f64, v4 : f64) -> Callable[[f64], Callable[[f64], list]]:
    return Closure2(v0, v1, v2, v3, v4)
def method9() -> Callable[[US1], Tuple[i32, f64, char]]:
    return Closure4()
def method10() -> string:
    v0 = ""
    return v0
def method3(v0 : i32, v1 : i32, v2 : f64, v3 : f64, v4 : f64) -> any:
    async def __new_async_unit__():
        v9943, v9944, v9945 = method4(v2, v3, v4)
        v9946 = []
        v9946.insert(0, (5.0, 40.0))
        v9949 = v9946 
        del v9946
        v9949.insert(0, (10.0, 10.0))
        v9953 = v9949 
        del v9949
        v9953.insert(0, (20.0, -40.0))
        v9957 = v9953 
        del v9953
        cubes = v9957 
        del v9957
        v9963 = Closure0(v9943, v9944, v9945)
        get_cube_points = v9963 
        del v9963
        v9964 = [x for cube in cubes for x in get_cube_points(*cube)]
        v9969 = US2_1()
        v9970 = [v9969 for _ in range(160 * 44)]
        del v9969
        v9974 = Closure6(v9970)
        for (idx, ooz, ch) in v9964: v9974(idx)(ooz, ch)
        del v9964, v9974
        v9975 = method10()
        v9979 = CustomStringIO(v9975)
        del v9975
        v9982 = Closure8(v9970, v9979)
        del v9970
        for row in range(44): v9982(row)
        del v9982
        v9986 = str(v9979)
        del v9979
        v9988 = v0 < 0
        if v9988:
            sys.stdout.write("\033[1;1H")
        else:
            pass
        del v9988
        print(v9986)
        del v9986
        v10020 = asyncio.sleep(1 / 1000)
        await v10020 
        del v10020
        v10025 = v0 > 0
        if v10025:
            v10026 = v1 >= v0
            v10027 = v10026
        else:
            v10027 = False
        del v10025
        if v10027:
            pass
        else:
            v10028 = v9943 + 0.05
            v10029 = v9944 + 0.05
            v10030 = v9945 + 0.01
            v10031 = v1 + 1
            v10032 = method2(v0, v10031, v10028, v10029, v10030)
            del v10028, v10029, v10030, v10031
            await v10032()
            del v10032
        del v9943, v9944, v9945, v10027
        """ new_async_unit
    del v0, v1, v2, v3, v4
    new_async_unit """
    v10033 = __new_async_unit__
    return v10033
def method2(v0 : i32, v1 : i32, v2 : f64, v3 : f64, v4 : f64) -> any:
    return method3(v0, v1, v2, v3, v4)
# main_defs
def main_body():
    v1 = (cp if cuda else np).array([],dtype=object)
    del v1
    v67 = method0()
    v68 = method1(v67)
    del v67
    v69 = len(v68)
    del v68
    v70 = 0 == v69
    del v69
    if v70:
        v71 = "AUTOMATION"
        v72 = method1(v71)
        del v71
        v73 = len(v72)
        del v72
        v74 = 0 == v73
        del v73
        v75 = v74
    else:
        v75 = False
    del v70
    if v75:
        v92 = -1
    else:
        v92 = 50
    del v75
    v93 = 1
    v94 = 0.0
    v95 = 0.0
    v96 = 0.0
    v97 = method2(v92, v93, v94, v95, v96)
    del v92, v93, v94, v95, v96
    asyncio.run(v97())
    del v97
    return 

def main():
    r = main_body()
    if cuda: cp.cuda.get_current_stream().synchronize() # This line is here so the `__trap()` calls on the kernel aren't missed.
    return r

if __name__ == '__main__': result = main(); None if result is None else print(result)
