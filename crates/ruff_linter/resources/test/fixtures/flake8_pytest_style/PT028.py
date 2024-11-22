import pytest
import warnings

def test_ok():
    with pytest.warns(UserWarning):
        [].size


async def test_ok_trivial_with():
    with pytest.warns(UserWarning):
        with context_manager_under_test():
            pass

    with pytest.warns(UserWarning):
        with context_manager_under_test():
            warnings.warn("foo")

    with pytest.warns(UserWarning):
        async with context_manager_under_test():
            pass


def test_ok_complex_single_call():
    with pytest.warns(UserWarning):
        my_func(
            [].size,
            [].size,
        )


def test_ok_func_and_class():
    with pytest.warns(UserWarning):
        class A:
            pass

    with pytest.warns(UserWarning):
        def f():
            pass


def test_error_multiple_statements():
    with pytest.warns(UserWarning):
        len([])
        [].size


async def test_error_complex_statement():
    with pytest.warns(UserWarning):
        if True:
            [].size

    with pytest.warns(UserWarning):
        for i in []:
            [].size

    with pytest.warns(UserWarning):
        async for i in []:
            [].size

    with pytest.warns(UserWarning):
        while True:
            [].size

    with pytest.warns(UserWarning):
        async with context_manager_under_test():
            if True:
                warnings.warn("foo")


def test_error_try():
    with pytest.warns(UserWarning):
        try:
            [].size
        except:
            warnings.warn("foo")
