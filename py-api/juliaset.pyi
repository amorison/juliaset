from typing import Tuple
from numpy.typing import NDArray
import numpy as np


class ComplexRegion:
    def __init__(self, xleft: float, xright: float, yleft: float, yright: float): ...

    def build(self, resolution: int) -> NDArray[np.complex128]: ...


class JuliaDiv:
    def __init__(
        self,
        c_0: complex,
        threshold: float,
        n_iterations: Tuple[int, int],
        resolution: int
    ): ...

    def over(self, region: ComplexRegion) -> NDArray[np.float64]: ...
