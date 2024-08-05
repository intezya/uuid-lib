import uuid


class UUID:
    def __str__(self) -> uuid.UUID: ...
    def __repr__[object_info](self) -> object_info: ...
    def __init__(self, uuid: uuid.UUID) -> object: ...
    def __new__(cls, *args, **kwargs): ...

def uuid1() -> UUID: ...
def uuid2() -> UUID: ...
def uuid3() -> UUID: ...
def uuid4() -> UUID: ...
def uuid5() -> UUID: ...
def uuid6() -> UUID: ...
def uuid7() -> UUID: ...
def uuid8(bytes: bytes) -> UUID: ...
