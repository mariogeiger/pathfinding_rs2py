# Pathfinding

Binding of [pathfinding](https://github.com/samueltardieu/pathfinding) for Python.

## Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install .
```

## Progress

- [x] `pathfinding::kuhn_munkres::kuhn_munkres_min`
- [ ] `pathfinding::kuhn_munkres::kuhn_munkres`
- [ ] `pathfinding::directed::*`
- [ ] all the rest...

## Testing

```bash
python -m pip install nox
python -m nox
```
