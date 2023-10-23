from setuptools import Extension, setup
from Cython.Build import cythonize

extensions = [
    Extension("pyru.example", ["pyru/example.pyx"],
              language="c",
              library_dirs=["../rust/pyru/target/release"],
              include_dirs=["../c"],
              libraries=["pyru"]
    )
]
setup(
    name="pyru",
    ext_modules=cythonize(extensions, language_level="3str", annotate=True)
)