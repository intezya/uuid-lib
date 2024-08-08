import uuid
from time import time
import uuid_lib


def test_uuid1():
    uuid = uuid_lib.uuid1()
    assert isinstance(uuid, uuid_lib.UUID)

test_uuid1()


def test_speed():
    start = time()
    for i in range(10000):
        uuid_lib.uuid1()
    print('uuid1 ok')

    for i in range(10000):
        uuid_lib.uuid2()
    print('uuid2 ok')

    for i in range(10000):
        uuid_lib.uuid3()
    print('uuid3 ok')

    for i in range(10000):
        uuid_lib.uuid4()
    print('uuid4 ok')

    for i in range(10000):
        uuid_lib.uuid5()
    print('uuid5 ok')

    for i in range(10000):
        uuid_lib.uuid6()
    print('uuid6 ok')

    for i in range(10000):
        uuid_lib.uuid7()
    print('uuid7 ok')
    end = time()

    print(end-start)

test_speed()
