#![cfg(feature = "alloc")]

use crate::common::{DEMO_COMPRESSED_P61_FAST, DEMO_UNCOMPRESSED};
use exomizer::original::{CrunchOptions, Output, crunch_raw};
use std::fmt::Arguments;

mod common;

#[test]
fn original_crunch() {
    let mut output = CaptureOutput::new();
    let crunched = unsafe {
        crunch_raw(
            &mut output,
            DEMO_UNCOMPRESSED,
            None,
            &CrunchOptions::builder()
                .flags_proto(61)
                .direction_forward(false)
                .favor_speed(true)
                .write_reverse(true)
                .build()
                .unwrap(),
        )
    };
    assert_eq!(DEMO_COMPRESSED_P61_FAST, &crunched.output);
    assert_eq!(CAPTURED_OUTPUT, &output.output());
}

struct CaptureOutput {
    output: Vec<String>,
    bar: Option<(Vec<char>, usize)>,
}
impl CaptureOutput {
    fn new() -> CaptureOutput {
        CaptureOutput {
            output: Vec::new(),
            bar: None,
        }
    }

    fn output(&self) -> Vec<&str> {
        if self.bar.is_some() {
            panic!("CaptureOutput: bar not closed")
        }
        self.output.iter().map(|x| x.as_str()).collect()
    }
}

impl Output for CaptureOutput {
    fn log_dump(&mut self, _args: Arguments) {}

    fn log_dump_ln(&mut self, _args: Arguments) {}

    fn log_debug(&mut self, _args: Arguments) {}

    fn log_debug_ln(&mut self, _args: Arguments) {}

    fn log_normal_ln(&mut self, args: Arguments) {
        self.output.push(format!("{args}"));
    }

    fn log_brief_ln(&mut self, args: Arguments) {
        self.output.push(format!("{args}"));
    }

    fn log_error_ln(&mut self, args: Arguments) {
        self.output.push(format!("{args}"));
    }

    fn progress_bar_length(&self) -> usize {
        64
    }

    fn progress_init(&mut self, s: &str) {
        if self.bar.is_some() {
            panic!("CaptureOutput: double bar")
        }
        self.bar = Some((Vec::with_capacity(self.progress_bar_length() + 5), 0));
        for c in s.chars() {
            self.progress_bump(c);
        }
    }

    fn progress_bump(&mut self, c: char) {
        if let Some((bar, pos)) = self.bar.as_mut() {
            if c == '\r' {
                *pos = 0;
            } else {
                if *pos < bar.len() {
                    bar[*pos] = c;
                } else {
                    bar.push(c);
                }
                *pos += 1;
            }
        } else {
            panic!("CaptureOutput: missing bar")
        }
    }

    fn progress_end(&mut self) {
        if let Some((bar, _)) = self.bar.take() {
            self.output.push(bar.into_iter().collect());
        } else {
            panic!("CaptureOutput: missing bar")
        }
    }
}

const CAPTURED_OUTPUT: &[&str] = &[
    "",
    "Phase 1: Preprocessing file",
    "---------------------------",
    " [building.directed.acyclic.graph.building.directed.acyclic.graph.]",
    " Length of indata: 1648 bytes.",
    " Preprocessing file, done.",
    "",
    "Phase 2: Calculating encoding",
    "-----------------------------",
    " pass 1:",
    "optimizing ..",
    " [finding.shortest.path.finding.shortest.path.finding.shortest.pat]",
    "  size 7393.0 bits ~925 bytes",
    " pass 2: optimizing ..",
    " [finding.shortest.path.finding.shortest.path.finding.shortest.pat]",
    "  size 7368.0 bits ~921 bytes",
    " pass 3: optimizing ..",
    " [finding.shortest.path.finding.shortest.path.finding.shortest.pat]",
    "  size 7365.0 bits ~921 bytes",
    " pass 4: optimizing ..",
    " [finding.shortest.path.finding.shortest.path.finding.shortest.pat]",
    "  size 7363.0 bits ~921 bytes",
    " pass 5: optimizing ..",
    " [finding.shortest.path.finding.shortest.path.finding.shortest.pat]",
    "  size 7360.0 bits ~920 bytes",
    " pass 6: optimizing ..",
    " Calculating encoding, done.",
    "",
    "Phase 3: Generating output file",
    "-------------------------------",
    " Enc: 1011010200000000,2122,3212114335665737,533434454575587A,4445555556766789",
    " Length of crunched data: 957 bytes.",
    " Crunched data reduced 691 bytes (41.93%).",
];
