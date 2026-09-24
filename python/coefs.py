def b1(x: list[float], y: list[float]) -> float:
    n  = len(x)
    xy = sum(xi * yi for xi, yi in zip(x, y))
    xs = sum(x)
    ys = sum(y)
    x2 = sum(xi**2 for xi in x)
    return ((n * xy - xs * ys) / ((n * x2) - sum(x)**2))

def b0(x: list[float], y: list[float]) -> float:
    ys  = sum(y)
    xs  = sum(x)
    n   = len(x)
    fb1 = b1(x,y)
    return (ys - fb1 * xs) / n