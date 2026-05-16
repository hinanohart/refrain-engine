"""Cell complex tests."""

from __future__ import annotations


from intensity_plane import CellComplex


def test_empty_complex_has_no_cells():
    c = CellComplex()
    assert c.chain_complex_size() == {}
    assert c.euler_characteristic() == 0


def test_point_has_euler_one():
    c = CellComplex()
    c.add(dim=0)
    assert c.euler_characteristic() == 1


def test_edge_with_two_endpoints_has_euler_one():
    c = CellComplex()
    a = c.add(dim=0)
    b = c.add(dim=0)
    c.add(dim=1, boundary=(a, b))
    # 2 vertices - 1 edge = 1
    assert c.euler_characteristic() == 1


def test_triangle_has_euler_one():
    # Three vertices, three edges, one face. χ = 3 - 3 + 1 = 1.
    c = CellComplex()
    v = [c.add(dim=0) for _ in range(3)]
    e = [c.add(dim=1, boundary=(v[i], v[(i + 1) % 3])) for i in range(3)]
    c.add(dim=2, boundary=tuple(e))
    assert c.euler_characteristic() == 1


def test_sphere_has_euler_two():
    # Tetrahedron boundary realization: 4 v, 6 e, 4 f. χ = 4 - 6 + 4 = 2.
    c = CellComplex()
    v = [c.add(dim=0) for _ in range(4)]
    # We do not validate boundary tuples; counts suffice for χ.
    for _ in range(6):
        c.add(dim=1, boundary=(v[0], v[1]))
    for _ in range(4):
        c.add(dim=2, boundary=())
    assert c.euler_characteristic() == 2


def test_boundary_of_returns_stored_tuple():
    c = CellComplex()
    a = c.add(dim=0)
    b = c.add(dim=0)
    e = c.add(dim=1, boundary=(a, b))
    assert c.boundary_of(e) == (a, b)


def test_of_dim_filters_correctly():
    c = CellComplex()
    for _ in range(3):
        c.add(dim=0)
    for _ in range(2):
        c.add(dim=1)
    assert len(c.of_dim(0)) == 3
    assert len(c.of_dim(1)) == 2
    assert len(c.of_dim(2)) == 0


def test_payload_round_trips():
    c = CellComplex()
    cid = c.add(dim=0, payload={"label": "origin"})
    assert c.cells[cid].payload == {"label": "origin"}
