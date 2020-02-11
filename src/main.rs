use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use log::info;
use env_logger;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    env_logger::init();
    
    info!("args given: pattern: {} file: {:#?}", args.pattern, args.file);

    let content = std::fs::read_to_string(&args.file)
       .with_context(|_| format!("Could not read file {:#?}", &args.file))?;
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}

