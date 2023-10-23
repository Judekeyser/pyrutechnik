cdef extern from "example.h":
    cdef void * add_example(const char *c_s1, const char *c_s2, char ** c_sr)
    cdef void destroy_example(void * ptr)


cdef bytes __call(bytes b_1, bytes b_2):
    cdef char* c_1 = b_1
    cdef char* c_2 = b_2

    cdef bytes py_result
    cdef char* c_result
    cdef void* example_ptr = add_example(c_1,c_2, &c_result)

    try:
        py_result = c_result
    finally:
        destroy_example(example_ptr)
    return py_result


def call(a: str, b: str) -> str:
    cdef bytes b_1 = a.encode('utf-8')
    cdef bytes b_2 = b.encode('utf-8')
    cdef bytes result = __call(b_1, b_2)
    return bytes(result).decode('utf-8')
