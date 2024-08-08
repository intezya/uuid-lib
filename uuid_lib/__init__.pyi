from typing import TypeAlias


Bytes: TypeAlias = bytes


class UUID:
    """
    A class representing a universally unique identifier (UUID).

    The UUID class provides a convenient way to work with UUIDs, offering
    various methods for converting to string representation and accessing
    the UUID in bytes form.
    """

    def __init__(self, bytes: Bytes) -> UUID:
        """
        Initialize a UUID object from a 16-byte string.

        This method creates a UUID object using a 16-byte sequence, which represents the binary form of a UUID.
        The input `bytes` must be exactly 16 bytes long.

        Args:
            bytes (bytes): A 16-byte string representing the UUID. The input must be exactly 16 bytes in length.

        Raises:
            ValueError: If the provided bytes are not exactly 16 bytes long.

        Examples:
            - UUID(bytes=b'\x12\x34\x56\x78\x9a\xbc\xde\xf0\x12\x34\x56\x78\x9a\xbc\xde\xf0'): Creates a UUID object using the provided 16 bytes.
            - UUID(uuid4().bytes)

        """
        ...

    def __str__(self) -> str:
        """
        Return the string representation of the UUID.

        This method returns the canonical 36-character string representation of the UUID,
        which includes 32 hexadecimal digits and four hyphens in the format 8-4-4-4-12.

        Returns:
            str: The string representation of the UUID.
        """
        ...

    def __repr__(self) -> str:
        """
        Return a formal string representation of the UUID.

        This method returns a string that includes the class name and the UUID string,
        suitable for debugging and logging.

        Returns:
            str: The formal string representation of the UUID, e.g., 'UUID("12345678-1234-5678-1234-567812345678")'.
        """
        ...

    @property
    def bytes(self) -> bytes:
        """
        Return the UUID as a 16-byte string.

        This property provides the UUID as a sequence of 16 bytes, which is the binary representation
        of the UUID.

        Returns:
            bytes: The 16-byte representation of the UUID.
        """
        ...


def uuid1() -> UUID:
    """
    Generate a UUID based on the host’s MAC address and the current time.

    UUID1 is generated using the current time and the MAC address of the host, ensuring that it is unique
    to the machine and the point in time it was generated. This is a Version 1 UUID.

    Returns:
        UUID: A UUID object representing a Version 1 UUID.
    """
    ...

def uuid2() -> UUID:
    """
    Generate a UUID based on a DCE Security version and the current time.

    UUID2 is similar to UUID1, but it includes a 16-bit POSIX UID or GID. This is a Version 2 UUID,
    specific to DCE Security.

    Returns:
        UUID: A UUID object representing a Version 2 UUID.
    """
    ...

def uuid3() -> UUID:
    """
    Generate a UUID based on the MD5 hash of a namespace identifier and a name.

    UUID3 uses the MD5 hashing algorithm to generate a UUID from a given namespace and name. This ensures that
    the same UUID is generated every time for the same namespace and name. This is a Version 3 UUID.

    Returns:
        UUID: A UUID object representing a Version 3 UUID.
    """
    ...

def uuid4() -> UUID:
    """
    Generate a random UUID.

    UUID4 is generated using random or pseudo-random numbers, making it completely unpredictable. This is a Version 4 UUID.

    Returns:
        UUID: A UUID object representing a Version 4 UUID.
    """
    ...

def uuid5() -> UUID:
    """
    Generate a UUID based on the SHA-1 hash of a namespace identifier and a name.

    UUID5 uses the SHA-1 hashing algorithm to generate a UUID from a given namespace and name. Like UUID3, it produces the
    same UUID for the same namespace and name, but it uses SHA-1 instead of MD5. This is a Version 5 UUID.

    Returns:
        UUID: A UUID object representing a Version 5 UUID.
    """
    ...

def uuid6() -> UUID:
    """
    Generate a UUID based on the time and clock sequence, reordered for improved database indexing.

    UUID6 is a reordered version of UUID1, designed to be more efficiently indexed in databases while maintaining
    temporal uniqueness. This is a Version 6 UUID.

    Returns:
        UUID: A UUID object representing a Version 6 UUID.
    """
    ...

def uuid7() -> UUID:
    """
    Generate a UUID based on the Unix timestamp.

    UUID7 is generated using the Unix timestamp, designed to be time-ordered and easier to index in databases.
    This is a Version 7 UUID.

    Returns:
        UUID: A UUID object representing a Version 7 UUID.
    """
    ...

def uuid8(bytes: Bytes) -> UUID:
    """
    Generate a UUID from a custom 16-byte string.

    UUID8 allows for the creation of a UUID using a custom 16-byte input, offering flexibility for specific use cases.
    This is a Version 8 UUID.

    Args:
        bytes (bytes): A 16-byte string to be used for generating the UUID.

    Returns:
        UUID: A UUID object representing a Version 8 UUID.
    """
    ...
