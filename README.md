# Slapstick

Two long slats joined at one end. Swing it and the free ends clap together with a crack
far louder than the effort suggests — the sound effect that gave stage comedy its name.
This one carries a face on each side, from the **comedy and tragedy masks**: laughing on
one, weeping on the other. Output is millimetre-true — `1 user unit = 1 mm` with a
physical `width`/`height` — so it prints and cuts at real size.

<table>
<tr>
<td align="center"><a href="slapstick_happy.svg"><img src="previews/slapstick_happy.svg" alt="The happy sheet: three long paddle blanks each engraved with a face outline and a rectangle, with eyes, a smiling mouth and three holes cut, and five short handle strips below" width="620"></a></td>
</tr>
<tr>
<td align="center"><sub>slapstick_happy.svg · comedy · 495 × 297mm</sub></td>
</tr>
<tr>
<td align="center"><a href="slapstick_sad.svg"><img src="previews/slapstick_sad.svg" alt="The sad sheet: the same parts, with the mouth curving the other way" width="620"></a></td>
</tr>
<tr>
<td align="center"><sub>slapstick_sad.svg · tragedy · 495 × 297mm</sub></td>
</tr>
</table>

*One sheet per side. Click either to download it — these are display renderings, since the
cut files draw a hairline on no background that a browser shows almost invisibly.*

**[Read the writeup](https://gernreich.github.io/slapstick/)**

Built for **[LaserMadeMusic](https://www.youtube.com/@LaserMadeMusic)**, where the cutting
and assembly are shown.

**[Download everything as a ZIP](https://github.com/Gernreich/slapstick/archive/refs/heads/main.zip)**

## Colour is the cut order

**Red is not a cut.** The rest run first to last:

| | Colour | What it is | When |
|---|---|---|---|
| | red `#ff0000` | face outline and glue rectangle | **engrave — never cut** |
| 1 | green `#00ff00` | eyes and mouth | first cut |
| 2 | blue `#0000ff` | the three holes at the far end | second cut |
| 3 | magenta `#ff00ff` | the five handle strips | third cut |
| 4 | black `#000000` | the three paddle outlines | last cut |

Outlines last, because once a cut frees a part anything still to be cut inside it can
move. A per-colour job silently skips any colour you leave unmapped.

## What is on each sheet

Measured out of the files. Both sheets are identical here — 32 objects each:

| Part | Count | Size |
|---|---|---|
| Paddle blank | 3 | 479.7 × 89.7mm |
| Handle strip | 5 | 90 × 25mm |
| Face outline, engraved | 3 | 50mm circle, centred 56.7mm from the left end |
| Glue rectangle, engraved | 3 | 90 × 25mm, starting 294.5mm from the left end |
| Eyes | 6 | 6mm each |
| Mouth | 3 | 9.5 × 26mm |
| Holes | 9 | ~5mm, 12.3mm from the right end |

The face reads sideways on the sheet because the paddle is laid out along its length; it
comes upright when the slapstick is held to swing.

## Building it

1. **Glue three paddle blanks face to face** — three for comedy, three for tragedy. Each
   laminate is one slat.
2. **Glue the five short strips into one block.** That is the handle.
3. **Glue the handle over the engraved rectangle**, which is there to show you where. It
   is a mark, not a recess — engraving removes very little material.
4. The three holes at the far end are cut but unused by these files; how the two slats
   join is yours to decide.

## Before you cut

**Material and thickness are yours, and nothing here has been validated against cut
stock.** Three laminated layers set how stiff each slat is, and a slapstick's whole job is
to be swung hard and stopped abruptly.

**It is loud on purpose, and it is a lever.** Keep fingers clear of where the slats meet.

## Also here

`SlapstickSmilyFace.svg` and `SlapstickSadFace.svg` — two earlier 100mm discs, each a face
cut clean through, with no hinge or fixing holes. They decorate something you build,
rather than being part of the slapstick above.

## Files

| | |
|---|---|
| `slapstick_happy.svg` · `slapstick_sad.svg` | one sheet per side |
| `SlapstickSmilyFace.svg` · `SlapstickSadFace.svg` | the two 100mm face discs |
| `previews/` | display renderings — **not** cut files |
| `index.md` · `index.html` | the published page; the markdown is the source |

Released under [CC0 1.0](LICENSE).
