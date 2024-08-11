use clap::Parser;
use integration::{prove::verify_chunk, test_util::load_chunk};
use prover::{utils::init_env_and_log, ChunkProvingTask};
use std::env;
use std::{error::Error, result::Result};
use gevulot_shim::{Task, TaskResult};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Get params dir path.
    #[clap(short, long = "params", default_value = "params")]
    params_path: String,
    /// Get asserts dir path.
    #[clap(short, long = "assets", default_value = "test_assets")]
    proof_file_path: String,
    /// Get BlockTrace from file or dir.
    #[clap(
        short,
        long = "trace",
        default_value = "tests/extra_traces/batch_34700/chunk_1236462/block_4176564.json"
    )]
    trace_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    gevulot_shim::run(run_task)
}

// The main function that executes the prover program.
fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    // Display program arguments we received. These could be used for
    // e.g. parsing CLI arguments with clap.
    println!("prover: task.args: {:?}", &task.args);

    // -----------------------------------------------------------------------
    // Here would be the control logic to run the prover with given arguments.
    // -----------------------------------------------------------------------
    // Layer config files are located in `./integration/configs`.
    env::set_current_dir("./integration").unwrap();
    let output_dir = init_env_and_log("trace_prover");
    log::info!("Initialized ENV and created output-dir {output_dir}");
    
    //let args = Args::parse_from(task.args);//Args::parse();
    // `task.args` contains only the task args, which doesn't include the binary
    // name. To use existing CLI args parser, create a Vec<String> of args,
    // including the binary name.
    let mut args_with_bin_name = vec![std::env::args()
    .collect::<Vec<String>>()
    .first()
    .unwrap()
    .clone()];
    args_with_bin_name.append(&mut task.args.clone());

    // Parse the cli args.
    let args = Args::parse_from(args_with_bin_name);
    
    let traces = load_chunk(&args.trace_path).1;
    prover::eth_types::constants::set_scroll_block_constants_with_trace(&traces[0]);
    let chunk = ChunkProvingTask::from(traces);

    verify_chunk(// same with `make test-chunk-prove`, to load vk
        &args.params_path,
        &args.proof_file_path,
        &output_dir,
    );
    log::info!("chunk verify done");

    // Write generated proof to a file.
    std::fs::write("/workspace/verify.dat", b"this is a verify of proof.")?;

    // Return TaskResult with reference to the generated proof file.
    task.result(vec![], vec![String::from("/workspace/verify.dat")])
}


