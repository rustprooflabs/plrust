use std::path::PathBuf;

#[test]
fn uitests() {
    let plrustc = env!("CARGO_BIN_EXE_plrustc");
    let root = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").into());
    let bless = std::env::var_os("PLRUSTC_BLESS").is_some();

    compiletest_rs::run_tests(&compiletest_rs::Config {
        mode: compiletest_rs::common::Mode::Ui,
        edition: Some("2021".into()),
        rustc_path: std::path::PathBuf::from(&plrustc),
        bless,
        src_base: root.join("uitests"),
        target_rustcflags: Some("--emit=metadata -Fplrust_lints -Dwarnings -Zui-testing".into()),

        ..compiletest_rs::Config::default()
    });
}
