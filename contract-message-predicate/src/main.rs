use fuel_tx::{Address, Bytes32};
use std::fs;
use std::path::Path;

const OUTPUT_DIR: &str = "./out";
const SCRIPT_BUILD_PATH: &str = "./out/contract_message_script.bin";
const PREDICATE_BUILD_PATH: &str = "./out/contract_message_predicate.bin";

fn main() {
    //get predicate and script bytecode
    let script = contract_message_predicate::script_bytecode();
    let predicate = contract_message_predicate::predicate_bytecode();

    //output to console and build files
    let script_hash = contract_message_predicate::script_hash();
    let predicate_root = contract_message_predicate::predicate_root();
    println!("Script bytecode size is {} bytes.", script.len());
    println!("Script hash: 0x{}", Bytes32::from(script_hash));
    println!("Predicate bytecode size is {} bytes.", predicate.len());
    println!("Predicate root: 0x{}", Address::from(predicate_root));
    fs::create_dir_all(OUTPUT_DIR).expect(&format!(
        "Failed to create output directory [{}].",
        OUTPUT_DIR
    ));
    fs::write(Path::new(SCRIPT_BUILD_PATH), script).expect(&format!(
        "Failed to wite to script binary file output [{}].",
        SCRIPT_BUILD_PATH
    ));
    fs::write(Path::new(PREDICATE_BUILD_PATH), predicate).expect(&format!(
        "Failed to wite to predicate binary file output [{}].",
        PREDICATE_BUILD_PATH
    ));
}
