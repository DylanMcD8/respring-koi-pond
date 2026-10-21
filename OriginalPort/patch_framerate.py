#!/usr/bin/env python3
"""Build a guarded, data-only frame-rate variant of the original Koi Pond IPA.

The original 24 Hz timer advances most game logic by a fixed 1/30 second.
Retain that historical animation speed when requesting more updates. Water
uses per-step velocity: its spring coefficient scales with dt squared, damping
exponentially with dt, and random acceleration with dt**1.5. This approximates
its original continuous motion; it is not bit-identical 24 Hz simulation.
No ARM instructions or generated translations are changed.
"""
import argparse
import hashlib
import json
import struct
import zipfile
from pathlib import Path

EXECUTABLE = 'Payload/KoiPond.app/KoiPond'
ORIGINAL_SHA256 = '0f375e3e96e748faac4505a05924e458e1fadd69d2726bf02b6de3b31d99d8d6'

def file_offset(data, address):
    assert struct.unpack_from('<I', data)[0] == 0xfeedface
    offset = 28
    for _ in range(struct.unpack_from('<I', data, 16)[0]):
        command, size = struct.unpack_from('<II', data, offset)
        if command == 1:
            vmaddr, vmsize, fileoff, filesize = struct.unpack_from('<IIII', data, offset + 24)
            if vmaddr <= address < vmaddr + filesize:
                return fileoff + address - vmaddr
        offset += size
    raise ValueError(f'Unmapped address {address:#x}')

def patch(data, fps):
    if hashlib.sha256(data).hexdigest() != ORIGINAL_SHA256:
        raise ValueError("This patch only supports the verified original Koi Pond 2.5 executable")
    if fps not in (24, 60, 120):
        raise ValueError('Expected 24, 60 or 120 FPS')
    result = bytearray(data)
    changes = []
    def word(address, expected, value, label):
        offset = file_offset(data, address)
        actual = struct.unpack_from('<I', data, offset)[0]
        if actual != expected:
            raise ValueError(f'{label} signature mismatch at {address:#x}: {actual:#x}')
        struct.pack_into('<I', result, offset, value)
        changes.append(dict(address=hex(address), label=label, before=hex(expected), after=hex(value)))
    def floating(address, expected, value, label):
        word(address, struct.unpack('<I', struct.pack('<f', expected))[0],
             struct.unpack('<I', struct.pack('<f', value))[0], label)
    lo, hi = struct.unpack('<II', struct.pack('<d', 1 / fps))
    for address, value in [(0xe338,lo),(0xe340,hi),(0xe7bc,lo),(0xe7c0,hi)]:
        word(address, 0x55555555 if address in (0xe338,0xe7bc) else 0x3fa55555,
             value, 'foreground timer interval')
    ratio = 24 / fps
    for address in (0x8468, 0x8494):
        floating(address, 1/30, ratio/30, 'fish/rain/ambient fixed timestep')
    floating(0xea58, 0.1, 0.1*ratio, 'UI fade step')
    floating(0xad7c, 6.0, 6.0*ratio, 'fish speed ramp')
    floating(0xad84, 0.07, 1-0.93**ratio, 'fish speed smoothing')
    floating(0x3fd0, 0.7, 0.7*ratio**2, 'water spring coefficient')
    floating(0x3fd4, -0.04, 0.96**ratio - 1, 'water velocity damping')
    for address, expected in [(0x3fc4,0.003),(0x3fc8,0.002)]:
        floating(address, expected, expected*ratio**1.5, 'water random acceleration')
    return bytes(result), changes

def build(source, destination, fps):
    with zipfile.ZipFile(source) as src, zipfile.ZipFile(destination, 'w') as dst:
        data = src.read(EXECUTABLE)
        patched, changes = patch(data, fps)
        for info in src.infolist():
            dst.writestr(info, patched if info.filename == EXECUTABLE else src.read(info.filename))
    return dict(fps=fps, source_executable_sha256=hashlib.sha256(data).hexdigest(),
                patched_executable_sha256=hashlib.sha256(patched).hexdigest(), changes=changes)

if __name__ == '__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('source', type=Path); parser.add_argument('destination', type=Path)
    parser.add_argument('--fps', type=int, choices=(24,60,120), required=True)
    args=parser.parse_args()
    print(json.dumps(build(args.source,args.destination,args.fps),indent=2))
