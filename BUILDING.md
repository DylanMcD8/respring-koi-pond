# Building Koi Pond

Use full Xcode selected with `xcode-select`, Python 3, Rust installed through rustup, and CMake on PATH. The supported build target here is ARM64 iOS or an Apple silicon iOS simulator; this package is not a desktop or Android build recipe.

```sh
rustup toolchain install 1.99.0 --profile minimal
rustup target add --toolchain 1.99.0 aarch64-apple-ios aarch64-apple-ios-sim
```

The initial toolchain installation needs network access. Cargo sources and modified SDL2 are vendored; the build uses `--offline --locked`. No sibling Respring/Chuzzle project, private cache, or saved signing team is required. SDL2 is compiled with CMake for each target on its first build.

## Xcode

Open `KoiPond.xcodeproj`, choose the KoiPond scheme, and select a destination. For a device, set your own development team and an available bundle identifier. Xcode processes `OriginalPort/ios/Info.xcode.plist`, expanding the selected identifier/executable and retaining icon metadata. The copy phase excludes the final root Info.plist to avoid competing with Xcode's plist processor.

```sh
xcodebuild -project KoiPond.xcodeproj -scheme KoiPond \
  -configuration Release -destination 'generic/platform=iOS Simulator' \
  -derivedDataPath /tmp/KoiPond-sim CODE_SIGNING_ALLOWED=NO

xcodebuild -project KoiPond.xcodeproj -scheme KoiPond \
  -configuration Release -destination 'generic/platform=iOS' \
  -derivedDataPath /tmp/KoiPond-device CODE_SIGNING_ALLOWED=NO
```

Leave signing enabled and choose your team when using Run on a physical device. Both Debug and Release use optimized Rust; unoptimized translation builds are too slow for normal use.

## Direct build and unsigned package

```sh
CHUZZLE_NATIVE_BUILD=/tmp/KoiPond-build bash OriginalPort/build.sh iphoneos
python3 Scripts/package_ipa.py /tmp/KoiPond-build/iphoneos/KoiPond.app \
  --output release-output/Koi-Pond.ipa
python3 Scripts/audit_release.py --ipa release-output/Koi-Pond.ipa
```

Use `iphonesimulator` for a simulator app, which cannot be distributed as a device IPA. The package script stages a copy, strips signing/provisioning and account metadata, sanitizes any nested original IPAs, and leaves the signed input app unchanged. The result needs the recipient's signing method.

Build outside iCloud/File Provider-managed output directories; Xcode's DerivedData or `/tmp` works well. Legacy internal symbols and environment variables may retain the `CHUZZLE_` prefix because these ports share runtime ancestry.

## Source and execution

- `OriginalPort/ios`: native host, frame pacing, Rust entry point, app metadata and Settings bundle.
- `OriginalPort/compatibility-core`: this app's modified touchHLE runtime and instruction handling.
- `Original`: sanitized original app payload plus provenance and the exact executable/icon used by the translation generator.
- `Koi Pond.icon`: editable Icon Composer document.
- `Vendor`: pinned Cargo sources and the modified SDL2 source.
- `Notices`: dependency licenses copied into built apps.

Execution: **static ARM/Thumb-to-ARM64 translation; interpreter fallback disabled**. `CHUZZLE_AOT_REFERENCE=1` is only for development comparison; shipping builds default to static translation without that fallback.

Koi Pond builds default to Rust optimization level 1, matching the completed build 15 configuration. Override CARGO_PROFILE_RELEASE_OPT_LEVEL only for a deliberate rebuild. Its 60/120 FPS IPA variants are generated from guarded data-only patches in patch_framerate.py. The guarded sleep/input adaptations are reproduced by generate_native.py.

## Regression checks

Run with the pinned Rust toolchain available on PATH:

```sh
python3 OriginalPort/Tests/test_framerate_patch.py
python3 OriginalPort/Tests/test_touch_timing.py
python3 OriginalPort/Tests/test_sleep_timer.py
python3 OriginalPort/Tests/test_sleep_host.py
```

The included tests cover timer/input behavior and guarded frame-rate patches; hardware Auto-Lock timing remains controlled by iOS settings. Device performance and original gameplay require testing on the intended hardware. Simulator performance is not representative.

Original game content remains outside the runtime's open-source license grant. The supplied original payload has account/purchase and signing sidecar metadata removed; it is still original game content. See LICENSE.md and THIRD_PARTY_NOTICES.md.
