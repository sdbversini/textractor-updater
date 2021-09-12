use crate::updater::Updater;
use clap::App;
use clap::Arg;

mod updater;

fn main() {
    let clap_matches = App::new("Textractor-Updater")
        .version("0.5")
        .author("Bastien V")
        .about("Updates Textractor and launches it")
        .arg(
            Arg::with_name("architecture")
                .short("a")
                .long("arch")
                .value_name("ARCH")
                .help(r#"Sets architecture of the executable to launch. Possibles values are "x86", "86", "x64" and "64""#)
                .required(true),
        ).get_matches();

    let mut updater = Updater::new();
    if let Some(arch) = clap_matches.value_of("architecture") {
        match arch {
            "x86" | "86" => updater.update_and_run("x86"),
            "x64" | "64" => updater.update_and_run("x64"),
            _ => {
                eprintln!("Improper argument, falling back to x86");
                updater.update_and_run("x86")
            }
        }
    }
}
