"""Check original binary signatures and execute production timer-hook bodies."""
from pathlib import Path
import importlib.util, struct, subprocess, tempfile
root=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('patch',root/'patch_framerate.py')
p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
data=(root/'../Original/Extracted/Payload/KoiPond.app/KoiPond').read_bytes()
for addr,word in [(0x8040,0xda000014),(0x8078,0xeb001d8e),(0x8094,0x9a0006bb)]:
 assert struct.unpack_from('<I',data,p.file_offset(data,addr))[0]==word
source=(root/'compatibility-core/src/cpu/interpreter/mod.rs').read_text()
methods=source[source.index('    fn koi_sleep_check'):source.index('    pub fn new(null_page_count')]
methods=methods.replace('#[cfg(target_os = "ios")]','')
harness=r'''
use std::sync::Mutex;
static EVENTS:Mutex<Vec<f32>>=Mutex::new(Vec::new());
#[no_mangle] extern "C" fn KoiSleepTimerUpdate(s:f32){EVENTS.lock().unwrap().push(s);}
#[no_mangle] extern "C" fn KoiSleepTimerFade(g:f32){EVENTS.lock().unwrap().push(g);}
#[no_mangle] extern "C" fn KoiSleepTimerExpired(){EVENTS.lock().unwrap().push(-999.);}
struct Mem;
type CpuState=u32;
struct InterpreterCpu {regs:[u32;16],extregs:[u32;64],cpsr:u32}
impl InterpreterCpu {
 fn cond_passed(&self,c:u32)->bool {
  let z=self.cpsr&(1<<30)!=0;let carry=self.cpsr&(1<<29)!=0;
  let n=self.cpsr&(1<<31)!=0;let v=self.cpsr&(1<<28)!=0;
  match c {9=>!carry||z,13=>z||(n!=v),_=>panic!()}
 }
 fn exec_arm(&mut self,w:u32,pc:u32,_:&mut Mem)->Option<CpuState> {
  assert!(w==0xe1a00000 || (w==0xda000014 && pc==0x8040));
  let next=if w==0xda000014 && self.cond_passed(13){0x8098}else{pc+4};
  self.regs[15]=next;Some(next)
 }
'''+methods+r'''
}
#[test] fn original_timer_hooks() {
 let mut c=InterpreterCpu{regs:[0;16],extregs:[0;64],cpsr:0};let mut m=Mem;
 // Exhaust every NZCV combination: the old exit branch is never taken,
 // and native expiration is signalled only when the original BLS is taken.
 for flags in 0..16 {
  EVENTS.lock().unwrap().clear();c.cpsr=flags<<28;
  let expired=c.cond_passed(9);
  assert_eq!(c.koi_sleep_expire(&mut m),Some(0x8098));
  assert_eq!(EVENTS.lock().unwrap().len(),usize::from(expired));
 }
 for (seconds,flags,next) in [(0.,1<<30,0x8098),(10.,1<<29,0x8044)] {
  EVENTS.lock().unwrap().clear();c.extregs[15]=f32::to_bits(seconds);c.cpsr=flags;
  assert_eq!(c.koi_sleep_check(&mut m),Some(next));
  assert_eq!(*EVENTS.lock().unwrap(),vec![seconds]);
 }
 // Listener fade must be independent of the user's source volume.
 for volume in [0.2f32,0.5,1.] {
  EVENTS.lock().unwrap().clear();c.extregs[13]=volume.to_bits();
  c.regs[0]=(volume*volume*0.25).to_bits();
  assert_eq!(c.koi_sleep_fade(&mut m),Some(0x807c));
  assert!((EVENTS.lock().unwrap()[0]-0.25).abs()<1e-6);
 }
 c.extregs[13]=0; c.regs[0]=0; c.koi_sleep_fade(&mut m);
 assert_eq!(*EVENTS.lock().unwrap().last().unwrap(),0.);
}
'''
import shutil
rustc=shutil.which("rustc") or "rustc"
with tempfile.TemporaryDirectory(prefix='koi-sleep-tests-') as folder:
 t=Path(folder);(t/'test.rs').write_text(harness)
 subprocess.run([str(rustc),'--edition=2021','--test',str(t/'test.rs'),'-o',str(t/'test')],check=True)
 subprocess.run([str(t/'test')],check=True)
