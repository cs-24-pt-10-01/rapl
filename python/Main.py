from ctypes import *

class Rapl:
    def __init__(self, lib):
        self.__lib = lib

    @staticmethod
    def load():
        # TODO: Handling on different OS
        lib = cdll.LoadLibrary('./rapl_lib.dll')
        return Rapl(lib)

    def start(self):
        self.__lib.start_rapl()

    def stop(self):
        self.__lib.stop_rapl()

rapl = Rapl.load()
rapl.start()

# loop 10000 times and add a number
j = 0
for i in range(10000):
    j += 1

rapl.stop()

print(j)