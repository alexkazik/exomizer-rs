use exomizer::original::{
    CrunchOptions, Crunched, DecrunchOptions, Level, Logger, Output, crunch_raw, decrunch_raw,
};
use exomizer::simple::dynamic::DynProto;
use exomizer::simple::generic::GenProto;
use std::env::args;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Instant;

// change this values for other tests
const TEST_FAST: &[bool] = &[false, true];
const TEST_PROTO: &[u8] = &[61];
const TEST_TRAITS: &[u8] = &[0];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut log = Logger::stdout();
    log.level(Level::Error);

    #[cfg(debug_assertions)]
    eprintln!("It's recommended to run in release mode.");

    if args().len() != 2 {
        panic!("Usage: {} </path/to/exomizer>", args().next().unwrap());
    }
    let exomizer_exe = args().nth(1).unwrap();

    for file in fs::read_dir("examples/demo")? {
        let file = file?;
        if file.file_type()?.is_dir() || file.file_name().to_string_lossy().contains(".exo") {
            continue;
        }
        for fast in TEST_FAST.iter().copied() {
            for proto in TEST_PROTO.iter().copied() {
                for traits in TEST_TRAITS.iter().copied() {
                    eprintln!();
                    eprintln!(
                        "# Testing {} with {}proto {proto}, traits {traits}",
                        file.path().display(),
                        if fast { "fast, " } else { "" }
                    );
                    let mut exo_filename = file.path().clone();
                    let base_name = exo_filename
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    exo_filename.pop();
                    exo_filename.push(format!(
                        "{}.exoP{proto}T{traits}{}",
                        base_name,
                        if fast { "C" } else { "" }
                    ));
                    if !fs::exists(&exo_filename)?
                        || fs::metadata(&exo_filename)?.modified()? < file.metadata()?.modified()?
                    {
                        // compress using the "regular" exomizer
                        let mut command = Command::new(&exomizer_exe);
                        command.stdin(Stdio::null());
                        command.arg("raw");
                        command.arg("-b");
                        command.arg("-r");
                        if fast {
                            command.arg("-C");
                        }
                        command.arg("-P");
                        command.arg(proto.to_string());
                        command.arg("-T");
                        command.arg(traits.to_string());
                        command.arg("-o");
                        command.arg(&exo_filename);
                        command.arg(file.path());
                        if !command.status()?.success() {
                            panic!("failed to execute exomizer");
                        }
                    }
                    let demo_uncompressed = fs::read(file.path())?;
                    let demo_compressed = fs::read(&exo_filename)?;

                    check_crunch(
                        &mut log,
                        &demo_uncompressed,
                        &demo_compressed,
                        fast,
                        proto,
                        traits,
                    );

                    check_decrunch(&mut log, &demo_compressed, &demo_uncompressed, proto);

                    check_decrunch_simple_dyn(&demo_compressed, &demo_uncompressed, proto);

                    check_decrunch_simple_gen(&demo_compressed, &demo_uncompressed, proto);

                    eprintln!("DONE.");
                }
            }
        }
    }

    Ok(())
}

fn check_crunch<O: Output>(
    log: &mut O,
    demo_uncompressed: &[u8],
    demo_compressed: &[u8],
    fast: bool,
    proto_num: u8,
    traits: u8,
) {
    let crunch_options = CrunchOptions::builder()
        .flags_proto(proto_num)
        .flags_notrait_num(traits)
        .favor_speed(fast)
        .direction_forward(false)
        .write_reverse(true)
        .build()
        .unwrap();

    let start = Instant::now();
    let Crunched {
        output,
        enc_bytes,
        enc_string,
        crunch_info,
    } = unsafe { crunch_raw(log, demo_uncompressed, None, &crunch_options) };
    let duration = start.elapsed();

    println!("Time elapsed in crunch() is: {:?}", duration);
    eprintln!("Encoding: {enc_string} == {enc_bytes:?}");

    if output != demo_compressed {
        eprintln!("Error in compression: the C and Rust exomizer differ in output!");
        eprintln!();
        eprintln!("CrunchOptions: {crunch_options:#?}");
        eprintln!("CrunchInfo: {crunch_info:#?}");
        eprintln!("Encoding: {enc_bytes:?}");
        panic!();
    }
}

fn check_decrunch<O: Output>(
    log: &mut O,
    demo_compressed: &[u8],
    demo_uncompressed: &[u8],
    proto_num: u8,
) {
    let decrunch_options = DecrunchOptions::builder()
        .flags_proto(proto_num)
        .direction_forward(true)
        .write_reverse(true)
        .build()
        .unwrap();

    let start = Instant::now();
    let output = unsafe { decrunch_raw(log, demo_compressed, &decrunch_options) };
    let duration = start.elapsed();

    if output != demo_uncompressed {
        eprintln!("Error in decompression: the C and Rust exomizer differ in output!");
        eprintln!();
        eprintln!("DecrunchOptions: {decrunch_options:#?}");
        panic!();
    }

    println!("Time elapsed in decrunch() is: {:?}", duration);
}

fn check_decrunch_simple_dyn(demo_compressed: &[u8], demo_uncompressed: &[u8], proto: u8) {
    let Ok(proto) = DynProto::from_num(proto) else {
        return;
    };

    let start = Instant::now();
    let mut vec = vec![0; demo_uncompressed.len()];
    let result = exomizer::simple::dynamic::decrunch_exact(
        proto,
        demo_compressed.iter().copied(),
        vec.as_mut_slice(),
    );
    let duration = start.elapsed();

    if let Err(err) = result {
        eprintln!("Error in dyn simple decompression: the C and Rust exomizer differ in output!");
        eprintln!();
        eprintln!("The decompressor did return {err}");
        panic!();
    }

    if vec != demo_uncompressed {
        eprintln!("Error in dyn simple decompression: the C and Rust exomizer differ in output!");
        eprintln!();
        eprintln!("Proto: {}", proto.to_num());
        fs::write("/tmp/simple.de.fail", vec).unwrap();
        panic!();
    }

    println!("Time elapsed in dyn_simple_decrunch() is: {:?}", duration);
}

fn check_decrunch_simple_gen(demo_compressed: &[u8], demo_uncompressed: &[u8], proto: u8) {
    match proto {
        9 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P9, proto),
        13 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P13, proto),
        25 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P25, proto),
        29 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P29, proto),
        41 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P41, proto),
        45 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P45, proto),
        57 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P57, proto),
        61 => check_decrunch_simple_gen2(demo_compressed, demo_uncompressed, GenProto::P61, proto),
        _ => (),
    }
}

fn check_decrunch_simple_gen2<const P: u8>(
    demo_compressed: &[u8],
    demo_uncompressed: &[u8],
    proto: GenProto<P>,
    proto_num: u8,
) {
    let start = Instant::now();
    let mut vec = vec![0; demo_uncompressed.len()];
    let result = exomizer::simple::generic::decrunch_exact(
        proto,
        demo_compressed.iter().copied(),
        vec.as_mut_slice(),
    );
    let duration = start.elapsed();

    if let Err(err) = result {
        eprintln!("Error in gen simple decompression: the C and Rust exomizer differ in output!");
        eprintln!();
        eprintln!("The decompressor did return {err}");
        panic!();
    }

    if vec != demo_uncompressed {
        eprintln!("Error in gen simple decompression: the C and Rust exomizer differ in output!");
        eprintln!();
        eprintln!("Proto: {proto_num}");
        fs::write("/tmp/simple.de.fail", vec).unwrap();
        panic!();
    }

    println!("Time elapsed in gen_simple_decrunch() is: {:?}", duration);
}
