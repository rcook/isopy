from abc import ABC, abstractmethod


class ArgHelper(ABC):
    @abstractmethod
    def dir_path_type(self, s): raise NotImplementedError()

    @abstractmethod
    def file_path_type(self, s): raise NotImplementedError()

    @abstractmethod
    def add_common_args(self, parser): raise NotImplementedError()
