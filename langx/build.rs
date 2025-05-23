fn main() {
    // Tell Cargo to re-run this build script if the parser grammar changes
    println!("cargo:rerun-if-changed=src/parser/grammar.lalrpop");
    
    // Use LALRPOP to generate the parser
    lalrpop::process_root().unwrap();
} 