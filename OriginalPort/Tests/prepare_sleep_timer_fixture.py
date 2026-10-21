"""Create a separate accelerated simulator app under the specified output directory."""
from pathlib import Path
import shutil,importlib.util,zipfile,struct,plistlib,sys
root=Path(__file__).resolve().parents[1]
src=root/'.build/iphonesimulator/KoiPond.app'
dst=Path(sys.argv[1])/'iphonesimulator/Koi Pond.app'
assert plistlib.loads((src/'Info.plist').read_bytes())['CFBundleVersion']=='15'
shutil.copytree(src,dst,dirs_exist_ok=True)
spec=importlib.util.spec_from_file_location('patch',root/'patch_framerate.py');p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
for ipa in dst.glob('Koi-Pond*.ipa'):
 tmp=ipa.with_suffix('.tmp')
 with zipfile.ZipFile(ipa) as z,zipfile.ZipFile(tmp,'w') as out:
  for info in z.infolist():
   data=z.read(info)
   if info.filename==p.EXECUTABLE:
    data=bytearray(data);off=p.file_offset(data,0x3d30)
    assert struct.unpack_from('<f',data,off)[0]==60.0
    struct.pack_into('<f',data,off,2.0)
   out.writestr(info,data)
 tmp.replace(ipa)
print('Simulator-only countdown multiplier: 2 seconds/minute. Production IPA untouched.')
