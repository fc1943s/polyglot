from __future__ import annotations
from collections.abc import Callable
from typing import Any
from .....deps.spiral.lib.spiral.lib import SpiralTrace_trace
from .....deps.spiral.lib.spiral.trace import US0
from fable_modules.fable_library.reflection import (TypeInfo, union_type)
from fable_modules.fable_library.types import (Array, Union)

nl: str = "\n"

q: str = "\""

def _expr193() -> TypeInfo:
    return union_type("Polyglot.Common.TraceLevel", [], TraceLevel, lambda: [[], [], [], [], []])


class TraceLevel(Union):
    def __init__(self, tag: int, *fields: Any) -> None:
        super().__init__()
        self.tag: int = tag or 0
        self.fields: Array[Any] = list(fields)

    @staticmethod
    def cases() -> list[str]:
        return ["Verbose", "Debug", "Info", "Warning", "Critical"]


TraceLevel_reflection = _expr193

def to_trace_level(_arg: TraceLevel) -> US0:
    """## trace
    """
    if _arg.tag == 1:
        return US0(1)

    elif _arg.tag == 2:
        return US0(2)

    elif _arg.tag == 3:
        return US0(3)

    elif _arg.tag == 4:
        return US0(4)

    else: 
        return US0(0)



def trace(level: TraceLevel, fn: Callable[[], str], locals: Callable[[], str]) -> None:
    SpiralTrace_trace(to_trace_level(level))(fn)(locals)


