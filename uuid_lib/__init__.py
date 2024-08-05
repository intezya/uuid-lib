from uuid import UUID

from uuid_lib._uuid_lib import (
    uuid1,
    uuid2,
    uuid3,
    uuid4,
    uuid5,
    uuid6,
    uuid7,
)

from uuid_lib import _uuid_lib


def uuid8(buf: UUID | bytes) -> str:
    if isinstance(buf, UUID):
        buf = buf.bytes

    return _uuid_lib.uuid8(buf)
