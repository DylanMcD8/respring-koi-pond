"""Exercise production timing math without linking the full iOS host."""
from pathlib import Path
import os, subprocess, tempfile
root = Path(__file__).resolve().parents[1]
source = (root / 'compatibility-core/src/frameworks/core_animation/ca_media_timing_function.rs').read_text()
a = source.index('impl CAMediaTimingFunctionHostObject {')
b = source.index('\npub const CLASSES', a)
harness = 'struct CAMediaTimingFunctionHostObject { control_points: [[f32; 2]; 2] }\n' + source[a:b] + r'''
#[test] fn endpoints_and_non_finite() {
 let c=CAMediaTimingFunctionHostObject{control_points:[[0.42,0.0],[0.58,1.0]]};
 for x in [f32::NEG_INFINITY,-1.0,0.0,f32::NAN] {assert_eq!(c.solve_for_input(x),0.0);}
 for x in [1.0,1.5,f32::INFINITY] {assert_eq!(c.solve_for_input(x),1.0);}
}
#[test] fn standard_curves_are_finite_monotonic() {
 for points in [[[0.,0.],[1.,1.]],[[0.42,0.],[0.58,1.]],[[0.42,0.],[1.,1.]],[[0.,0.],[0.58,1.]]] {
  let c=CAMediaTimingFunctionHostObject{control_points:points}; let mut last=0.;
  for i in 0..=100000 {let x=i as f32/100000.;let y=c.solve_for_input(x);assert!(y.is_finite() && y >= last-1e-6 && y <= 1.);last=y;}
 }
}
#[test] fn previously_nonconverging_float_inputs() {
 for (points,bits) in [([[0.,0.],[1.,1.]],0x3f00575cu32), ([[0.42,0.],[1.,1.]],0x3f29b5a6u32)] {
  let c=CAMediaTimingFunctionHostObject{control_points:points};
  let y=c.solve_for_input(f32::from_bits(bits));assert!(y.is_finite());
  if bits==0x3f00575c {assert!((y-0.501333f32).abs()<1e-6);}
 }
}
#[test] fn known_curve_values() {
 let c=CAMediaTimingFunctionHostObject{control_points:[[0.42,0.],[0.58,1.]]};
 for (x,y) in [(0.25,0.12916),(0.5,0.5),(0.75,0.87084)] {assert!((c.solve_for_input(x)-y).abs()<0.0001);}
}
'''
rustc = os.environ.get('RUSTC', 'rustc')
with tempfile.TemporaryDirectory(prefix='koi-timing-tests-') as folder:
 p=Path(folder);(p/'tests.rs').write_text(harness)
 subprocess.run([rustc,'--edition=2021','--test',str(p/'tests.rs'),'-o',str(p/'tests')],check=True)
 subprocess.run([str(p/'tests')],check=True,timeout=15)
