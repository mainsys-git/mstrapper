use anyhow::Result;
use clap::Args;
use std::io::Error;
use std::process::{Command};

#[derive(Args)]
pub struct ViteArgs {
    pub name: String,

    #[arg(short, long)]
    pub framework: Option<String>,

    #[arg(long = "ts")]
    pub ts: bool,

    #[arg(long = "tw")]
    pub tailwind: bool,

    #[arg(long = "pm")]
    pub package_manager: Option<String>,
}

pub fn run(args: ViteArgs) -> Result<(), clap::Error> {
    let pm = pm_as_string(&args);
    let tw = args.tailwind; // Todo

    make_command(&args, pm)?;

    Ok(())
}

fn make_command(args: &ViteArgs, pm: (&str, Vec<&str>)) -> Result<(), Error> {
    let mut vite_command = Command::new(pm.0);
    vite_command.args(pm.1);
    vite_command.args([
        "create-vite@latest",
        &args.name,
        "--template",
        framework_as_str(&args),
        "--no-interactive",
    ]);

    vite_command.status()?;

    remove_boilerplate(&args)?;

    Ok(())
}

fn pm_as_string(args: &ViteArgs) -> (&str, Vec<&str>) {
    match args.package_manager.clone().unwrap_or("npx".to_string()).as_str() {
        "pnpm" => ("pnpm", vec!["dlx"]),
        "yarn" => ("yarn", vec!["dlx"]),
        "bunx" => ("bunx", vec![]),
        "npm" => ("npx", vec![]),
        _ => ("npx", vec![]),
    }
}

fn framework_as_str(args: &ViteArgs) -> &str {
    match (args.framework.clone().unwrap_or("vanilla".to_string()).as_str(), args.ts) {
        ("vanilla", true) => "vanilla-ts",
        ("vue", false) => "vue",
        ("vue", true) => "vue-ts",
        ("react", false) => "react",
        ("react", true) => "react-ts",
        ("preact", false) => "preact",
        ("preact", true) => "preact-ts",
        ("lit", false) => "lit",
        ("lit", true) => "lit-ts",
        ("svelte", false) => "svelte",
        ("svelte", true) => "svelte-ts",
        ("solid", false) => "solid",
        ("solid", true) => "solid-ts",
        ("qwik", false) => "qwik",
        ("qwik", true) => "qwik-ts",
        _ => "vanilla",
    }
}

fn remove_boilerplate(args: &ViteArgs) -> Result<(), Error> {
    std::env::set_current_dir(&args.name)?;
    if args.framework.clone().unwrap().as_str() == "react" {
        let to_remove = vec!["src/assets/react.svg", "src/App.css"];
        let to_clear = vec!["src/index.css"];


        for file in to_remove {
            std::fs::remove_file(file)?;
        }

        for file in to_clear {
            if file.contains(".tsx") {
                std::fs::write(file,
                               r#"function App() {

  return (
    <>

    </>
  )
}

export default App

                "#
                ).expect("TODO: panic message");
            } else {
                std::fs::write(file, r#""#).expect("");
            }
        }
    }

    install_script(&args)?;
    Ok(())
}

fn install_script(args: &ViteArgs) -> Result<(), Error> {
    let mut pm = pm_as_string(&args);
    let current_dir = std::env::current_dir()?;

    if pm.0 == "npx" {
        pm.0 = "npm"
    }

    Command::new(pm.0)
        .arg("install")
        .current_dir(current_dir)
        .status()?;

    Ok(())

}