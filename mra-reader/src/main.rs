use std::env;
use std::fs::File;
use std::io::prelude::*;

mod mra;

fn print_phf(class: &mra::MraClass) {
    println!(
        "pub static {}: phf::Map<u8, &'static str> = phf_map! {{",
        class.short_name
    );
    for prop in &class.properties {
        println!("    {}u8 => \"{}\",", prop.epc, prop.name.ja);
    }
    println!("}};");
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage:\nmra-reader MRA_DATA_PATH");
        anyhow::bail!("invalid command line argument");
    }

    let filename = &args[1];
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let class: mra::MraClass = serde_json::from_str(&contents)?;
    print_phf(&class);

    Ok(())
}
