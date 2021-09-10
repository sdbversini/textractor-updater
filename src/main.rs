use crate::updater::Updater;

mod updater;


fn main() {
    let mut updater = Updater::new();
    updater.update_and_run("x86");
}