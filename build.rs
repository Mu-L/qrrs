use clap::{App, IntoApp, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;

use roff::{line_break, roman, Roff};

use std::{env, error::Error, path::PathBuf};

include!("src/cli.rs");

type Res = Result<(), Box<dyn Error>>;

fn main() -> Res {
    let mut cli = Arguments::command();

    let profile = env::var("PROFILE")?;
    match profile.as_str() {
        "release" => {
            let outdir_c = PathBuf::from("./completions");
            let outdir_m = PathBuf::from("./man");

            generate_completions(&mut cli, &outdir_c)?;
            generate_manpage(cli, &outdir_m)?;
        }
        // "debug" => {
        //     let outdir = match env::var_os("OUT_DIR") {
        //         None => return Ok(()),
        //         Some(outdir) => PathBuf::from(outdir),
        //     };

        //     generate_completions(&mut cli, &outdir)?;
        //     generate_manpage(cli, &outdir)?;
        // }
        _ => (),
    }

    Ok(())
}

fn generate_completions(cli: &mut App, outdir: &PathBuf) -> Res {
    ["bash", "zsh", "fish", "powershell"]
        .iter()
        .map(|sh| Shell::from_str(sh, true))
        .filter_map(|sh| sh.ok())
        .for_each(|sh| {
            let path = generate_to(sh, cli, "qrrs", &outdir);

            println!(
                "cargo:warning=completion file for {:?} is generated: {:?}",
                sh, path
            );
        });

    Ok(())
}

fn generate_manpage(cli: App, outdir: &PathBuf) -> Res {
    let mut buffer: Vec<u8> = Default::default();

    let man_file = outdir.join("qrrs.1");
    let man = Man::new(cli);
    man.render(&mut buffer)?;
    Roff::new()
        .control("SH", ["EXAMPLES OF USAGE"])
        .text([
            roman("qrrs \"Your input here\""),
            line_break(),
            roman("qrrs \"Something\" /tmp/qr.png "),
            line_break(),
            roman("qrrs -r /tmp/qr.png "),
            line_break(),
        ])
        .to_writer(&mut buffer)?;

    std::fs::write(&man_file, buffer)?;
    println!("cargo:warning=manpage file is generated: {:?}", man_file);

    Ok(())
}
