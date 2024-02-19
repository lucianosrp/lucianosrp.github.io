import random


def foo() -> str | None:
    if random.randint(0, 10) == 1:
        return "Hello"


def bar(s: str | None) -> str:
    if isinstance(s, str):
        return s.upper()
    return ""


random_val = foo()
assert bar(random_val) in ("HELLO", "")
