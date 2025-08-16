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

    let gtest_package_url = "https://gojo-cli.github.io/packages/gtest.sh";
    let curl = Command::new("bash")
        .arg(format!("<(curl --proto '=https' --tlsv1.2 -sSf {gtest_package_url}"))
        .current_dir(format!("{home}"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

        /*
    let source = Command::new("source")
        .args(["-s", "--", "-y"])
        .stdin(Stdio::from(curl.stdout.unwrap()))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();*/
    
    let output = source.wait_with_output().unwrap();
    let result = std::str::from_utf8(&output.stdout).unwrap();
    println!("{result}");
    println!("gtest successfully installed");
    Ok(())
}
