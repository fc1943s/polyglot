import { Union } from "../../deps/Fable/src/fable-library-ts/Types.js";
import { union_type, TypeInfo } from "../../deps/Fable/src/fable-library-ts/Reflection.js";
import { int32 } from "../../deps/Fable/src/fable-library-ts/Int32.js";
import { US0_$union, US0_US0_0, US0_US0_4, US0_US0_3, US0_US0_2, US0_US0_1 } from "../../../../../../../deps/spiral/lib/spiral/trace.fsx";
import { SpiralTrace_trace } from "../../../../../../../deps/spiral/lib/spiral/lib.fsx";

export const nl = "\n";

export const q = "\"";

export type TraceLevel_$union = 
    | TraceLevel<0>
    | TraceLevel<1>
    | TraceLevel<2>
    | TraceLevel<3>
    | TraceLevel<4>

export type TraceLevel_$cases = {
    0: ["Verbose", []],
    1: ["Debug", []],
    2: ["Info", []],
    3: ["Warning", []],
    4: ["Critical", []]
}

export function TraceLevel_Verbose() {
    return new TraceLevel<0>(0, []);
}

export function TraceLevel_Debug() {
    return new TraceLevel<1>(1, []);
}

export function TraceLevel_Info() {
    return new TraceLevel<2>(2, []);
}

export function TraceLevel_Warning() {
    return new TraceLevel<3>(3, []);
}

export function TraceLevel_Critical() {
    return new TraceLevel<4>(4, []);
}

export class TraceLevel<Tag extends keyof TraceLevel_$cases> extends Union<Tag, TraceLevel_$cases[Tag][0]> {
    constructor(readonly tag: Tag, readonly fields: TraceLevel_$cases[Tag][1]) {
        super();
    }
    cases() {
        return ["Verbose", "Debug", "Info", "Warning", "Critical"];
    }
}

export function TraceLevel_$reflection(): TypeInfo {
    return union_type("Polyglot.Common.TraceLevel", [], TraceLevel, () => [[], [], [], [], []]);
}

/**
 * ## trace
 */
export function to_trace_level(_arg: TraceLevel_$union): US0_$union {
    switch (_arg.tag) {
        case /* Debug */ 1:
            return US0_US0_1();
        case /* Info */ 2:
            return US0_US0_2();
        case /* Warning */ 3:
            return US0_US0_3();
        case /* Critical */ 4:
            return US0_US0_4();
        default:
            return US0_US0_0();
    }
}

export function trace(level: TraceLevel_$union, fn: (() => string), locals: (() => string)): void {
    SpiralTrace_trace(to_trace_level(level))(fn)(locals);
}

