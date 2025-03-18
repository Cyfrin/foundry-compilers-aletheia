use crate::Result;
use foundry_compilers::solc::Solc;
use itertools::Itertools;
use semver::VersionReq;
use serde::Serialize;
use std::{
    io::{BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
    sync::LazyLock,
};

// `--base-path` was introduced in 0.6.9 <https://github.com/ethereum/solidity/releases/tag/v0.6.9>
static SUPPORTS_BASE_PATH: LazyLock<VersionReq> =
    LazyLock::new(|| VersionReq::parse(">=0.6.9").unwrap());

// `--include-path` was introduced in 0.8.8 <https://github.com/ethereum/solidity/releases/tag/v0.8.8>
static SUPPORTS_INCLUDE_PATH: LazyLock<VersionReq> =
    LazyLock::new(|| VersionReq::parse(">=0.8.8").unwrap());

fn configure_cmd(solc: &Solc, current_dir: &Path) -> Command {
    let mut cmd = Command::new(&solc.solc);
    cmd.stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped());
    cmd.args(&solc.extra_args);

    if !solc.allow_paths.is_empty() {
        cmd.arg("--allow-paths");
        cmd.arg(solc.allow_paths.iter().map(|p| p.display()).join(","));
    }
    if let Some(base_path) = &solc.base_path {
        if SUPPORTS_BASE_PATH.matches(&solc.version) {
            if SUPPORTS_INCLUDE_PATH.matches(&solc.version) {
                // `--base-path` and `--include-path` conflict if set to the same path, so
                // as a precaution, we ensure here that the `--base-path` is not also used
                // for `--include-path`
                for path in solc.include_paths.iter().filter(|p| p.as_path() != base_path.as_path())
                {
                    cmd.arg("--include-path").arg(path);
                }
            }

            cmd.arg("--base-path").arg(base_path);
        }
    }

    // NOTE: I moved this below line outside the above if-stmnt from the original code
    // because Sablier wasn't working when --base-path was passed
    // Original version: [`Solc::compile_output`]
    cmd.current_dir(current_dir);

    cmd.arg("--standard-json");

    cmd
}

pub fn compile_output<T: Serialize>(solc: &Solc, current_dir: &Path, input: &T) -> Result<Vec<u8>> {
    let mut cmd = configure_cmd(solc, current_dir);
    let mut child = cmd.spawn()?;

    {
        let mut stdin = BufWriter::new(child.stdin.take().unwrap());
        serde_json::to_writer(&mut stdin, input)?;
        stdin.flush()?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(String::from_utf8_lossy(output.stderr.as_ref()).into())
    }
}
