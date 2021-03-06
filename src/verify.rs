use crate::util;
use console::{style, Emoji};
use indicatif::ProgressBar;
use std::fs;
use toml::Value;

pub fn verify(start_at: Option<&str>) -> Result<(), ()> {
    let toml: Value = fs::read_to_string("info.toml").unwrap().parse().unwrap();
    let tomlvec: &Vec<Value> = toml.get("exercises").unwrap().as_array().unwrap();
    let mut hit_start_at = false;

    for i in tomlvec {
        let path = i.get("path").unwrap().as_str().unwrap();

        if let Some(start_at) = start_at {
            if start_at.ends_with(path) {
                hit_start_at = true;
            } else if !hit_start_at {
                continue;
            }
        }

        match i.get("mode").unwrap().as_str().unwrap() {
            "test" => test(path)?,
            "compile" => compile_only(path)?,
            _ => (),
        }
    }
    Ok(())
}

fn compile_only(filename: &str) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", filename).as_str());
    progress_bar.enable_steady_tick(100);
    let compilecmd = util::compile_cmd(filename);
    progress_bar.finish_and_clear();
    if compilecmd.status.success() {
        let formatstr = format!(
            "{} Successfully compiled {}!",
            Emoji("✅", "✓"),
            filename
        );
        println!("{}", style(formatstr).green());
        util::clean();
        Ok(())
    } else {
        let formatstr = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
        util::clean();
        Err(())
    }
}

pub fn test(filename: &str) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {}...", filename).as_str());
    progress_bar.enable_steady_tick(100);
    let testcmd = util::compile_test_cmd(filename);
    if testcmd.status.success() {
        progress_bar.set_message(format!("Running {}...", filename).as_str());
        let runcmd = util::run_cmd();
        progress_bar.finish_and_clear();

        if runcmd.status.success() {
            let formatstr = format!("{} Successfully tested {}!", Emoji("✅", "✓"), filename);
            println!("{}", style(formatstr).green());
            util::clean();
            Ok(())
        } else {
            let formatstr = format!(
                "{} Testing of {} failed! Please try again. Here's the output:",
                Emoji("⚠️ ", "!"),
                filename
            );
            println!("{}", style(formatstr).red());
            println!("{}", String::from_utf8_lossy(&runcmd.stdout));
            util::clean();
            Err(())
        }
    } else {
        progress_bar.finish_and_clear();
        let formatstr = format!(
            "{} Compiling of {} failed! Please try again. Here's the output:",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&testcmd.stderr));
        util::clean();
        Err(())
    }
}
