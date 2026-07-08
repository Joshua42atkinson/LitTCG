#!/usr/bin/env python3
"""Generate LitTCG app icon at all Android densities."""

from PIL import Image, ImageDraw, ImageFont
import math
import os

# Android icon sizes (square; Android crops to circle/squircle)
DENSITIES = {
    "mipmap-mdpi": 48,
    "mipmap-hdpi": 72,
    "mipmap-xhdpi": 96,
    "mipmap-xxhdpi": 144,
    "mipmap-xxxhdpi": 192,
}

# LitTCG brand colors (deep literary blue + arcane gold)
BG = (18, 24, 36)         # #121824
PANEL = (35, 45, 65)      # #232d41
GOLD = (232, 198, 110)    # #e8c66e
SOFT_GLOW = (120, 160, 220, 180)  # rgba
WHITE = (245, 245, 240)   # #f5f5f0


def draw_card_shape(draw, cx, cy, w, h, radius, fill, outline=None, outline_width=1):
    """Draw a rounded rectangle card shape."""
    x0 = cx - w // 2
    y0 = cy - h // 2
    x1 = cx + w // 2
    y1 = cy + h // 2
    draw.rounded_rectangle([(x0, y0), (x1, y1)], radius=radius, fill=fill)
    if outline:
        for i in range(outline_width):
            draw.rounded_rectangle(
                [(x0 + i, y0 + i), (x1 - i, y1 - i)],
                radius=max(0, radius - i),
                outline=outline,
                width=1,
            )


def draw_sparkles(draw, cx, cy, size, color, count=4):
    """Draw small sparkle accents around a point."""
    for i in range(count):
        angle = (i / count) * 2 * math.pi
        r = size * 0.38
        sx = cx + math.cos(angle) * r
        sy = cy + math.sin(angle) * r
        r_dot = max(1, size // 24)
        draw.ellipse(
            [(sx - r_dot, sy - r_dot), (sx + r_dot, sy + r_dot)],
            fill=color,
        )


def generate_icon(size):
    """Generate one icon at the given size."""
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Outer rounded background (squircle)
    corner = size // 5
    draw.rounded_rectangle(
        [(0, 0), (size - 1, size - 1)],
        radius=corner,
        fill=BG,
    )

    # Inner panel (the "card back")
    margin = size // 12
    draw_card_shape(
        draw,
        size // 2,
        size // 2,
        size - 2 * margin,
        int((size - 2 * margin) * 1.35),
        radius=max(1, size // 10),
        fill=PANEL,
        outline=GOLD,
        outline_width=max(1, size // 48),
    )

    # Central "L" glyph or book motif
    # For small sizes, a bold gold 'L' reads best
    try:
        # Use Bevy font if available, else default
        font_path = os.path.join(
            os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
            "assets",
            "Orbitron-Regular.ttf",
        )
        if os.path.exists(font_path):
            font_size = int(size * 0.55)
            font = ImageFont.truetype(font_path, font_size)
        else:
            raise FileNotFoundError("Orbitron font not found")
    except Exception:
        font = ImageFont.load_default()

    text = "L"
    bbox = draw.textbbox((0, 0), text, font=font)
    tw = bbox[2] - bbox[0]
    th = bbox[3] - bbox[1]
    tx = (size - tw) // 2
    ty = (size - th) // 2 - size // 24  # nudge up slightly
    draw.text((tx, ty), text, font=font, fill=GOLD)

    # Sparkle accents
    draw_sparkles(draw, size // 2, size // 5, size, GOLD, count=5)

    return img


def main():
    base = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    for folder, size in DENSITIES.items():
        path = os.path.join(base, "res", folder, "ic_launcher.png")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        icon = generate_icon(size)
        icon.save(path, "PNG")
        print(f"Generated {path} ({size}x{size})")

    # High-res source asset (store listing / store graphics)
    src_path = os.path.join(base, "assets", "branding", "ic_launcher_source.png")
    os.makedirs(os.path.dirname(src_path), exist_ok=True)
    src = generate_icon(512)
    src.save(src_path, "PNG")
    print(f"Generated {src_path} (512x512) — source asset")


if __name__ == "__main__":
    main()
