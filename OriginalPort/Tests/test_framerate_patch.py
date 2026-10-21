import importlib.util
from pathlib import Path
import struct
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('patch_framerate',ROOT/'patch_framerate.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
ORIGINAL=(ROOT/'../Original/Extracted/Payload/KoiPond.app/KoiPond').read_bytes()

def value(data,address,fmt='<f'):
    return struct.unpack_from(fmt,data,module.file_offset(data,address))[0]

class FrameratePatchTests(unittest.TestCase):
    def test_legacy_is_unchanged(self):
        self.assertEqual(module.patch(ORIGINAL,24)[0],ORIGINAL)
    def test_refuses_unknown_executable(self):
        with self.assertRaises(ValueError):module.patch(ORIGINAL+b'changed',60)
    def test_motion_seconds_are_preserved(self):
        for fps in (60,120):
            data,_=module.patch(ORIGINAL,fps)
            for address in (0x8468,0x8494):
                self.assertAlmostEqual(value(data,address)*fps,value(ORIGINAL,address)*24,places=6)
            self.assertAlmostEqual(value(data,0xe7bc,'<d'),1/fps)
            low=value(data,0xe338,'<I');high=value(data,0xe340,'<I')
            self.assertEqual(struct.unpack('<d',struct.pack('<II',low,high))[0],1/fps)
    def test_water_coefficients_preserve_time_units(self):
        for fps in (60,120):
            data,_=module.patch(ORIGINAL,fps);ratio=24/fps
            self.assertAlmostEqual(value(data,0x3fd0)/ratio**2,value(ORIGINAL,0x3fd0),places=6)
            self.assertAlmostEqual((1+value(data,0x3fd4))**fps,(1+value(ORIGINAL,0x3fd4))**24,places=6)
            self.assertAlmostEqual(value(data,0x3fc4)**2*fps/ratio**2,value(ORIGINAL,0x3fc4)**2*24,places=9)

if __name__=='__main__':unittest.main()
