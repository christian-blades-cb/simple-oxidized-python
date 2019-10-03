from simple._native import lib, ffi
import six


def hello_rust():
    res = lib.hello_world()
    text = ffi.string(res)
    print(text)
    lib.free_string(res)
