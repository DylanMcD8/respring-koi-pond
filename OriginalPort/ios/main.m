#include "render_scale.h"
#include <SDL.h>
#include <SDL_main.h>
#include <stdlib.h>
#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import <UIKit/UIKit.h>

#import <OpenGLES/EAGL.h>
#import <objc/runtime.h>
#import <OpenAL/al.h>
#import <OpenAL/alc.h>
// SDL owns one drawable view per context. Its window can be created before
// UIKit connects the scene; attach it once a foreground scene is available.
@interface EAGLContext (SkeeSDLDrawable)
@property(nonatomic, readonly) UIView *sdlView;
@end
void KoiEnsureDrawableScene(void) {
    EAGLContext *context=EAGLContext.currentContext;
    if (![context respondsToSelector:@selector(sdlView)]) return;
    static UIWindow *drawableWindow;
    UIView *view=context.sdlView;
    UIWindow *window=view.window;
    if (window) drawableWindow=window;
    else if (drawableWindow && view) {
        // SDL keeps multiple contexts associated with one window, but creating
        // a guest context replaces the compositor view without clearing the
        // old view's SDL association. Restore the view that is presenting.
        drawableWindow.rootViewController.view=view;
        window=drawableWindow;
    }
    if (!window || window.windowScene) return;
    for (UIScene *scene in UIApplication.sharedApplication.connectedScenes) {
        if (![scene isKindOfClass:UIWindowScene.class] ||
            scene.activationState == UISceneActivationStateBackground ||
            scene.activationState == UISceneActivationStateUnattached) continue;
        UIWindowScene *windowScene=(UIWindowScene *)scene;
        window.windowScene=windowScene;
        id<UIWindowSceneDelegate> delegate=(id)scene.delegate;
        if ([delegate respondsToSelector:@selector(window)] && delegate.window != window)
            delegate.window.hidden=YES;
        if ([delegate respondsToSelector:@selector(setWindow:)]) delegate.window=window;
        [window makeKeyAndVisible];
        if (@available(iOS 27.1,*)) {
            // Duo inner displays resize independently of physical orientation.
            [window.rootViewController setNeedsUpdateOfSupportedInterfaceOrientations];
        } else if (@available(iOS 16.0,*)) {
            [window.rootViewController setNeedsUpdateOfSupportedInterfaceOrientations];
            UIWindowSceneGeometryPreferencesIOS *geometry=[[UIWindowSceneGeometryPreferencesIOS alloc]
                initWithInterfaceOrientations:UIInterfaceOrientationMaskPortrait];
            [windowScene requestGeometryUpdateWithPreferences:geometry errorHandler:^(NSError *error) {
                NSLog(@"Koi Pond portrait orientation: %@",error);
            }];
        }
        break;
    }
}

// UIKit converts touch locations through this transform, so SDL's existing
// normalized-input mapping remains aligned with the enlarged pond.
static BOOL koiFillScreenCropped;
static void KoiApplyCroppedPresentation(EAGLContext *context) {
    if (![context respondsToSelector:@selector(sdlView)]) return;
    UIView *view = context.sdlView;
    if (!view || !view.window) return;
    UIWindow *window = view.window;
    if (@available(iOS 27.1,*)) {
        // SDL's original window was sized for one screen/orientation. Refresh
        // from the active scene after rotation or a Duo display transition.
        CGRect sceneBounds = window.windowScene.coordinateSpace.bounds;
        if (sceneBounds.size.width > 0 && sceneBounds.size.height > 0 &&
            !CGRectEqualToRect(window.frame, sceneBounds)) window.frame = sceneBounds;
    }
    CGSize windowSize = window.bounds.size;
    if (windowSize.width > 0 && windowSize.height > 0 &&
        !CGSizeEqualToSize(view.bounds.size, windowSize)) {
        // Never assign frame while the previous crop transform is active.
        view.transform = CGAffineTransformIdentity;
        view.bounds = (CGRect){CGPointZero, windowSize};
        view.center = CGPointMake(CGRectGetMidX(window.bounds), CGRectGetMidY(window.bounds));
        [view setNeedsLayout];
        NSLog(@"Koi drawable resized: %.0fx%.0f", windowSize.width, windowSize.height);
    }
    CGSize size = view.bounds.size;
    if (size.width <= 0 || size.height <= 0) return;
    CGFloat zoom = KoiPresentationZoom(size.width, size.height, koiFillScreenCropped);
    CGAffineTransform transform = CGAffineTransformMakeScale(zoom, zoom);
    if (!CGAffineTransformEqualToTransform(view.transform, transform)) {
        view.transform = transform;
        view.window.clipsToBounds = YES;
        NSLog(@"Koi presentation: cropped=%d zoom=%.4f view=%.0fx%.0f window=%.0fx%.0f",
              koiFillScreenCropped, zoom, size.width, size.height,
              view.window.bounds.size.width, view.window.bounds.size.height);
    }
}

// Timer expiration permits normal iOS Auto-Lock; public UIKit cannot force
// an immediate lock. Keep this state across interruptions until actual input.
static BOOL koiSleepExpired, koiSleepArmed, koiRestoreSleepGain;
static float koiSleepBaseGain = 1.0f;
void KoiSleepTimerUpdate(float seconds) {
    if (seconds > 0 && !koiSleepArmed) {
        if (koiSleepExpired || koiRestoreSleepGain) {
            if (alcGetCurrentContext()) alListenerf(AL_GAIN, koiSleepBaseGain);
            koiRestoreSleepGain = NO;
        } else if (alcGetCurrentContext()) alGetListenerf(AL_GAIN, &koiSleepBaseGain);
        koiSleepArmed = YES;
        koiSleepExpired = NO;
        UIApplication.sharedApplication.idleTimerDisabled =
            UIApplication.sharedApplication.applicationState == UIApplicationStateActive;
        NSLog(@"Koi sleep timer armed");
    } else if (seconds <= 0 && koiSleepArmed && !koiSleepExpired) {
        koiSleepArmed = NO;
        koiRestoreSleepGain = YES;
    }
}
void KoiSleepTimerFade(float gain) {
    if (alcGetCurrentContext()) alListenerf(AL_GAIN, koiSleepBaseGain * fmaxf(0, fminf(1, gain)));
}
void KoiSleepTimerExpired(void) {
    koiSleepExpired = YES;
    koiSleepArmed = NO;
    KoiSleepTimerFade(0);
    UIApplication.sharedApplication.idleTimerDisabled = NO;
    NSLog(@"Koi sleep timer expired: idleTimerDisabled=%d, pond retained", UIApplication.sharedApplication.idleTimerDisabled);
}
static void (*koiOriginalSendEvent)(id, SEL, UIEvent *);
static void KoiSendEvent(id app, SEL selector, UIEvent *event) {
    if ((koiSleepExpired || koiSleepArmed) && event.type == UIEventTypeTouches) {
        for (UITouch *touch in event.allTouches) {
            if (touch.phase != UITouchPhaseBegan) continue;
            koiSleepExpired = NO;
            koiRestoreSleepGain = YES;
            UIApplication.sharedApplication.idleTimerDisabled = YES;
            NSLog(@"Koi sleep timer: touch resumed pond, idleTimerDisabled=%d", UIApplication.sharedApplication.idleTimerDisabled);
            break;
        }
    }
    koiOriginalSendEvent(app, selector, event);
}

static BOOL (*koiOriginalPresent)(id,SEL,NSUInteger);
static BOOL KoiPresent(id context, SEL selector, NSUInteger target) {
    if (koiRestoreSleepGain && alcGetCurrentContext()) {
        alListenerf(AL_GAIN, koiSleepBaseGain);
        koiRestoreSleepGain = NO;
    }
    KoiEnsureDrawableScene();
    KoiApplyCroppedPresentation(context);
    if (getenv("KOI_MUTE") && alcGetCurrentContext()) alListenerf(AL_GAIN,0.0f);
    return koiOriginalPresent(context,selector,target);
}

extern void ChuzzleFrameTimingSetActive(BOOL active);
static UIBackgroundTaskIdentifier backgroundSave;

void ChuzzleFinishBackgroundSave(void) {
    if (backgroundSave == UIBackgroundTaskInvalid) return;
    UIBackgroundTaskIdentifier task = backgroundSave;
    backgroundSave = UIBackgroundTaskInvalid;
    [UIApplication.sharedApplication endBackgroundTask:task];
}

// Keep the native mixer context across session deactivation. Merely resuming
// AL sources is insufficient after iOS disconnects the audio output.
static ALCcontext *koiPausedAudioContext;
void ChuzzleHostAudioActive(int active) {
    NSError *error = nil;
    if (!active) {
        ALCcontext *context = alcGetCurrentContext();
        if (context) {
            koiPausedAudioContext = context;
            alcSuspendContext(context);
            alcMakeContextCurrent(NULL);
        }
    }
    BOOL activated = [AVAudioSession.sharedInstance setActive:(active != 0)
        withOptions:active ? 0 : AVAudioSessionSetActiveOptionNotifyOthersOnDeactivation error:&error];
    if (active && activated && koiPausedAudioContext) {
        BOOL restored = alcMakeContextCurrent(koiPausedAudioContext);
        if (restored) alcProcessContext(koiPausedAudioContext);
        NSLog(@"Koi audio context restored=%d", restored);
    }
    if (error) NSLog(@"Koi audio lifecycle: %@", error);
}

static void KoiObserveAudioInterruptions(void) {
    [NSNotificationCenter.defaultCenter addObserverForName:AVAudioSessionInterruptionNotification
        object:AVAudioSession.sharedInstance queue:NSOperationQueue.mainQueue
        usingBlock:^(NSNotification *notification) {
            NSUInteger type = [notification.userInfo[AVAudioSessionInterruptionTypeKey] unsignedIntegerValue];
            if (type == AVAudioSessionInterruptionTypeBegan) {
                ChuzzleHostAudioActive(0);
            } else if (UIApplication.sharedApplication.applicationState == UIApplicationStateActive &&
                ([notification.userInfo[AVAudioSessionInterruptionOptionKey] unsignedIntegerValue] &
                 AVAudioSessionInterruptionOptionShouldResume)) {
                ChuzzleHostAudioActive(1);
            }
        }];
}

@interface ChuzzleSceneDelegate : NSObject <UIWindowSceneDelegate>
@property (nonatomic, strong) UIWindow *window;
@end

@implementation ChuzzleSceneDelegate
- (void)scene:(UIScene *)scene willConnectToSession:(UISceneSession *)session options:(UISceneConnectionOptions *)options {
    if (![scene isKindOfClass:UIWindowScene.class]) return;
    self.window = [[UIWindow alloc] initWithWindowScene:(UIWindowScene *)scene];
    self.window.rootViewController = [UIViewController new];
    self.window.rootViewController.view.backgroundColor = UIColor.blackColor;
    [self.window makeKeyAndVisible];
    NSLog(@"Chuzzle Compatibility scene connected");
}
- (void)sceneWillResignActive:(UIScene *)scene {
    UIApplication.sharedApplication.idleTimerDisabled = NO;
    ChuzzleFrameTimingSetActive(NO);
    id<UIApplicationDelegate> delegate = UIApplication.sharedApplication.delegate;
    if ([delegate respondsToSelector:@selector(applicationWillResignActive:)])
        [delegate applicationWillResignActive:UIApplication.sharedApplication];
}
- (void)sceneDidBecomeActive:(UIScene *)scene {
    ChuzzleFrameTimingSetActive(YES);
    id<UIApplicationDelegate> delegate = UIApplication.sharedApplication.delegate;
    if ([delegate respondsToSelector:@selector(applicationDidBecomeActive:)])
        [delegate applicationDidBecomeActive:UIApplication.sharedApplication];
    UIApplication.sharedApplication.idleTimerDisabled = !koiSleepExpired;
    if (koiSleepExpired) NSLog(@"Koi sleep timer: foreground retains Auto-Lock, idleTimerDisabled=%d", UIApplication.sharedApplication.idleTimerDisabled);
}
- (void)sceneDidEnterBackground:(UIScene *)scene {
    UIApplication.sharedApplication.idleTimerDisabled = NO;
    if (backgroundSave == UIBackgroundTaskInvalid) {
        backgroundSave = [UIApplication.sharedApplication beginBackgroundTaskWithName:@"Save Chuzzle"
            expirationHandler:^{ ChuzzleFinishBackgroundSave(); }];
    }
    id<UIApplicationDelegate> delegate = UIApplication.sharedApplication.delegate;
    if ([delegate respondsToSelector:@selector(applicationDidEnterBackground:)])
        [delegate applicationDidEnterBackground:UIApplication.sharedApplication];
}
- (void)sceneWillEnterForeground:(UIScene *)scene {
    id<UIApplicationDelegate> delegate = UIApplication.sharedApplication.delegate;
    if ([delegate respondsToSelector:@selector(applicationWillEnterForeground:)])
        [delegate applicationWillEnterForeground:UIApplication.sharedApplication];
}
@end

extern void ChuzzleConfigureFrameTiming(NSString *documents);

extern int chuzzle_ios_run(const char *ipa, const char *resources, const char *documents);

int SDL_main(int argc, char *argv[]) {
    @autoreleasepool {
        // Detailed frame metrics are opt-in for diagnostic launches.
        setenv("CHUZZLE_RUNTIME_DIAGNOSTICS", "0", 0);
        NSError *error = nil;
        AVAudioSession *audio = [AVAudioSession sharedInstance];
        [audio setCategory:AVAudioSessionCategoryAmbient error:&error];
        [audio setActive:YES error:&error];
        if (error) NSLog(@"Audio session: %@", error);
        NSString *resources = NSBundle.mainBundle.resourcePath;
        NSString *ipa = [resources stringByAppendingPathComponent:@"Koi-Pond.ipa"];
        NSString *documents = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES).firstObject;
        NSUserDefaults *settings = NSUserDefaults.standardUserDefaults;
        [settings registerDefaults:@{ @"AllowFullResolution": @YES, @"UnlockFrameRate": @YES, @"FillScreenCropped": @NO }];
        koiFillScreenCropped = [settings boolForKey:@"FillScreenCropped"];
        BOOL unlockFrameRate = [settings boolForKey:@"UnlockFrameRate"];
        // Diagnostic launches can retain the original cadence for comparison.
        const char *gameFPS = getenv("KOI_GAME_FPS");
        // Match ProMotion hardware; ordinary displays use the retimed 60 Hz variant.
        int defaultGameFPS = unlockFrameRate ? (UIScreen.mainScreen.maximumFramesPerSecond >= 120 ? 120 : 60) : 24;
        int requestedGameFPS = gameFPS ? atoi(gameFPS) : defaultGameFPS;
        if (requestedGameFPS != 24 && requestedGameFPS != 60 && requestedGameFPS != 120)
            requestedGameFPS = defaultGameFPS;
        setenv("KOI_GAME_FPS", [NSString stringWithFormat:@"%d", requestedGameFPS].UTF8String, 1);
        if (requestedGameFPS == 120) setenv("CHUZZLE_TARGET_FPS", "120", 0);
        ChuzzleConfigureFrameTiming(documents);
        if (requestedGameFPS == 60 || requestedGameFPS == 120) {
            ipa = [resources stringByAppendingPathComponent:
                [NSString stringWithFormat:@"Koi-Pond-%d.ipa", requestedGameFPS]];
        }
        NSLog(@"Koi Pond game target: %d FPS", requestedGameFPS);
        KoiObserveAudioInterruptions();
        Method sendEvent = class_getInstanceMethod(UIApplication.class, @selector(sendEvent:));
        koiOriginalSendEvent = (void *)method_setImplementation(sendEvent, (IMP)KoiSendEvent);
        Method present = class_getInstanceMethod(EAGLContext.class, @selector(presentRenderbuffer:));
        koiOriginalPresent = (void *)method_setImplementation(present, (IMP)KoiPresent);
        CGSize pixels = UIScreen.mainScreen.nativeBounds.size;
        // The game stays portrait even if the device was launched sideways.
        double width = MIN(pixels.width, pixels.height);
        double height = MAX(pixels.width, pixels.height);
        // Keep the original 2:3 viewport, with enough render pixels to cover
        // its physical screen area. A diagnostic launch can request 1x.
        BOOL allowFullResolution = [settings boolForKey:@"AllowFullResolution"];
        if (!getenv("CHUZZLE_RENDER_SCALE")) {
            unsigned renderScale = allowFullResolution ? ChuzzleRenderScale(koiFillScreenCropped ? fmax(width, height * 2.0 / 3.0) : width,
                koiFillScreenCropped ? fmax(height, width * 1.5) : height) : 1;
            NSString *scale = [NSString stringWithFormat:@"%u", renderScale];
            setenv("CHUZZLE_RENDER_SCALE", scale.UTF8String, 1);
        }
        NSLog(@"Koi Pond rendering: %dx%d (%sx), original 2:3 aspect",
              320 * atoi(getenv("CHUZZLE_RENDER_SCALE")),
              480 * atoi(getenv("CHUZZLE_RENDER_SCALE")), getenv("CHUZZLE_RENDER_SCALE"));
        if (strcmp(getenv("CHUZZLE_RUNTIME_DIAGNOSTICS"), "1") == 0) {
            NSDictionary *renderInfo = @{ @"nativeWidth": @(width), @"nativeHeight": @(height),
                @"screenScale": @(UIScreen.mainScreen.scale),
                @"allowFullResolution": @(allowFullResolution), @"unlockFrameRate": @(unlockFrameRate),
                @"gameFPS": @(requestedGameFPS), @"fillScreenCropped": @(koiFillScreenCropped),
                @"renderScale": @(atoi(getenv("CHUZZLE_RENDER_SCALE"))) };
            [renderInfo writeToFile:[documents stringByAppendingPathComponent:@"rendering-info.plist"] atomically:YES];
        }
        return chuzzle_ios_run(ipa.fileSystemRepresentation, resources.fileSystemRepresentation, documents.fileSystemRepresentation);
    }
}

// Keep the host's UIKit entry point explicit instead of depending on an
// entry point pulled indirectly from the Rust static library's SDL objects.
#undef main
int main(int argc, char *argv[]) {
    backgroundSave = UIBackgroundTaskInvalid;
    // Restore normal one-swipe Home behavior instead of SDL's fullscreen default.
    SDL_SetHintWithPriority(SDL_HINT_IOS_HIDE_HOME_INDICATOR, "0", SDL_HINT_OVERRIDE);
    if (@available(iOS 27.1,*)) {
        // Rotate the host window normally while the guest keeps portrait coordinates.
        SDL_SetHintWithPriority(SDL_HINT_ORIENTATIONS,
            "Portrait PortraitUpsideDown LandscapeLeft LandscapeRight", SDL_HINT_OVERRIDE);
    }
    return SDL_UIKitRunApp(argc, argv, SDL_main);
}
