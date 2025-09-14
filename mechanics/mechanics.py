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
# T1) shortest job first
# T2) Rather than minimizing the waiting time, I was thinking about
#     maximizing the utilization of the mechanics.
#
# Observation: In Input sample #1 the last job can be assigned to any
# mechanic, as only the waiting time matters but not the total repair
# time. Hence it doesn't matter when the last job is finished.
#
# My proposed solution uses the following strategy:
#
# 1. Sort the jobs by duration and queue the shortest job first.
# 2. In the initial batch assign the longest job to the fastest mechanic in
#    order to minimize the waiting time for follow-up jobs.
# 3. When assigning the remaining jobs, then always pick the mechanic with the
#    shortest waiting time.

class Mechanic:
    def __init__(self, factor: int):
        self.factor = factor
        self.jobs: list[int] = []

    def queue(self, job: int) -> None:
        self.jobs.append(job)

    def waiting_time(self, total: bool = False) -> int:
        """
        If optional arg total is True, then include all jobs, otherwise
        exclude the last job, as there is no other job needing to wait for the
        last job.
        """
        jobs = self.jobs + [0] if total else self.jobs
        # n-1 jobs are waiting for the first job
        # n-2 jobs are waiting for the 2nd job, etc.
        weighted = [i * job for i, job in enumerate(reversed(jobs))]
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

    def distribute(self, jobs: list[int]) -> Workshop:
        jobs = sorted(jobs)

        # Slowest mechanic picks shortest job.
        for m in self.mechanics:
            m.queue(jobs.pop(0))
        LOG.debug(f'After initial queuing: {self.mechanics}')

        # For each remaining job: pick the mechanic with shortest expected
        # waiting time.
        for job in jobs:
            self._pick_mechanic.queue(job)
        LOG.debug(f'After final queuing: {self.mechanics}')
        return self

    def report(self, printer: Callable[[str], None]):
        total = 0
        for m in self.mechanics:
            total += m.waiting_time()

        printer(f'Total waiting time: {total}\nMechanics:')
        for i, m in enumerate(self.mechanics):
            printer(f'{i+1}. Factor {m.factor}, jobs: {m.jobs}.')


def distribute(jobs: list[int], factors: list[int]):
    print(f'\nDistributing {jobs} to {factors}')
    w = Workshop(factors).distribute(jobs).report(print)


if __name__ == "__main__":
    distribute([20, 5, 10, 15], [1, 2])
    distribute([1, 1, 1, 1], [3])
    distribute([15, 5, 10, 20], [3])
