extern crate capnpc;

fn main() {
    // Compile capnp schema source code
    capnpc::CompilerCommand::new()
        .file("src/spec/event.capnp")
        .output_path("")
        .run()
        .expect("schema compilation to succeed");
}
