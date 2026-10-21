"""Run the production touch-disturbance timing against controlled event streams."""
from pathlib import Path
import subprocess, tempfile
root = Path(__file__).resolve().parents[1]
source = (root/'compatibility-core/src/cpu/interpreter/koi_touch_timing.rs').read_text()
harness = 'mod timing {\n'+source+'\n}\nuse timing::TouchDisturbanceTiming;\n'+r'''
#[test] fn equal_drag_budget_at_24_60_120_and_irregular_rates() {
 for rate in [24,60,120,240] {
  let mut t=TouchDisturbanceTiming::default();
  assert_eq!(t.strength(0,2.,0.,true),2.);
  let sum:f32=(1..=rate*2).map(|n| t.strength(0,1.,n as f64/rate as f64,true)).sum();
  assert!((sum-48.).abs()<0.0002,"rate={rate} sum={sum}");
 }
 let mut t=TouchDisturbanceTiming::default();t.strength(0,2.,0.,true);
 let mut time=0.; let mut sum=0.;
 for i in 0..200 {time+=if i%2==0 {0.005} else {0.02};sum+=t.strength(0,1.,time,true);}
 assert!((sum-24.*time as f32).abs()<0.0002);
}
#[test] fn taps_and_non_touch_impulses_unchanged() {
 let mut t=TouchDisturbanceTiming::default();
 for n in 0..100 {assert_eq!(t.strength(0,2.,n as f64/120.,true),2.);}
 for slot in 4..8 {assert_eq!(t.strength(slot,1.,0.,true),1.);assert_eq!(t.strength(slot,1.,0.001,true),1.);}
}
#[test] fn original_mode_is_exactly_unchanged() {
 let mut t=TouchDisturbanceTiming::default();
 for n in 0..120 {assert_eq!(t.strength(0,1.,n as f64/120.,false),1.);}
}
#[test] fn independent_fingers_sparse_events_and_new_touches() {
 let mut t=TouchDisturbanceTiming::default();
 for slot in 0..4 {assert_eq!(t.strength(slot,2.,0.,true),2.);assert!((t.strength(slot,1.,1./120.,true)-0.2).abs()<1e-6);}
 assert_eq!(t.strength(0,1.,100.,true),1.);
 assert_eq!(t.strength(0,2.,101.,true),2.);
 assert!((t.strength(0,1.,101.+1./60.,true)-0.4).abs()<1e-6);
}
'''
import shutil
rustc=shutil.which("rustc") or "rustc"
with tempfile.TemporaryDirectory(prefix='koi-touch-tests-') as folder:
 p=Path(folder);(p/'tests.rs').write_text(harness)
 subprocess.run([str(rustc),'--edition=2021','--test',str(p/'tests.rs'),'-o',str(p/'tests')],check=True)
 subprocess.run([str(p/'tests')],check=True)
