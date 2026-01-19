mod rzone;
mod generate;
mod pinmapping;
mod util;
mod perimap;
mod interrupts;
mod mstp;
mod timer;

#[macro_export]
macro_rules! regex {
    ($re:literal) => {{
        ::ref_thread_local::ref_thread_local! {
            static managed REGEX: ::regex::Regex = ::regex::Regex::new($re).unwrap();
        }
        <REGEX as ::ref_thread_local::RefThreadLocal<::regex::Regex>>::borrow(&REGEX)
    }};
}

struct Stopwatch {
    start: std::time::Instant,
    section_start: Option<std::time::Instant>,
}

impl Stopwatch {
    fn new() -> Self {
        eprintln!("Starting timer");
        let start = std::time::Instant::now();
        Self {
            start,
            section_start: None,
        }
    }

    fn section(&mut self, status: &str) {
        let now = std::time::Instant::now();
        self.print_done(now);
        eprintln!("  {status}");
        self.section_start = Some(now);
    }

    fn stop(self) {
        let now = std::time::Instant::now();
        self.print_done(now);
        let total_elapsed = now - self.start;
        eprintln!("Total time: {:.2} seconds", total_elapsed.as_secs_f32());
    }

    fn print_done(&self, now: std::time::Instant) {
        if let Some(section_start) = self.section_start {
            let elapsed = now - section_start;
            eprintln!("    done in {:.2} seconds", elapsed.as_secs_f32());
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let mut stopwatch = Stopwatch::new();

    stopwatch.section("Parsing headers");
    let (chips, rzones) = rzone::Rzones::parse()?;

    println!("Parsed {} chips", chips.len());

    stopwatch.section("Parsing pin mappings");
    let pin_mappings = pinmapping::PinMappings::parse()?;

    stopwatch.section("Parsing interrupts");
    let family_interrupts = interrupts::parse_all()?;

    stopwatch.section("Parsing Timers");
    let chip_timers = timer::parse_all()?;

    stopwatch.section("Generating data");
    // MSTP is now computed on-the-fly using rules from bsp_module_stop.h
    generate::generate(&rzones, &pin_mappings, &family_interrupts, &chip_timers)?;

    stopwatch.section("Parsing other stuff");

    stopwatch.stop();

    Ok(())
}

