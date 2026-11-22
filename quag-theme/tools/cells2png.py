"""Render ratatui cell dumps (*.cells from `--dump`) to PNG screenshots.
usage: python3 -I tools/cells2png.py <cells_dir> <out_dir>
"""
import sys, re, glob, os
from PIL import Image, ImageDraw, ImageFont
FONT = os.path.expanduser('~/.local/share/fonts/JetBrainsMono/JetBrainsMonoNerdFontMono-%s.ttf')
reg = ImageFont.truetype(FONT % 'Regular', 17); bold = ImageFont.truetype(FONT % 'Bold', 17)
CW, CH = 10, 22; BG = (0x0E, 0x16, 0x20)
def col(s, default):
    m = re.match(r'Rgb\((\d+), (\d+), (\d+)\)', s)
    return tuple(map(int, m.groups())) if m else default
src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)
for p in sorted(glob.glob(src + '/*.cells')):
    cells = [l.rstrip('\n').split('\t') for l in open(p, encoding='utf8')]
    W = max(int(c[0]) for c in cells) + 1; H = max(int(c[1]) for c in cells) + 1
    pad = 16; im = Image.new('RGB', (W * CW + 2 * pad, H * CH + 2 * pad), BG); d = ImageDraw.Draw(im)
    for x, y, sym, fg, bg, b, u in cells:
        x, y = int(x), int(y); X, Y = pad + x * CW, pad + y * CH
        fgc = col(fg, (0xE4, 0xF3, 0xFB)); bgc = col(bg, BG)
        d.rectangle([X, Y, X + CW - 1, Y + CH - 1], fill=bgc)
        blocks = {'▀': (0, 0, CW, CH // 2), '▄': (0, CH // 2, CW, CH), '█': (0, 0, CW, CH),
                  '▌': (0, 0, CW // 2, CH), '▎': (0, 0, CW // 4, CH)}
        if sym in blocks:
            a, b_, c, e = blocks[sym]; d.rectangle([X + a, Y + b_, X + c - 1, Y + e - 1], fill=fgc); continue
        if sym == '━': d.rectangle([X, Y + CH // 2 - 2, X + CW - 1, Y + CH // 2 + 1], fill=fgc); continue
        if sym.strip():
            d.text((X, Y + 1), sym, font=bold if b == '1' else reg, fill=fgc)
        if u == '1': d.line([X, Y + CH - 3, X + CW, Y + CH - 3], fill=fgc)
    out = os.path.join(dst, os.path.basename(p).replace('.cells', '.png')); im.save(out); print(out)
