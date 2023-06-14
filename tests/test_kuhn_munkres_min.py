from pathfinding import kuhn_munkres_min
import numpy as np


def test_kuhn_munkres_min():
    weights = np.array(
        [
            [0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 0.0],
        ]
    )
    cost, assignments = kuhn_munkres_min(weights)
    assert cost == 0.0
    assert np.all(assignments == np.array([0, 1, 2]))
