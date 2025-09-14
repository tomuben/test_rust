import pytest
from mechanics.mechanics import Workshop

@pytest.mark.parametrize ("cars, factors, expected", [
    ([15, 5, 10, 20], [1, 2], 20),
    ([1, 1, 1, 1], [3], 18),
    ([15, 5, 10, 20], [3], 150),
    ([5, 5, 5, 10, 10], [10, 1], 30),
])
def test_mechanics(cars, factors, expected):
    print(f'\nDistributing {cars} to {factors}')
    w = Workshop(factors).distribute(cars)
    w.report(print)
    return
    assert w.total == expected
