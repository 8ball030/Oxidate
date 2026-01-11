from __future__ import annotations

import argparse
import shutil
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter


BG = (20, 24, 30, 255)
INNER = (34, 40, 52, 255)


def _make_vertical_gradient(height: int, top: tuple[int, int, int, int], bottom: tuple[int, int, int, int]) -> Image.Image:
    col = Image.new("RGBA", (1, height))
    px = col.load()
    for y in range(height):
        t = y / max(1, (height - 1))
        px[0, y] = (
            int(top[0] + (bottom[0] - top[0]) * t),
            int(top[1] + (bottom[1] - top[1]) * t),
            int(top[2] + (bottom[2] - top[2]) * t),
            int(top[3] + (bottom[3] - top[3]) * t),
        )
    return col


def render_icon(base_size: int = 1024, supersample: int = 4) -> Image.Image:
    """Render a high-quality icon with antialias.

    We draw at (base_size * supersample) and downscale with Lanczos.
    """
    ss = int(base_size * supersample)
    cx = cy = ss / 2

    # Geometry tuned for the 1024px final icon.
    r_outer = int(0.41 * ss)
    r_inner = int(0.295 * ss)

    img = Image.new("RGBA", (ss, ss), BG)

    # Ring mask (donut)
    ring_mask = Image.new("L", (ss, ss), 0)
    dmask = ImageDraw.Draw(ring_mask)
    dmask.ellipse([cx - r_outer, cy - r_outer, cx + r_outer, cy + r_outer], fill=255)
    dmask.ellipse([cx - r_inner, cy - r_inner, cx + r_inner, cy + r_inner], fill=0)
    # Slight blur makes the ring edge extra smooth after downscale.
    ring_mask = ring_mask.filter(ImageFilter.GaussianBlur(radius=1.2 * supersample))

    # Vertical gradient for the ring
    grad_col = _make_vertical_gradient(
        ss,
        top=(245, 150, 45, 255),
        bottom=(205, 120, 35, 255),
    )
    grad = grad_col.resize((ss, ss), resample=Image.Resampling.BILINEAR)
    ring = Image.composite(grad, Image.new("RGBA", (ss, ss), (0, 0, 0, 0)), ring_mask)
    img.alpha_composite(ring)

    # Inner fill
    draw = ImageDraw.Draw(img)
    inner_r = r_inner - int(0.01 * ss)
    draw.ellipse([cx - inner_r, cy - inner_r, cx + inner_r, cy + inner_r], fill=INNER)

    # Soft highlight dot
    hd_r = int(0.05 * ss)
    hx = cx + int(0.23 * ss)
    hy = cy - int(0.21 * ss)
    dot = Image.new("RGBA", (ss, ss), (0, 0, 0, 0))
    dd = ImageDraw.Draw(dot)
    dd.ellipse([hx - hd_r, hy - hd_r, hx + hd_r, hy + hd_r], fill=(255, 230, 160, 180))
    dot = dot.filter(ImageFilter.GaussianBlur(radius=0.012 * ss))
    img.alpha_composite(dot)

    # Downscale to final base size.
    return img.resize((base_size, base_size), resample=Image.Resampling.LANCZOS)


def write_png(img: Image.Image, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)


def write_iconset(base_png: Image.Image, iconset_dir: Path) -> None:
    # macOS iconset required files:
    # 16, 32, 128, 256, 512 and their @2x counterparts.
    sizes: list[tuple[str, int]] = [
        ("icon_16x16.png", 16),
        ("icon_16x16@2x.png", 32),
        ("icon_32x32.png", 32),
        ("icon_32x32@2x.png", 64),
        ("icon_128x128.png", 128),
        ("icon_128x128@2x.png", 256),
        ("icon_256x256.png", 256),
        ("icon_256x256@2x.png", 512),
        ("icon_512x512.png", 512),
        ("icon_512x512@2x.png", 1024),
    ]

    if iconset_dir.exists():
        shutil.rmtree(iconset_dir)
    iconset_dir.mkdir(parents=True, exist_ok=True)

    for name, px in sizes:
        out = iconset_dir / name
        resized = base_png.resize((px, px), resample=Image.Resampling.LANCZOS)
        resized.save(out)


def build_icns(iconset_dir: Path, icns_path: Path) -> None:
    icns_path.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(["iconutil", "-c", "icns", str(iconset_dir), "-o", str(icns_path)], check=True)


def main() -> None:
    p = argparse.ArgumentParser(description="Generate Oxidate app icon assets (PNG + iconset + ICNS).")
    p.add_argument("--assets-dir", default="assets", help="Assets output directory (default: assets)")
    p.add_argument("--base-size", type=int, default=1024, help="Base PNG size (default: 1024)")
    p.add_argument("--supersample", type=int, default=4, help="Supersample factor for antialias (default: 4)")
    args = p.parse_args()

    assets_dir = Path(args.assets_dir)
    base_png_path = assets_dir / "oxidate.png"
    iconset_dir = assets_dir / "oxidate.iconset"
    icns_path = assets_dir / "oxidate.icns"

    base = render_icon(base_size=args.base_size, supersample=max(1, args.supersample))
    write_png(base, base_png_path)
    write_iconset(base, iconset_dir)
    build_icns(iconset_dir, icns_path)

    print(f"wrote {base_png_path}")
    print(f"wrote {icns_path}")


if __name__ == "__main__":
    main()
