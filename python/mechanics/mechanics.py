from __future__ import annotations
import logging
from typing import Callable

LOG = logging.getLogger(__name__)
LOG.setLevel(logging.INFO)
logging.basicConfig(
    level=logging.INFO,
    format="[%(levelname)s] %(message)s",
)

# Source https://neps.academy/exercise/2568
#
# Initial thoughts:
#
# T1) shortest car first
# T2) Rather than minimizing the waiting time, I was thinking about
#     maximizing the utilization of the mechanics.
#
# Observation: In Input sample #1 the last car can be assigned to any
# mechanic, as only the waiting time matters but not the total repair
# time. Hence it doesn't matter when the last car is finished.
#
# My proposed solution uses the following strategy:
#
# 1. Sort the cars by duration and queue the shortest car first.
# 2. In the initial batch assign the longest car to the fastest mechanic in
#    order to minimize the waiting time for follow-up cars.
# 3. When assigning the remaining cars, then always pick the mechanic with the
#    shortest waiting time.

class Mechanic:
    def __init__(self, factor: int):
        self.factor = factor
        self.cars: list[int] = []

    def queue(self, car: int) -> None:
        self.cars.append(car)

    def waiting_time(self, total: bool = False) -> int:
        """
        If optional arg total is True, then include all cars, otherwise
        exclude the last car, as there is no other car needing to wait for the
        last car.
        """
        cars = self.cars + [0] if total else self.cars
        # n-1 cars are waiting for the first car
        # n-2 cars are waiting for the 2nd car, etc.
        weighted = [i * car for i, car in enumerate(reversed(cars))]
        return self.factor * sum(weighted)


class Workshop:
    def __init__(self, factors: list[int]):
        self.mechanics = [Mechanic(f) for f in sorted(factors, reverse=True)]
        LOG.debug(f'mechanics (slowest first): {self.mechanics}')

    @property
    def _pick_mechanic(self) -> Mechanic:
        pivot = self.mechanics[0]
        for m in self.mechanics:
            if m.waiting_time(total=True) < pivot.waiting_time(total=True):
                pivot = m
        return pivot

    def distribute(self, cars: list[int]) -> Workshop:
        cars = sorted(cars)

        # Slowest mechanic picks shortest car.
        for m in self.mechanics:
            m.queue(cars.pop(0))
        LOG.debug(f'After initial queuing: {self.mechanics}')

        # For each remaining car: pick the mechanic with shortest expected
        # waiting time.
        for car in cars:
            self._pick_mechanic.queue(car)
        LOG.debug(f'After final queuing: {self.mechanics}')
        return self

    @property
    def total(self) -> int:
        result = 0
        for m in self.mechanics:
            result += m.waiting_time()
        return result


    def report(self, printer: Callable[[str], None]):
        printer(f'Total waiting time: {self.total}\nMechanics:')
        for i, m in enumerate(self.mechanics):
            printer(f'{i+1}. Factor {m.factor}, cars: {m.cars}.')


def distribute(cars: list[int], factors: list[int]):
    print(f'\nDistributing {cars} to {factors}')
    w = Workshop(factors).distribute(cars).report(print)


if __name__ == "__main__":
    distribute([15, 5, 10, 20], [1, 2])
    distribute([1, 1, 1, 1], [3])
    distribute([15, 5, 10, 20], [3])
    distribute([5, 5, 5, 10, 10], [10, 1])
