#!/usr/bin/env python3
"""The slapstick in 3D, assembled from slapstick_happy.svg and slapstick_sad.svg.

    python3 viewer.py [--out=slapstick-3d.html]

The parts are the cut files' own outlines, read by lasermade-tools/turn3d.py.
This file is only where they go, as the README's "Building it" lays them out:
three happy paddles glued into the comedy slat, three sad ones into the
tragedy slat, four strips glued into a handle block over the engraved rectangle
on each slat's outer face, and a rope laced through the three holes at the far
end as the hinge. Each sheet's fifth strip is a spare. The tragedy slat is drawn a few degrees
open, the way it hangs before the swing closes it.
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', 'lasermade-tools'))
import turn3d as t   # noqa: E402

TH = 3.0
OPEN = math.radians(6)


def sheet(name):
    ps = t.parts(os.path.join(HERE, name))
    paddles = [p for p in ps if p['ink'] == '#000000']
    strips = [p for p in ps if p['ink'] == '#00ffff']
    assert len(paddles) == 3 and len(strips) == 5, (name, len(paddles), len(strips))
    return paddles, strips


def build():
    happy, strips = sheet('slapstick_happy.svg')
    sad, strips2 = sheet('slapstick_sad.svg')  # four strips a handle; the fifth on each sheet is a spare
    x0, y0, x1, y1 = happy[0]['box']
    cx, cy = (x0 + x1) / 2, (y0 + y1) / 2
    # the hinge: the three holes at the far end, measured off the paddle
    hx = sum(sum(q[0] for q in h) / len(h) for h in happy[0]['h'] if sum(q[0] for q in h) / len(h) > cx) \
        / sum(1 for h in happy[0]['h'] if sum(q[0] for q in h) / len(h) > cx)
    holes = sorted(sum(q[1] for q in h) / len(h) - cy for h in happy[0]['h'] if sum(q[0] for q in h) / len(h) > cx)
    hx -= cx
    out = []
    for k, p in enumerate(happy):                       # the comedy slat, face down and out
        out.append(t.placed(t.centred(p), t.T(0, 0, k * TH), (0, 0, -0.6 + 0.25 * k)))
    hinge = t.chain(t.T(hx, 0, 3 * TH), t.Ry(OPEN), t.T(-hx, 0, -3 * TH))
    for k, p in enumerate(sad):                         # the tragedy slat, face up and out
        out.append(t.placed(t.centred(p), t.chain(hinge, t.T(0, 0, (3 + k) * TH)), (0, 0, 0.6 + 0.25 * k)))
    # a handle on each slat, over the engraved rectangle on its outer face
    rect = min(happy[0]['e'], key=lambda e: len(e))
    rx = (min(q[0] for q in rect) + max(q[0] for q in rect)) / 2 - cx
    ry = (min(q[1] for q in rect) + max(q[1] for q in rect)) / 2 - cy
    for k, s in enumerate(strips[:4]):
        out.append(t.placed(t.centred(s), t.T(rx, ry, -(k + 1) * TH), (0, 0, -1.6 - 0.3 * k), 'dark'))
    for k, s in enumerate(strips2[:4]):
        out.append(t.placed(t.centred(s), t.chain(hinge, t.T(rx, ry, (6 + k) * TH)), (0, 0, 1.6 + 0.3 * k), 'dark'))
    # the rope: laced up and down through the three holes, and tied off
    zb, zt = -2, 6 * TH + 2
    tip = lambda z: (hx + math.sin(OPEN) * (z - 3 * TH), z)   # the open slat's holes lean with it
    cord = []
    for i, y in enumerate(holes):
        lo, hi = (zb, zt) if i % 2 == 0 else (zt, zb)
        for z in (lo, (lo + hi) / 2, hi):
            x = hx if z <= 3 * TH else tip(z)[0]
            cord.append((round(x, 2), round(y, 2), z))
        if i < len(holes) - 1:
            cord.append((round(hx + 9, 2), round((y + holes[i + 1]) / 2, 2), hi + (4 if hi > 0 else -4)))
    cord = [(hx + 14, holes[0] - 6, zb - 3)] + cord + [(hx + 14, holes[-1] + 6, zb - 3)] if len(holes) % 2 else cord
    return {'comedy and tragedy': {'parts': out, 'cords': [cord], 'ex': 22, 'view': [-0.35, -0.38, 2.6]}}


def main(argv):
    o = [a.split('=', 1)[1] for a in argv if a.startswith('--out=')]
    path = o[0] if o else os.path.join(HERE, 'slapstick-3d.html')
    t.write(path, 'Slapstick', build(),
            'Drag to turn; right-drag, shift-drag or move two fingers to slide the view; '
            'scroll or pinch to zoom in on whatever is under the pointer; double-click a part '
            'to turn round it, and Reset view to go back. Draw Explode to take it apart. Every part is the cut '
            'files’ own outline at 3mm: three happy paddles glued into the comedy slat underneath, '
            'three sad ones into the tragedy slat above, drawn a few degrees open, and a four-strip '
            'handle block, darker, on each slat’s outer face over its engraved rectangle. The '
            'red cord is the rope hinge laced through the three holes at the far end; its length and '
            'lacing are yours, and this is one way.', 'slapstick/viewer.py')
    print(f'  wrote {os.path.relpath(path)}')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
