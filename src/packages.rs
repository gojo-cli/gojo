use std::env;
use std::fs;
use std::io::Result;
use std::process::Command;
use std::process::Stdio;

pub fn install_gtest() -> Result<()> {
    #[allow(deprecated)]
    let tmp = env::home_dir().unwrap();
    let home = tmp.to_str().unwrap();

    if fs::exists(format!("{home}/repos/googletest"))? {
        println!("googletest is already installed");
        return Ok(());
    }

    let gtest_install_script = format!("{home}/.gojo/repos/packages/install/googletest.sh");
    Command::new("bash")
        .arg(gtest_install_script)
        .current_dir(format!("{home}"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    println!("gtest successfully installed");
    Ok(())
}
