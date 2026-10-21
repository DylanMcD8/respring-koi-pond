use std::ffi::{c_char, CStr};

// Original ARM/Thumb code is compiled ahead of time. The compatibility
// runtime preserves the original data layout and provides old iOS APIs.
#[no_mangle]
pub unsafe extern "C" fn chuzzle_ios_run(
    ipa: *const c_char,
    resources: *const c_char,
    documents: *const c_char,
) -> i32 {
    if ipa.is_null() || resources.is_null() || documents.is_null() {
        return 1;
    }
    let outcome = std::panic::catch_unwind(|| {
        let ipa = CStr::from_ptr(ipa).to_str().map_err(|e| e.to_string())?;
        let resources = CStr::from_ptr(resources).to_str().map_err(|e| e.to_string())?;
        let documents = CStr::from_ptr(documents).to_str().map_err(|e| e.to_string())?;
        std::fs::create_dir_all(documents).map_err(|e| e.to_string())?;
        std::env::set_var("TOUCHHLE_RESOURCE_DIR", resources);
        std::env::set_var("TOUCHHLE_USER_DATA_DIR", documents);
        std::env::set_current_dir(documents).map_err(|e| e.to_string())?;
        let mut arguments = vec![
            "Chuzzle Compatibility".to_owned(),
            ipa.to_owned(),
            "--fullscreen".to_owned(),
            format!("--scale-hack={}", std::env::var("CHUZZLE_RENDER_SCALE")
                .ok().and_then(|s| s.parse::<u32>().ok())
                .filter(|scale| (1..=8).contains(scale)).unwrap_or(1)),
            "--no-error-popup".to_owned(),
        ];
        let fps = std::env::var("CHUZZLE_TARGET_FPS").unwrap_or_else(|_| "60".to_owned());
        arguments.push(format!("--fps-limit={fps}"));
        if std::env::var("CHUZZLE_RUNTIME_DIAGNOSTICS").as_deref() == Ok("1") {
            arguments.push("--print-fps".to_owned());
        }
        touchHLE::main(arguments.into_iter())
    });
    match outcome {
        Ok(Ok(())) => 0,
        Ok(Err(error)) => { eprintln!("Chuzzle Compatibility: {error}"); 1 }
        Err(_) => { eprintln!("Chuzzle Compatibility: emulator panic; see Documents/touchHLE_log.txt"); 2 }
    }
}
