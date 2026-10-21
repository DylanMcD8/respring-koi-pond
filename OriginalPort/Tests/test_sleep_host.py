"""Execute production sleep-policy callbacks against a small UIKit/OpenAL fixture."""
from pathlib import Path
import subprocess,tempfile
root=Path(__file__).resolve().parents[1]
s=(root/'ios/main.m').read_text()
callbacks=s[s.index('static BOOL koiSleepExpired,'):s.index('static BOOL (*koiOriginalPresent)')]
fixture=r'''
#import <Foundation/Foundation.h>
#include <math.h>
#include <assert.h>
#define UIApplicationStateActive 0
#define UIEventTypeTouches 0
#define UITouchPhaseBegan 0
#define AL_GAIN 0
static float listener=0.8;
static void *alcGetCurrentContext(void){return (void*)1;}
static void alGetListenerf(int p,float *v){*v=listener;}
static void alListenerf(int p,float v){listener=v;}
@interface UIApplication:NSObject
@property BOOL idleTimerDisabled;
@property NSInteger applicationState;
+(instancetype)sharedApplication;
@end
@implementation UIApplication
+(instancetype)sharedApplication {static UIApplication *a;if(!a)a=[self new];return a;}
@end
@interface UITouch:NSObject
@property NSInteger phase;
@end
@implementation UITouch
@end
@interface UIEvent:NSObject
@property NSInteger type;
@property NSSet *allTouches;
@end
@implementation UIEvent
@end
'''+callbacks+r'''
static int forwarded;
static void forward(id a,SEL s,UIEvent *e){forwarded++;}
int main(void){@autoreleasepool {
 UIApplication *app=UIApplication.sharedApplication;
 koiOriginalSendEvent=forward;
 KoiSleepTimerUpdate(0);assert(!koiSleepArmed);
 KoiSleepTimerUpdate(300);assert(koiSleepArmed&&app.idleTimerDisabled);
 KoiSleepTimerFade(0.25);assert(fabs(listener-0.2)<1e-6);
 KoiSleepTimerUpdate(299);assert(fabs(koiSleepBaseGain-0.8)<1e-6);
 KoiSleepTimerExpired();assert(koiSleepExpired&&!app.idleTimerDisabled&&listener==0);
 KoiSleepTimerUpdate(-0.01);assert(koiSleepExpired&&!koiRestoreSleepGain);
 UIEvent *e=[UIEvent new];UITouch *t=[UITouch new];e.allTouches=[NSSet setWithObject:t];
 t.phase=1;KoiSendEvent(app,NULL,e);assert(koiSleepExpired&&forwarded==1);
 t.phase=0;KoiSendEvent(app,NULL,e);
 assert(!koiSleepExpired&&app.idleTimerDisabled&&koiRestoreSleepGain&&forwarded==2);
 KoiSleepTimerUpdate(300);assert(fabs(listener-0.8)<1e-6&&!koiRestoreSleepGain);
 KoiSleepTimerFade(0.5);KoiSleepTimerUpdate(0);
 assert(!koiSleepArmed&&koiRestoreSleepGain);
 KoiSleepTimerUpdate(300);assert(fabs(listener-0.8)<1e-6);
 KoiSleepTimerExpired();KoiSleepTimerUpdate(300);
 assert(!koiSleepExpired&&app.idleTimerDisabled&&fabs(listener-0.8)<1e-6);
 app.applicationState=2;KoiSleepTimerUpdate(0);KoiSleepTimerUpdate(300);
 assert(!app.idleTimerDisabled);
 puts("Sleep host: arming, fading, cancellation, expiration, touch resume, rearming and inactive policy passed");
}}
'''
with tempfile.TemporaryDirectory(prefix='koi-sleep-host-') as folder:
 t=Path(folder);(t/'test.m').write_text(fixture)
 subprocess.run(['xcrun','clang','-fobjc-arc','-framework','Foundation',str(t/'test.m'),'-o',str(t/'test')],check=True)
 subprocess.run([str(t/'test')],check=True)
