//! The slapstick's two cut sheets and its SolidCraft design, from a few
//! numbers (2026-11-08; drawn by hand in Inkscape until then).
//!
//!     target/release/slapstick [--length=MM ...] [--out=DIR] [--solidcraft=DIR] [--no-write]
//!
//! Each sheet is one side: three paddles, each a rectangle with a face on it,
//! and five handle strips. The face is an engraved circle with two eyes and a
//! mouth cut through: a band between two arcs about one centre, a smile on
//! the comedy sheet (slapstick_happy.svg) and the same band turned round in
//! its own box, a frown, on the tragedy sheet (slapstick_sad.svg). Three rope
//! holes across the far end are the hinge, and an engraved rectangle marks
//! where the handle block goes. The parts are laid edge to edge, as the drawn
//! sheets were: neighbouring paddles share a cut line, and so do the strips
//! and the last paddle.
//!
//! What the hand drawing had that this does not: paddles 479.71 by 89.71
//! (Inkscape's size for a 480 by 90 rectangle less its stroke), two hinge
//! holes per paddle drawn 4.94 across and 0.12 mm in from the third, and
//! every part 0.008 mm in from the sheet's corner. Here they are 480 by 90,
//! three 5 mm holes on one line, from the corner.
//!
//! Every number below is a flag (--handle-x=280) and a parameter of the same
//! name in the SolidCraft design that --solidcraft=DIR writes: the six
//! paddles laminated into two slats, closed, and a four-strip handle block on
//! each slat's outer face. Engraving has no depth and is left out of it.
use lasermade_tools::flat::{outs, write, Checks, Params, Sheet, Unit::*};
use lasermade_tools::py::{mcos as cos, msin as sin, radians};
use lasermade_tools::solidcraft::Script;

const HERE: &str = env!("CARGO_MANIFEST_DIR");
const PADDLES: usize = 3; // per slat, and per sheet
const STRIPS: usize = 5; // per sheet: a handle block of four and a spare
const BLOCK: usize = 4; // strips glued into a handle
const BED: (f64, f64) = (600.0, 305.0);
const DENSITY: f64 = 0.68; // Baltic birch, g/cm3

fn table() -> Params {
    Params::new(&[
        ("ply", 3.0, Len, "the ply: every part"),
        ("length", 480.0, Len, "a paddle, hinge end to tip"),
        ("width", 90.0, Len, "a paddle, across"),
        ("face_x", 56.75, Len, "the face's centre from the tip end"),
        ("face_d", 50.0, Len, "the engraved face circle"),
        ("eye_d", 6.0, Len, "an eye"),
        ("eye_dx", 7.0, Len, "the eyes from the face's centre, toward the hinge"),
        ("eye_dy", 9.0, Len, "each eye from the centreline"),
        ("mouth_ro", 15.0, Len, "the mouth's outer arc, radius"),
        ("mouth_ri", 11.0, Len, "the mouth's inner arc, radius"),
        ("mouth_span", 120.0, Ang, "the angle the mouth's arcs span"),
        ("mouth_dx", 2.5, Len, "the smile's arc centre from the face's centre, toward the hinge"),
        ("hinge_d", 5.0, Len, "a rope hole"),
        ("hinge_inset", 12.5, Len, "the rope holes from the hinge end"),
        ("hinge_pitch", 22.5, Len, "rope hole to rope hole"),
        ("handle_x", 294.5, Len, "the handle's near end from the tip end"),
        ("handle_len", 90.0, Len, "the handle and each strip, long"),
        ("handle_w", 25.0, Len, "the handle and each strip, across"),
    ])
}

type P = (f64, f64);

/// The mouth's arc centre (x from the tip end) and the angle its arcs are
/// centred on: the smile opens toward the hinge, the frown is it turned
/// round in its own box.
fn mouth(p: &Params, happy: bool) -> (f64, f64) {
    let ch = p.get("face_x") + p.get("mouth_dx");
    if happy {
        (ch, 180.0)
    } else {
        (ch - p.get("mouth_ro") - p.get("mouth_ri") * cos(radians(p.get("mouth_span") / 2.0)), 0.0)
    }
}

/// The mouth as an SVG path, about a centre line at y.
fn mouth_d(p: &Params, happy: bool, y: f64) -> String {
    let (cx, mid) = mouth(p, happy);
    let (ro, ri, h) = (p.get("mouth_ro"), p.get("mouth_ri"), p.get("mouth_span") / 2.0);
    let at = |r: f64, a: f64| -> P { (cx + r * cos(radians(a)), y + r * sin(radians(a))) };
    let f = lasermade_tools::flat::f;
    let (a, b) = (mid - h, mid + h);
    let (o0, o1, i1, i0) = (at(ro, a), at(ro, b), at(ri, b), at(ri, a));
    // in the sheet's y-down frame an increasing angle is clockwise: sweep 1
    format!(
        "M {},{} A {},{} 0 0 1 {},{} L {},{} A {},{} 0 0 0 {},{} Z",
        f(o0.0),
        f(o0.1),
        f(ro),
        f(ro),
        f(o1.0),
        f(o1.1),
        f(i1.0),
        f(i1.1),
        f(ri),
        f(ri),
        f(i0.0),
        f(i0.1)
    )
}

fn sheet(p: &Params, happy: bool) -> Sheet {
    let (l, w) = (p.get("length"), p.get("width"));
    let (hl, hw) = (p.get("handle_len"), p.get("handle_w"));
    let mut s = Sheet::new(l.max(STRIPS as f64 * hl), PADDLES as f64 * w + hw);
    for k in 0..PADDLES {
        let y0 = k as f64 * w;
        let c = y0 + w / 2.0;
        s.circle("#0000ff", p.get("face_x"), c, p.get("face_d"));
        s.rect("#0000ff", p.get("handle_x"), c - hw / 2.0, hl, hw);
        for e in [-1.0, 1.0] {
            s.circle("#00ff00", p.get("face_x") + p.get("eye_dx"), c + e * p.get("eye_dy"), p.get("eye_d"));
        }
        s.path("#00ff00", mouth_d(p, happy, c));
        for j in [-1.0, 0.0, 1.0] {
            s.circle("#ff8000", l - p.get("hinge_inset"), c + j * p.get("hinge_pitch"), p.get("hinge_d"));
        }
        s.rect("#000000", 0.0, y0, l, w);
    }
    for j in 0..STRIPS {
        s.rect("#00ffff", j as f64 * hl, PADDLES as f64 * w, hl, hw);
    }
    s
}

/// A paddle's area less its eyes, mouth and rope holes.
fn paddle_area(p: &Params) -> f64 {
    let pi = std::f64::consts::PI;
    let disc = |d: f64| pi * d * d / 4.0;
    let mouth = (p.get("mouth_ro").powi(2) - p.get("mouth_ri").powi(2)) * radians(p.get("mouth_span")) / 2.0;
    p.get("length") * p.get("width") - 2.0 * disc(p.get("eye_d")) - mouth - 3.0 * disc(p.get("hinge_d"))
}

// --- the SolidCraft design --------------------------------------------------------

fn design(p: &Params) -> (String, Vec<(String, f64)>) {
    let mut s = Script::new();
    p.declare(&mut s);
    let mut vols = vec![];
    let ply = p.get("ply");
    let pa = paddle_area(p) * ply;
    let strip = p.get("handle_len") * p.get("handle_w") * ply;
    // the comedy slat on the bench, face down; the tragedy slat on it, face up
    for (slat, happy, z0) in [("comedy", true, 0usize), ("tragedy", false, PADDLES)] {
        for k in 0..PADDLES {
            let name = format!("{slat} paddle {}", k + 1);
            let sk = s.sketch("XY", Some(&format!("{}*ply", z0 + k)), &format!("{name} sketch"));
            s.cmd("sketch.rectangle.two_point", r#"{"p0":["0 mm","0 mm"],"p1":["length","width"]}"#.into());
            for e in ["+", "-"] {
                s.cmd("sketch.circle.center", format!(r#"{{"center":["face_x + eye_dx","width/2 {e} eye_dy"],"diameter":"eye_d"}}"#));
            }
            // the mouth: outer arc, a radial end, the inner arc back, the other end
            let (cx, mid) = if happy { ("face_x + mouth_dx".to_string(), "180 deg") } else { ("face_x + mouth_dx - mouth_ro - mouth_ri*cos(mouth_span/2)".to_string(), "0 deg") };
            let pt = |r: &str, a: &str| format!(r#"["({cx}) + {r}*cos({mid} {a} mouth_span/2)","width/2 + {r}*sin({mid} {a} mouth_span/2)"]"#);
            let c = format!(r#"["{cx}","width/2"]"#);
            s.cmd("sketch.arc.center_point", format!(r#"{{"center":{c},"start":{},"end":{}}}"#, pt("mouth_ro", "-"), pt("mouth_ro", "+")));
            s.cmd("sketch.line", format!(r#"{{"points":[{},{}]}}"#, pt("mouth_ro", "+"), pt("mouth_ri", "+")));
            s.cmd("sketch.arc.center_point", format!(r#"{{"center":{c},"start":{},"end":{},"clockwise":true}}"#, pt("mouth_ri", "+"), pt("mouth_ri", "-")));
            s.cmd("sketch.line", format!(r#"{{"points":[{},{}]}}"#, pt("mouth_ri", "-"), pt("mouth_ro", "-")));
            for j in ["- hinge_pitch", "", "+ hinge_pitch"] {
                s.cmd("sketch.circle.center", format!(r#"{{"center":["length - hinge_inset","width/2 {j}"],"diameter":"hinge_d"}}"#));
            }
            s.finish();
            s.extrude(&sk, "ply", false, "new", &[], &name);
            vols.push((name, pa));
        }
    }
    // a handle block on each slat's outer face, over its engraved rectangle
    for (slat, below) in [("comedy", true), ("tragedy", false)] {
        for k in 0..BLOCK {
            let name = format!("{slat} handle strip {}", k + 1);
            let z = if below { format!("-{}*ply", k + 1) } else { format!("{}*ply", 2 * PADDLES + k) };
            let sk = s.sketch("XY", Some(&z), &format!("{name} sketch"));
            s.cmd("sketch.rectangle.two_point", r#"{"p0":["handle_x","width/2 - handle_w/2"],"p1":["handle_x + handle_len","width/2 + handle_w/2"]}"#.into());
            s.finish();
            s.extrude(&sk, "ply", false, "new", &[], &name);
            vols.push((name, strip));
        }
    }
    s.measure();
    (s.text(), vols)
}

fn run() -> Result<i32, String> {
    let defaults = table();
    let mut p = table();
    let rest = p.parse(std::env::args().skip(1).collect())?;
    let o = outs(rest, &p.flags())?;
    if let Some(dir) = &o.solidcraft {
        let (text, vols) = design(&p);
        write(&format!("{dir}/slapstick-comedy-and-tragedy.solidcraft.json"), &text)?;
        if std::env::var("SC_VOLUMES").is_ok() {
            for (n, v) in vols {
                eprintln!("{n}\t{v:.4}");
            }
        }
        return Ok(0);
    }
    let (l, w, ply) = (p.get("length"), p.get("width"), p.get("ply"));
    let (cx, _) = mouth(&p, true);
    let slat_g = PADDLES as f64 * paddle_area(&p) * ply * DENSITY / 1000.0;
    let handle_g = BLOCK as f64 * p.get("handle_len") * p.get("handle_w") * ply * DENSITY / 1000.0;
    let hinge_x = l - p.get("hinge_inset");
    println!("paddle   {l} × {w}mm, three to a slat: {}mm thick, {:.0}g", PADDLES as f64 * ply, slat_g);
    println!("handle   {} × {}mm, four strips: {}mm thick, {:.0}g", p.get("handle_len"), p.get("handle_w"), BLOCK as f64 * ply, handle_g);
    println!("pair     {:.0}g with both handles", 2.0 * (slat_g + handle_g));
    println!(
        "lever    tip {:.1}mm from the hinge, handle {:.0}–{:.0}mm: the tip moves {:.1}× to {:.1}× the hand",
        hinge_x,
        hinge_x - p.get("handle_x") - p.get("handle_len"),
        hinge_x - p.get("handle_x"),
        hinge_x / (hinge_x - p.get("handle_x") - p.get("handle_len")),
        hinge_x / (hinge_x - p.get("handle_x"))
    );
    let mut c = Checks::new();
    let fr = p.get("face_d") / 2.0;
    let eye = (p.get("eye_dx").hypot(p.get("eye_dy")) + p.get("eye_d") / 2.0).max(0.0);
    let (ro, ri, h) = (p.get("mouth_ro"), p.get("mouth_ri"), radians(p.get("mouth_span") / 2.0));
    // the mouth's farthest point from the face's centre, either way round
    let far = [true, false]
        .iter()
        .map(|&hp| {
            let (mx, mid) = mouth(&p, hp);
            let dx = mx - p.get("face_x");
            [radians(mid) - h, radians(mid), radians(mid) + h].iter().map(|a| (dx + ro * a.cos()).hypot(ro * a.sin())).fold(0.0, f64::max)
        })
        .fold(0.0, f64::max);
    c.note(eye <= fr && far <= fr, "eyes and mouth inside the face", format!("{eye:.1}mm and {far:.1}mm from its centre, of {fr}"));
    c.note(ri > 0.0 && ri < ro && h > 0.0 && h < std::f64::consts::FRAC_PI_2, "the mouth is a band", format!("{ri} to {ro}mm, {}°", p.get("mouth_span")));
    let mouth_gap = (p.get("eye_dx") - p.get("eye_d") / 2.0) - (cx - p.get("face_x") - ri * h.cos());
    c.note(mouth_gap >= 1.0, "eyes clear the mouth", format!("{mouth_gap:.1}mm of ply between (>= 1)"));
    c.note(p.get("face_x") - fr >= 0.0 && p.get("face_x") + fr <= p.get("handle_x") && fr <= w / 2.0, "face on the paddle, clear of the handle", format!("{:.1} to {:.1}mm", p.get("face_x") - fr, p.get("face_x") + fr));
    let edge = (w / 2.0 - p.get("hinge_pitch") - p.get("hinge_d") / 2.0).min(p.get("hinge_inset") - p.get("hinge_d") / 2.0);
    c.note(edge >= p.get("hinge_d"), "rope holes clear of the edges", format!("{edge:.1}mm of ply round the outer ones (>= a hole)"));
    c.note(p.get("handle_x") + p.get("handle_len") < hinge_x - p.get("hinge_d") && p.get("handle_w") <= w, "handle between face and hinge", format!("ends {:.1}mm short of the holes", hinge_x - p.get("handle_x") - p.get("handle_len")));
    let s = sheet(&p, true);
    c.note(s.w <= BED.0 && s.h <= BED.1, "one sheet a side", format!("{} × {}mm of the {} × {} bed", s.w, s.h, BED.0, BED.1));
    let rc = c.done();
    if !o.no_write {
        let dir = o.out.clone().unwrap_or_else(|| HERE.to_string());
        let made = p.changed(&defaults);
        let with = if made.is_empty() { String::new() } else { format!(" Made with {made}.") };
        for (happy, stem, side) in [(true, "slapstick_happy", "comedy"), (false, "slapstick_sad", "tragedy")] {
            let desc = format!(
                "1 user unit = 1mm. The slapstick's {side} side, drawn by slapstick.rs: three paddles and five handle strips in {}mm ply.{with} Blue #0000ff engraves the face circle and the handle mark, a line, never a cut. Then green #00ff00 cuts the eyes and mouth, orange #ff8000 the rope holes, cyan #00ffff frees the strips and black #000000 the paddles. Neighbouring parts share a cut line.",
                ply
            );
            write(&format!("{dir}/{stem}.svg"), &sheet(&p, happy).svg(&format!("Slapstick, {side}"), &desc))?;
        }
    }
    Ok(rc)
}

fn main() {
    std::process::exit(match run() {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    });
}
