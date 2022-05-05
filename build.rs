use std::path::Path;

fn main() {
    let mut cc = cc::Build::new();
    let jp2dir = Path::new("vendor/src/lib/openjp2");

    let target = std::env::var("CARGO_CFG_TARGET_FAMILY").expect("CARGO_CFG_TARGET_FAMILY");
    if target == "windows" {
        cc.define("OPJ_HAVE__ALIGNED_MALLOC", Some("1"));
        #[cfg(feature = "threads")]
        cc.define("MUTEX_win32", Some("1"));
    } else if target == "wasm" {
        let wasi_sdk_path = std::env::var("WASI_SDK_PATH").expect("WASI_SDK_PATH not set");
        cc.flag(&format!("--sysroot={}/share/wasi-sysroot/", wasi_sdk_path);
        cc.define("_WASI_EMULATED_PROCESS_CLOCKS", Some("1"));
        cc.define("CFLAGS", Some("-lwasi-emulated-process-clocks"));
    } else {
        cc.define("OPJ_HAVE_POSIX_MEMALIGN", Some("1"));
        #[cfg(feature = "threads")]
        cc.define("MUTEX_pthread", Some("1"));
    }
    cc.include(jp2dir);
    cc.include("config");
    cc.define("OPJ_STATIC", Some("1"));
    #[cfg(not(feature = "threads"))]
    cc.define("MUTEX_stub", Some("1"));

    let files = [
        "thread.c",
        "bio.c",
        "cio.c",
        "dwt.c",
        "event.c",
        "ht_dec.c",
        "image.c",
        "invert.c",
        "j2k.c",
        "jp2.c",
        "mct.c",
        "mqc.c",
        "openjpeg.c",
        "opj_clock.c",
        "pi.c",
        "t1.c",
        "t2.c",
        "tcd.c",
        "tgt.c",
        "function_list.c",
        "opj_malloc.c",
        "sparse_array.c",
    ];
    for file in files.iter() {
        cc.file(jp2dir.join(file));
    }
    cc.compile("openjp2");
}
