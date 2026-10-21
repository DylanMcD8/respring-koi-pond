/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. https://mozilla.org/MPL/2.0/ */
//! Legacy UISlider value, rendering and touch tracking.
use crate::environment::Environment;
use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::objc::{id, nil, impl_HostObject_with_superclass, msg, msg_class, msg_super, objc_classes, ClassExports, NSZonePtr};
use super::{send_actions, UIControlEventValueChanged};
struct UISliderHostObject { superclass: super::UIControlHostObject, value: f32, minimum: f32, maximum: f32, track: id, thumb: id }
impl_HostObject_with_superclass!(UISliderHostObject);
impl Default for UISliderHostObject {
 fn default() -> Self { Self { superclass: Default::default(), value:0.0, minimum:0.0, maximum:1.0, track:nil, thumb:nil } }
}
fn initialize(env: &mut Environment, this: id) -> id {
 let track:id=msg_class![env; UIView new]; let thumb:id=msg_class![env; UIView new];
 let gray:id=msg_class![env; UIColor grayColor]; let white:id=msg_class![env; UIColor whiteColor];
 ()=msg![env; track setBackgroundColor:gray]; ()=msg![env; thumb setBackgroundColor:white];
 ()=msg![env; track setUserInteractionEnabled:false]; ()=msg![env; thumb setUserInteractionEnabled:false];
 ()=msg![env; this addSubview:track]; ()=msg![env; this addSubview:thumb];
 let h=env.objc.borrow_mut::<UISliderHostObject>(this);h.track=track;h.thumb=thumb;
 crate::objc::release(env,track);crate::objc::release(env,thumb);
 ()=msg![env; this layoutSubviews];this
}
fn tracking(env:&mut Environment,this:id,touches:id,event:id) {
 let enabled:bool=msg![env; this isEnabled];if !enabled{return;}
 let touch:id=msg![env; touches anyObject];let p:CGPoint=msg![env; touch locationInView:this];let b:CGRect=msg![env; this bounds];
 let h=env.objc.borrow::<UISliderHostObject>(this);
 let v=h.minimum+((p.x-b.origin.x-10.0)/(b.size.width-20.0).max(1.0)).clamp(0.0,1.0)*(h.maximum-h.minimum);
 ()=msg![env; this setValue:v];send_actions(env,this,event,UIControlEventValueChanged);
}
pub const CLASSES:ClassExports=objc_classes!{
(env,this,_cmd);
@implementation UISlider: UIControl
+ (id)allocWithZone:(NSZonePtr)_zone {env.objc.alloc_object(this,Box::<UISliderHostObject>::default(),&mut env.mem)}
- (id)initWithFrame:(CGRect)frame {let this:id=msg_super![env; this initWithFrame:frame];initialize(env,this)}
- (id)initWithCoder:(id)coder {
 let this:id=msg_super![env; this initWithCoder:coder];
 for (key,field) in [("UIMinValue",0),("UIMaxValue",1),("UIValue",2)] {
  let key=get_static_str(env,key);let has:bool=msg![env; coder containsValueForKey:key];
  if has {let v:f32=msg![env; coder decodeFloatForKey:key];let h=env.objc.borrow_mut::<UISliderHostObject>(this);match field {0=>h.minimum=v,1=>h.maximum=v,_=>h.value=v}}
 }
 initialize(env,this)
}
- (f32)value {env.objc.borrow::<UISliderHostObject>(this).value}
- (())setValue:(f32)value {let h=env.objc.borrow_mut::<UISliderHostObject>(this);h.value=if value.is_finite(){value.max(h.minimum).min(h.maximum)}else{h.minimum};()=msg![env; this layoutSubviews];}
- (())setValue:(f32)value animated:(bool)_animated {()=msg![env; this setValue:value];}
- (f32)minimumValue {env.objc.borrow::<UISliderHostObject>(this).minimum}
- (f32)maximumValue {env.objc.borrow::<UISliderHostObject>(this).maximum}
- (())setMinimumValue:(f32)v {env.objc.borrow_mut::<UISliderHostObject>(this).minimum=v;let value:f32=msg![env; this value];()=msg![env; this setValue:value];}
- (())setMaximumValue:(f32)v {env.objc.borrow_mut::<UISliderHostObject>(this).maximum=v;let value:f32=msg![env; this value];()=msg![env; this setValue:value];}
- (())setMinimumValueImage:(id)_image {}
- (())setMaximumValueImage:(id)_image {}
- (())layoutSubviews {
 let h=env.objc.borrow::<UISliderHostObject>(this);let (track,thumb,v,lo,hi)=(h.track,h.thumb,h.value,h.minimum,h.maximum);if track==nil{return;}
 let b:CGRect=msg![env; this bounds];let w=(b.size.width-20.0).max(1.0);let fraction=if hi>lo{((v-lo)/(hi-lo)).clamp(0.0,1.0)}else{0.0};
 let tr=CGRect{origin:CGPoint{x:b.origin.x+10.0,y:b.origin.y+b.size.height/2.0-2.0},size:CGSize{width:w,height:4.0}};
 let th=CGRect{origin:CGPoint{x:b.origin.x+fraction*w,y:b.origin.y+b.size.height/2.0-10.0},size:CGSize{width:20.0,height:20.0}};
 ()=msg![env; track setFrame:tr];()=msg![env; thumb setFrame:th];
}
- (())touchesBegan:(id)touches withEvent:(id)event {tracking(env,this,touches,event);}
- (())touchesMoved:(id)touches withEvent:(id)event {tracking(env,this,touches,event);}
- (())touchesEnded:(id)touches withEvent:(id)event {tracking(env,this,touches,event);}
@end
};
