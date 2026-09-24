def b1(x: list[float], y: list[float]) -> float:
    n  = len(x)
    xy = sum(xi * yi for xi, yi in zip(x, y))
    xs = sum(x)
    ys = sum(y)
    x2 = sum(xi**2 for xi in x)
    return ((n * xy - xs * ys) / ((n * x2) - sum(x)*2)), n, ys, xs

def b0(x: list[float], y: list[float]) -> float:
    fb1, n, ys, xs = b1(x,y)
    return (ys - fb1 * xs) / n