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

    // let gtest_package_url = "https://gojo-cli.github.io/packages/gtest.sh";
    let gtest_install_script = format!("{home}/.gojo/repos/packages/packages/gtest.sh");
    Command::new("bash")
        .arg(gtest_install_script)
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
    
    //let output = source.wait_with_output().unwrap();
    //let result = std::str::from_utf8(&output.stdout).unwrap();
    //println!("{result}");
    println!("gtest successfully installed");
    Ok(())
}
