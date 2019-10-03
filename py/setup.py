from setuptools import setup, find_packages
from os import getenv


def build_native(spec):
    is_dev = getenv('DEVBUILD') is not None
    build_flags = []
    if not is_dev:
        build_flags.append('--release')

    target_path = '../target/release'
    if is_dev:
        target_path = '../target/debug'

    build = spec.add_external_build(
        cmd=['cargo', 'build'] + build_flags,
        path='../cabi'
    )

    spec.add_cffi_module(
        module_path='simple._native',
        dylib=lambda: build.find_dylib('simple', in_path=target_path),
        header_filename=lambda: build.find_header('simple.h', in_path='.'),
        rtld_flags=['NOW', 'NODELETE']
    )


setup(
    name='simple',
    version='0.1.0',
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    setup_requires=[
        'milksnake',
        'pytest-runner'
    ],
    install_requires=[
        'milksnake',
        'typing',
        'six>=1.12'
    ],
    milksnake_tasks=[
        build_native
    ],
    tests_require=[
        'pytest'
    ]
)
