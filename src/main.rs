use std::env;
use std::fs::File;
use std::process::Command;
use std::io::Write;

fn help() {
    // Prints a help message that describes application
    println!("nannou-cli");
    println!("----------");
    println!("a command line application to maintain nannou workspaces");
    println!(" ");
    println!("USAGE:");
    println!("    nannou <COMMAND>");
    println!(" ");
    println!("COMMANDS:");
    println!("    run <SKETCHNAME>     Builds and runs a sketch");
    println!("    update               Updates Cargo.toml with all sketches in folder");
    // TO DO: maybe look into a way that doesn't require
    // updating the formatting by hand...
}

fn run (sketchname: String) {
    println!("Running {}", sketchname);

    // let test_command = format!("echo Running {}", sketchname);

            // dbg!(echo1.stdout);


    // println!("Shell returns: {}", echo1.stdout.to_string());

}

//-> std::io::Result<()>
fn update () -> std::io::Result<()>{
    // GENERATE CARGO.TOML

    let mut file = File::create("foo.txt")?;

    let ls_dir = Command::new("sh")
            .arg("-c")
            .arg("ls -d */")
            .output()
            .expect("failed to execute run process");

    let mut members = String::from_utf8( ls_dir.stdout )
                          .unwrap()
                          .replace("/\n","\n");

    println!("Writing the following directories to Cargo.toml:");
    println!("{}", members);

    members = members.replace("\n",",\n");

    let contents = format!("[workspace]\n\nmembers = [\n{}]", members);

    file.write_all(contents.as_bytes())?;

    Ok(())

}

fn main() {

    // Collect command arguments
    // and store in a Vector for Strings
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "run" {
            // if command is run, ensure there is a sketchname provided.
            if args.len() < 3 {
                println!("This command requires a sketchname.")
            } else {
                let sketchname = args[2].to_string();
                run(sketchname);
            }
        }
        else if args[1] == "update" {
            update().ok();
        }
        else {
            help();
        }
    } else {
        help();
    }

}
