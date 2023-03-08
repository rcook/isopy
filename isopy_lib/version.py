from collections import namedtuple
from functools import total_ordering


@total_ordering
class Version(namedtuple("Version", ["major", "minor", "build"])):
    @staticmethod
    def parse(s):
        major, minor, build = [int(x) for x in s.split(".", 3)]
        return Version(major=major, minor=minor, build=build)

    def __eq__(self, other):
        if not isinstance(other, Version):
            return NotImplemented

        return self.major == other.major \
            and self.minor == other.minor \
            and self.build == other.build

    def __lt__(self, other):
        if not isinstance(other, Version):
            return NotImplemented

        if self.major != other.major:
            return self.major < other.major

        if self.minor != other.minor:
            return self.minor < other.minor

        if self.build != other.build:
            return self.build < other.build

        return False

    def __str__(self):
        return f"{self.major}.{self.minor}.{self.build}"
