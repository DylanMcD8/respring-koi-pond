<p align="center">
  <img src="docs/images/KoiPondRespringHeader.webp" width="800" alt="Respring – Koi Pond">
</p>

# Respring – Koi Pond
The original Koi Pond, restored for modern iPhone/iPad devices as part of Respring.

### How does it work?
This build runs the original Koi Pond game through ahead-of-time ARM/Thumb-to-ARM64 translation. A modified touchHLE runtime supplies the legacy iOS APIs, with native iOS presentation and audio handling. No JIT is required, and the shipping configuration has no CPU interpreter fallback.

### Enhancements
- Full Resolution: render the pond at display resolution while retaining the original 3.5-inch iPhone aspect ratio.
- Unlock Frame Rate: choose the original 24 FPS behavior or rendering up to 60 FPS, or 120 FPS on ProMotion displays.
- Fill Screen (Cropped): optionally enlarge the original play area to fill the display.
- Modern lifecycle: audio resumes after interruptions, and the app stays awake during use. The sleep timer allows normal Auto-Lock instead of closing the app.
- App Icon: an editable Icon Composer icon with modern appearances.
- Duo support: the native window adapts to rotation and display changes on iOS 27.1 and later.

### Notes
- Change the enhancements in Settings → Apps → Koi Pond, then force quit and reopen the app.
- Cropped fill can cut off content; it does not change the original pond layout.
- Full-resolution rendering does not replace the original low-resolution artwork.
- The sleep timer follows system Auto-Lock settings; it cannot force a device lock or override Auto-Lock set to Never.
- If you encounter a crash or another problem, please open an Issue on this repository.

This restoration and its technical documentation were developed with help from generative AI.

## How to Install
Download **Koi-Pond.ipa** from [Releases](https://github.com/DylanMcD8/respring-koi-pond/releases). It is an **unsigned ARM64 device IPA**: use your own signing/sideloading workflow to sign it for your device. [Sideloadly](https://sideloadly.io) is one option.

Alternatively, build and run **KoiPond.xcodeproj** with your own Apple development team.

The deployment target is iOS/iPadOS 15.0. Earlier testing covered iPhone Air, iPhone 18 Pro, iPad Pro 13-inch (M4), and the iPhone Duo simulator. Those tests do not establish compatibility with every older OS version. The simulator build targets Apple silicon Macs.

## Build from Source
*Written by AI agent*

Use an Apple silicon Mac with full Xcode, Python 3, Rust 1.99.0, and CMake. Xcode 27.1 and CMake 4.3.5 were used for the completed restoration builds. Rust dependencies and the modified SDL2 sources are included.

1. Install the toolchain and ARM64 targets described in [BUILDING.md](BUILDING.md).
2. Open `KoiPond.xcodeproj`, select the **KoiPond** scheme and your destination.
3. For a physical device, select your own development team and an available bundle identifier.
4. Build and run. The first build compiles the runtime and generated game translations and can take a while.

See [BUILDING.md](BUILDING.md) for command-line builds, packaging, tests, and source layout.

## Credits and Licensing
Koi Pond and its original game content belong to their respective rights holders. The original executable, artwork, audio, and game-derived material are not relicensed by this project's open-source notices. This is an unofficial preservation project with no claim of rights-holder affiliation or endorsement.

The compatibility runtime derives from [touchHLE](https://github.com/touchHLE/touchHLE), with MPL-2.0 notices preserved. SDL2, Rust dependencies, fonts, and bundled runtime libraries retain their own licenses.

See [LICENSE.md](LICENSE.md), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), and the notices beside the sources. Matching project and runtime source downloads accompany the IPA.
