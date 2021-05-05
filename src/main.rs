use clap::Clap;

#[derive(Clap, Debug)]
struct Args {
    input: String,
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input_file_name=args.input;
    let output_file_name=args.output.unwrap_or(format!("{}.png",input_file_name));
    println!("Hello, world!\n{:?}\n{:?}",input_file_name,output_file_name);
}
