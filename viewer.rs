//! The slapstick in 3D, assembled from slapstick_happy.svg and slapstick_sad.svg.
//!
//!     target/release/viewer [--out=slapstick-3d.html]
//!
//! The parts are the cut files' own outlines, read by the turn3d module in lasermade-tools.
//! This file is only where they go, as the README's "Building it" lays them out:
//! three happy paddles glued into the comedy slat, three sad ones into the
//! tragedy slat, four strips glued into a handle block over the engraved rectangle
//! on each slat's outer face, and a rope laced through the three holes at the far
//! end as the hinge. Each sheet's fifth strip is a spare. The tragedy slat is drawn a few degrees
//! open, the way it hangs before the swing closes it.
use lasermade_tools::py::{msin, radians};
use lasermade_tools::turn3d::*;

const HERE: &str = env!("CARGO_MANIFEST_DIR");
const TH: f64 = 3.0;

fn sheet(name: &str) -> (Vec<Part>, Vec<Part>) {
    let ps = parts(&format!("{HERE}/{name}"));
    let paddles: Vec<Part> = ps.iter().filter(|p| p.ink == "#000000").cloned().collect();
    let strips: Vec<Part> = ps.iter().filter(|p| p.ink == "#00ffff").cloned().collect();
    assert!(paddles.len() == 3 && strips.len() == 5, "{name} {} {}", paddles.len(), strips.len());
    (paddles, strips)
}

fn mean(h: &[Pt], k: usize) -> f64 {
    h.iter().fold(0.0, |s, q| s + q[k].f()) / h.len() as f64
}

fn build() -> Vec<(String, J)> {
    let open = radians(6.0);
    let (happy, strips) = sheet("slapstick_happy.svg");
    let (sad, strips2) = sheet("slapstick_sad.svg"); // four strips a handle; the fifth on each sheet is a spare
    let [x0, y0, x1, y1] = happy[0].bx.map(Num::f);
    let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    // the hinge: the three holes at the far end, measured off the paddle
    let far: Vec<&Vec<Pt>> = happy[0].h.iter().filter(|h| mean(h, 0) > cx).collect();
    let mut hx = far.iter().fold(0.0, |s, h| s + mean(h, 0)) / far.len() as f64;
    let mut holes: Vec<f64> = far.iter().map(|h| mean(h, 1) - cy).collect();
    holes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hx -= cx;
    let mut out = vec![];
    for (k, p) in happy.iter().enumerate() {
        // the comedy slat, face down and out
        out.push(placed(&centred(p, None), &T(0, 0, k as f64 * TH), [I(0), I(0), F(-0.6 + 0.25 * k as f64)], "ply"));
    }
    let hinge = chain(&[T(hx, 0, 3.0 * TH), Ry(open), T(-hx, 0, -3.0 * TH)]);
    for (k, p) in sad.iter().enumerate() {
        // the tragedy slat, face up and out
        out.push(placed(&centred(p, None), &chain(&[hinge.clone(), T(0, 0, (3 + k) as f64 * TH)]), [I(0), I(0), F(0.6 + 0.25 * k as f64)], "ply"));
    }
    // a handle on each slat, over the engraved rectangle on its outer face
    let rect = happy[0].e.iter().fold(&happy[0].e[0], |a, b| if b.len() < a.len() { b } else { a });
    let fold = |k: usize, f: fn(f64, f64) -> f64| rect[1..].iter().fold(rect[0][k].f(), |a, q| f(a, q[k].f()));
    let rx = (fold(0, lasermade_tools::py::min) + fold(0, lasermade_tools::py::max)) / 2.0 - cx;
    let ry = (fold(1, lasermade_tools::py::min) + fold(1, lasermade_tools::py::max)) / 2.0 - cy;
    for (k, s) in strips[..4].iter().enumerate() {
        out.push(placed(&centred(s, None), &T(rx, ry, -((k + 1) as f64) * TH), [I(0), I(0), F(-1.6 - 0.3 * k as f64)], "dark"));
    }
    for (k, s) in strips2[..4].iter().enumerate() {
        out.push(placed(&centred(s, None), &chain(&[hinge.clone(), T(rx, ry, (6 + k) as f64 * TH)]), [I(0), I(0), F(1.6 + 0.3 * k as f64)], "dark"));
    }
    // the rope: laced up and down through the three holes, and tied off
    let (zb, zt) = (I(-2), F(6.0 * TH + 2.0));
    let tip = |z: Num| hx + msin(open) * (z.f() - 3.0 * TH); // the open slat's holes lean with it
    let mut cord: Vec<[Num; 3]> = vec![];
    for (i, &y) in holes.iter().enumerate() {
        let (lo, hi) = if i % 2 == 0 { (zb, zt) } else { (zt, zb) };
        for z in [lo, (lo + hi).div(I(2)), hi] {
            let x = if z.f() <= 3.0 * TH { hx } else { tip(z) };
            cord.push([F(x).round(2), F(y).round(2), z]);
        }
        if i < holes.len() - 1 {
            let up = if F(0.0).lt(hi) { I(4) } else { I(-4) };
            cord.push([F(hx + 9.0).round(2), F((y + holes[i + 1]) / 2.0).round(2), hi + up]);
        }
    }
    if holes.len() % 2 == 1 {
        let mut c = vec![[F(hx + 14.0), F(holes[0] - 6.0), zb - I(3)]];
        c.extend(cord);
        c.push([F(hx + 14.0), F(holes[holes.len() - 1] + 6.0), zb - I(3)]);
        cord = c;
    }
    vec![(
        "comedy and tragedy".into(),
        design(vec![
            ("parts", J::A(out)),
            ("cords", J::A(vec![lasermade_tools::turn3d::cord(&cord)])),
            ("ex", J::N(I(22))),
            ("view", nums(&[F(-0.35), F(-0.38), F(2.6)])),
        ]),
    )]
}

fn main() {
    let path = out_arg(&format!("{HERE}/slapstick-3d.html"));
    write(&path, "Slapstick", &mut build(), "Drag to turn; right-drag, shift-drag or move two fingers to slide the view; scroll or pinch to zoom in on whatever is under the pointer; double-click a part to turn round it, and Reset view to go back. Draw Explode to take it apart. Every part is the cut files’ own outline at 3 mm: three happy paddles glued into the comedy slat underneath, three sad ones into the tragedy slat above, drawn a few degrees open, and a four-strip handle block, darker, on each slat’s outer face over its engraved rectangle. The red cord is the rope hinge laced through the three holes at the far end; its length and lacing are yours, and this is one way.", "slapstick/viewer.rs", false);
    println!("  wrote {}", relpath(&path));
}
