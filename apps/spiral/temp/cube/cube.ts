import { toString, Record, Union } from "../../../../deps/Fable/src/fable-library-ts/Types.js";
import { char_type, int32_type, record_type, float64_type, string_type, union_type, TypeInfo } from "../../../../deps/Fable/src/fable-library-ts/Reflection.js";
import { int32, float64 } from "../../../../deps/Fable/src/fable-library-ts/Int32.js";
import { IComparable, IEquatable } from "../../../../deps/Fable/src/fable-library-ts/Util.js";
import { FSharpList, ofArray } from "../../../../deps/Fable/src/fable-library-ts/List.js";
import { Option } from "../../../../deps/Fable/src/fable-library-ts/Option.js";
import { iterate, choose, collect, empty, singleton, append, enumerateWhile, delay } from "../../../../deps/Fable/src/fable-library-ts/Seq.js";
import { fill, setItem, item } from "../../../../deps/Fable/src/fable-library-ts/Array.js";
import { StringBuilder_$ctor_Z721C83C5, StringBuilder__AppendLine, StringBuilder__Append_Z721C83C5 } from "../../../../deps/Fable/src/fable-library-ts/System.Text.js";
import { Async, singleton as singleton_1 } from "../../../../deps/Fable/src/fable-library-ts/AsyncBuilder.js";
import { rangeDouble } from "../../../../deps/Fable/src/fable-library-ts/Range.js";
import { startChild, sleep } from "../../../../deps/Fable/src/fable-library-ts/Async.js";

export interface IOsEnviron {
    environ(): any
}

export type US0_$union = 
    | US0<0>
    | US0<1>
    | US0<2>

export type US0_$cases = {
    0: ["US0_0", []],
    1: ["US0_1", []],
    2: ["US0_2", []]
}

export function US0_US0_0() {
    return new US0<0>(0, []);
}

export function US0_US0_1() {
    return new US0<1>(1, []);
}

export function US0_US0_2() {
    return new US0<2>(2, []);
}

export class US0<Tag extends keyof US0_$cases> extends Union<Tag, US0_$cases[Tag][0]> {
    constructor(readonly tag: Tag, readonly fields: US0_$cases[Tag][1]) {
        super();
    }
    cases() {
        return ["US0_0", "US0_1", "US0_2"];
    }
}

export function US0_$reflection(): TypeInfo {
    return union_type("Cube.US0", [], US0, () => [[], [], []]);
}

export type US1_$union = 
    | US1<0>
    | US1<1>
    | US1<2>
    | US1<3>
    | US1<4>
    | US1<5>
    | US1<6>

export type US1_$cases = {
    0: ["US1_0", [US0_$union]],
    1: ["US1_1", [US0_$union]],
    2: ["US1_2", [US0_$union]],
    3: ["US1_3", [US0_$union]],
    4: ["US1_4", [US0_$union]],
    5: ["US1_5", [US0_$union]],
    6: ["US1_6", [US0_$union]]
}

export function US1_US1_0(f0_0: US0_$union) {
    return new US1<0>(0, [f0_0]);
}

export function US1_US1_1(f1_0: US0_$union) {
    return new US1<1>(1, [f1_0]);
}

export function US1_US1_2(f2_0: US0_$union) {
    return new US1<2>(2, [f2_0]);
}

export function US1_US1_3(f3_0: US0_$union) {
    return new US1<3>(3, [f3_0]);
}

export function US1_US1_4(f4_0: US0_$union) {
    return new US1<4>(4, [f4_0]);
}

export function US1_US1_5(f5_0: US0_$union) {
    return new US1<5>(5, [f5_0]);
}

export function US1_US1_6(f6_0: US0_$union) {
    return new US1<6>(6, [f6_0]);
}

export class US1<Tag extends keyof US1_$cases> extends Union<Tag, US1_$cases[Tag][0]> {
    constructor(readonly tag: Tag, readonly fields: US1_$cases[Tag][1]) {
        super();
    }
    cases() {
        return ["US1_0", "US1_1", "US1_2", "US1_3", "US1_4", "US1_5", "US1_6"];
    }
}

export function US1_$reflection(): TypeInfo {
    return union_type("Cube.US1", [], US1, () => [[["f0_0", US0_$reflection()]], [["f1_0", US0_$reflection()]], [["f2_0", US0_$reflection()]], [["f3_0", US0_$reflection()]], [["f4_0", US0_$reflection()]], [["f5_0", US0_$reflection()]], [["f6_0", US0_$reflection()]]]);
}

export type US2_$union = 
    | US2<0>
    | US2<1>

export type US2_$cases = {
    0: ["US2_0", [string]],
    1: ["US2_1", []]
}

export function US2_US2_0(f0_0: string) {
    return new US2<0>(0, [f0_0]);
}

export function US2_US2_1() {
    return new US2<1>(1, []);
}

export class US2<Tag extends keyof US2_$cases> extends Union<Tag, US2_$cases[Tag][0]> {
    constructor(readonly tag: Tag, readonly fields: US2_$cases[Tag][1]) {
        super();
    }
    cases() {
        return ["US2_0", "US2_1"];
    }
}

export function US2_$reflection(): TypeInfo {
    return union_type("Cube.US2", [], US2, () => [[["f0_0", string_type]], []]);
}

export class Mut0 extends Record implements IEquatable<Mut0>, IComparable<Mut0> {
    l0: float64;
    constructor(l0: float64) {
        super();
        this.l0 = l0;
    }
}

export function Mut0_$reflection(): TypeInfo {
    return record_type("Cube.Mut0", [], Mut0, () => [["l0", float64_type]]);
}

export type US3_$union = 
    | US3<0>
    | US3<1>

export type US3_$cases = {
    0: ["US3_0", [int32, float64, string]],
    1: ["US3_1", []]
}

export function US3_US3_0(f0_0: int32, f0_1: float64, f0_2: string) {
    return new US3<0>(0, [f0_0, f0_1, f0_2]);
}

export function US3_US3_1() {
    return new US3<1>(1, []);
}

export class US3<Tag extends keyof US3_$cases> extends Union<Tag, US3_$cases[Tag][0]> {
    constructor(readonly tag: Tag, readonly fields: US3_$cases[Tag][1]) {
        super();
    }
    cases() {
        return ["US3_0", "US3_1"];
    }
}

export function US3_$reflection(): TypeInfo {
    return union_type("Cube.US3", [], US3, () => [[["f0_0", int32_type], ["f0_1", float64_type], ["f0_2", char_type]], []]);
}

export type US4_$union = 
    | US4<0>
    | US4<1>

export type US4_$cases = {
    0: ["US4_0", [float64, string]],
    1: ["US4_1", []]
}

export function US4_US4_0(f0_0: float64, f0_1: string) {
    return new US4<0>(0, [f0_0, f0_1]);
}

export function US4_US4_1() {
    return new US4<1>(1, []);
}

export class US4<Tag extends keyof US4_$cases> extends Union<Tag, US4_$cases[Tag][0]> {
    constructor(readonly tag: Tag, readonly fields: US4_$cases[Tag][1]) {
        super();
    }
    cases() {
        return ["US4_0", "US4_1"];
    }
}

export function US4_$reflection(): TypeInfo {
    return union_type("Cube.US4", [], US4, () => [[["f0_0", float64_type], ["f0_1", char_type]], []]);
}

export function method0(): string {
    return "VSCODE_PID";
}

export function method2(v0_1: string): string {
    return v0_1;
}

export function method3(): string {
    return "";
}

export function closure1(unitVar: void, v0_1: string): US2_$union {
    return US2_US2_0(v0_1);
}

export function method4(): ((arg0: string) => US2_$union) {
    return (v: string): US2_$union => closure1(undefined, v);
}

export function method1(v0_1: string): string {
    return process.env[v0_1] ?? "";
}

export function method7(v0_1: float64, v1: float64, v2: float64): [float64, float64, float64] {
    return [v0_1, v1, v2] as [float64, float64, float64];
}

export function method9(v0_1: float64, v1: Mut0): boolean {
    return v1.l0 < v0_1;
}

export function method10(v0_1: Iterable<float64>): Iterable<float64> {
    return v0_1;
}

export function method12(v0_1: float64, v1: float64, v2: float64, v3: string, v4: float64, v5: float64, v6: float64, v7: float64): US3_$union {
    const v8: float64 = Math.sin(v4);
    const v9: float64 = v1 * v8;
    const v10: float64 = Math.sin(v5);
    const v11: float64 = v9 * v10;
    const v12: float64 = Math.cos(v6);
    const v13: float64 = v11 * v12;
    const v14: float64 = Math.cos(v4);
    const v15: float64 = v2 * v14;
    const v16: float64 = v15 * v10;
    const v18: float64 = v13 - (v16 * v12);
    const v19: float64 = v1 * v14;
    const v20: float64 = Math.sin(v6);
    const v23: float64 = v2 * v8;
    const v25: float64 = (v18 + (v19 * v20)) + (v23 * v20);
    const v26: float64 = Math.cos(v5);
    const v27: float64 = v0_1 * v26;
    const v45: float64 = 1 / ((((v15 * v26) - (v9 * v26)) + (v0_1 * v10)) + 100);
    const v53: float64 = 40 * v45;
    const v92: int32 = (~~(((160 / 2) + v7) + ((v53 * (v25 + (v27 * v12))) * 2)) + (~~((44 / 2) + (v53 * (((((v19 * v12) + (v23 * v12)) - (v11 * v20)) + (v16 * v20)) - (v27 * v20)))) * 160)) | 0;
    if ((v92 >= 0) && (v92 < 7040)) {
        return US3_US3_0(v92, v45, v3);
    }
    else {
        return US3_US3_1();
    }
}

export function closure4(v0_1: float64, v1: float64, v2: float64, v3: float64, v4: float64, v5: float64, v6: float64): FSharpList<US3_$union> {
    const v7: float64 = -v3;
    const v12: float64 = -v5;
    return ofArray([method12(v5, v6, v7, ";", v0_1, v1, v2, v4), method12(v3, v6, v5, "\\", v0_1, v1, v2, v4), method12(v7, v6, v12, "/", v0_1, v1, v2, v4), method12(v12, v6, v3, "=", v0_1, v1, v2, v4), method12(v5, v7, -v6, ">", v0_1, v1, v2, v4), method12(v5, v3, v6, "<", v0_1, v1, v2, v4)]);
}

export function closure3(v0_1: float64, v1: float64, v2: float64, v3: float64, v4: float64, v5: float64): ((arg0: float64) => FSharpList<US3_$union>) {
    return (v: float64): FSharpList<US3_$union> => closure4(v0_1, v1, v2, v3, v4, v5, v);
}

export function method11(v0_1: float64, v1: float64, v2: float64, v3: float64, v4: float64): ((arg0: float64) => ((arg0: float64) => FSharpList<US3_$union>)) {
    return (v: float64): ((arg0: float64) => FSharpList<US3_$union>) => closure3(v0_1, v1, v2, v3, v4, v);
}

export function closure5(unitVar: void, v0_1: US3_$union): Option<[int32, float64, string]> {
    if ((v0_1.tag as int32) === /* US3_0 */ 0) {
        return [v0_1.fields[0] as any, v0_1.fields[1] as any, v0_1.fields[2] as any] as [int32, float64, string];
    }
    else {
        return undefined;
    }
}

export function method13(): ((arg0: US3_$union) => Option<[int32, float64, string]>) {
    return (v: US3_$union): Option<[int32, float64, string]> => closure5(undefined, v);
}

export function closure2(v0_1: float64, v1: float64, v2: float64, _arg: [float64, float64]): Iterable<[int32, float64, string]> {
    const v3: float64 = _arg[0];
    const v5: float64 = -v3;
    const v151: Iterable<float64> = method10(delay<float64>((): Iterable<float64> => {
        const v77: Mut0 = new Mut0(v5);
        return enumerateWhile<float64>((): boolean => method9(v3, v77), delay<float64>((): Iterable<float64> => append<float64>(singleton<float64>(v77.l0), delay<float64>((): Iterable<float64> => {
            const v81: float64 = v77.l0 + 0.6;
            v77.l0 = v81;
            return empty<float64>();
        }))));
    }));
    const v152: ((arg0: float64) => ((arg0: float64) => FSharpList<US3_$union>)) = method11(v0_1, v1, v2, v3, _arg[1]);
    const v153: ((arg0: US3_$union) => Option<[int32, float64, string]>) = method13();
    return delay<[int32, float64, string]>((): Iterable<[int32, float64, string]> => collect<float64, Iterable<[int32, float64, string]>, [int32, float64, string]>((cube_x: float64): Iterable<[int32, float64, string]> => collect<float64, Iterable<[int32, float64, string]>, [int32, float64, string]>((cube_y: float64): Iterable<[int32, float64, string]> => choose<US3_$union, [int32, float64, string]>(v153, v152(cube_x)(cube_y)), v151), v151));
}

export function method8(v0_1: float64, v1: float64, v2: float64): ((arg0: [float64, float64]) => Iterable<[int32, float64, string]>) {
    return (arg10$0040: [float64, float64]): Iterable<[int32, float64, string]> => closure2(v0_1, v1, v2, arg10$0040);
}

export function closure7(v0_1: US4_$union[], v1: int32, _arg: [float64, string]): void {
    const v3: string = _arg[1];
    const v2: float64 = _arg[0];
    const v5: US4_$union = item(v1, v0_1);
    if ((v5.tag as int32) === /* US4_0 */ 0) {
        const v9 = v5.fields[1] as any;
        if ((v5.fields[0] as any) >= v2) {
        }
        else {
            setItem(v0_1, v1, US4_US4_0(v2, v3));
        }
    }
    else {
        setItem(v0_1, v1, US4_US4_0(v2, v3));
    }
}

export function closure6(v0_1: US4_$union[], v1: int32): ((arg0: [float64, string]) => void) {
    return (arg10$0040: [float64, string]): void => {
        closure7(v0_1, v1, arg10$0040);
    };
}

export function method14(): string {
    return "";
}

export function closure9(v0_1: US4_$union[], v1: any, v2: int32, v3: int32): void {
    let v10: float64;
    const v7: US4_$union = item(v3 + (v2 * 160), v0_1);
    const v21: any = StringBuilder__Append_Z721C83C5(v1, ((v7.tag as int32) === /* US4_0 */ 0) ? ((v10 = (v7.fields[0] as any), v7.fields[1] as any)) : ".");
}

export function closure8(v0_1: US4_$union[], v1: any, v2: int32): void {
    for (let col = 0; col <= (160 - 1); col++) {
        closure9(v0_1, v1, v2, col);
    }
    const v11: any = StringBuilder__AppendLine(v1);
}

export function closure10(v0_1: string, unitVar: void): void {
    console.log(v0_1);
}

export function method6(v0_1: int32, v1: int32, v2: float64, v3: float64, v4: float64): Async<void> {
    return singleton_1.Delay<void>((): Async<void> => {
        const patternInput: [float64, float64, float64] = method7(v2, v3, v4);
        const v1198: float64 = patternInput[2];
        const v1197: float64 = patternInput[1];
        const v1196: float64 = patternInput[0];
        const v1215: Iterable<[int32, float64, string]> = collect<[float64, float64], Iterable<[int32, float64, string]>, [int32, float64, string]>(method8(v1196, v1197, v1198), [[20, -40] as [float64, float64], [10, 10] as [float64, float64], [5, 40] as [float64, float64]]);
        const v1221: US4_$union[] = fill(new Array(160 * 44), 0, 160 * 44, US4_US4_1());
        iterate<[int32, float64, string]>((_arg: [int32, float64, string]): void => {
            closure6(v1221, _arg[0])([_arg[1], _arg[2]] as [float64, string]);
        }, v1215);
        const v1231: any = StringBuilder_$ctor_Z721C83C5(method14());
        return singleton_1.Combine<void>(singleton_1.For<int32>(rangeDouble(0, 1, 44 - 1), (_arg_1: int32): Async<void> => {
            closure8(v1221, v1231, _arg_1);
            return singleton_1.Zero();
        }), singleton_1.Delay<void>((): Async<void> => {
            const v1238: string = toString(v1231);
            const v1241: boolean = v0_1 < 0;
            return singleton_1.Combine<void>(v1241 ? ((process.stdout.write('\u001B[1;1H'), singleton_1.Zero())) : singleton_1.Zero(), singleton_1.Delay<void>((): Async<void> => {
                let v1251: any;
                closure10(v1238, undefined);
                v1251 = undefined;
                const v1265: Async<void> = sleep(1);
                return singleton_1.Bind<void, void>(v1265, (): Async<void> => {
                    let v1285: Async<void>;
                    const v1280: boolean = (v0_1 > 0) && (v1 >= v0_1);
                    return singleton_1.Combine<void>(v1280 ? (singleton_1.Zero()) : ((v1285 = method5(v0_1, v1 + 1, v1196 + 0.05, v1197 + 0.05, v1198 + 0.01), singleton_1.Combine<void>(singleton_1.ReturnFrom<void>(v1285), singleton_1.Delay<void>((): Async<void> => {
                        return singleton_1.Zero();
                    })))), singleton_1.Delay<void>((): Async<void> => {
                        return singleton_1.Zero();
                    }));
                });
            }));
        }));
    });
}

export function method5(v0_1: int32, v1: int32, v2: float64, v3: float64, v4: float64): Async<void> {
    return method6(v0_1, v1, v2, v3, v4);
}

export function closure0(unitVar: void, v0_1: string[]): void {
    const v39: string = method1(method0());
    let v65: boolean;
    if (0 === v39.length) {
        const v43: string = method1("AUTOMATION");
        v65 = (0 === v43.length);
    }
    else {
        v65 = false;
    }
    if (v65) {
        process.stdout.write('\u001B[?25l');
    }
    const v132: Async<Async<void>> = startChild<void>(method5(v65 ? -1 : 50, 1, 0, 0, 0));
}

export const v0 = (v: string[]): void => {
    closure0(undefined, v);
};

export const main_: ((arg0: string[]) => void) = v0;

main_([]);

