use super::event::CaseComplete;
use super::Event;
use super::RunStatus;
use super::FAILED;
use super::OK;

#[derive(Default, Clone, Debug)]
pub(crate) struct Summary {
    num_run: usize,
    /// Number of tests and benchmarks that were filtered out (either by the
    /// filter-in pattern or by `--skip` arguments).
    num_filtered_out: usize,

    status: std::collections::HashMap<String, CaseComplete>,
    elapsed_s: Option<super::Elapsed>,
}

impl Summary {
    pub(crate) fn get_status(&self, name: &str) -> Option<RunStatus> {
        let event = self.status.get(name)?;
        event.status
    }

    pub(crate) fn write_start(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        let s = if self.num_run == 1 { "" } else { "s" };

        writeln!(writer)?;
        writeln!(writer, "running {} test{s}", self.num_run)?;
        Ok(())
    }

    pub(crate) fn write_complete(&self, writer: &mut dyn ::std::io::Write) -> std::io::Result<()> {
        let mut num_passed = 0;
        let mut num_failed = 0;
        let mut num_ignored = 0;
        let mut failures = std::collections::BTreeMap::new();
        for event in self.status.values() {
            match event.status {
                Some(RunStatus::Ignored) => num_ignored += 1,
                Some(RunStatus::Failed) => {
                    num_failed += 1;
                    failures.insert(&event.name, &event.message);
                }
                None => num_passed += 1,
            }
        }

        let has_failed = 0 < num_failed;

        let (summary, summary_style) = if has_failed {
            ("FAILED", FAILED)
        } else {
            ("ok", OK)
        };
        let num_filtered_out = self.num_filtered_out;
        let elapsed_s = self.elapsed_s;

        if has_failed {
            writeln!(writer)?;
            writeln!(writer, "failures:")?;
            writeln!(writer)?;

            // Print messages of all tests
            for (name, msg) in &failures {
                if let Some(msg) = msg {
                    writeln!(writer, "---- {name} ----")?;
                    writeln!(writer, "{msg}")?;
                    writeln!(writer)?;
                }
            }

            // Print summary list of failed tests
            writeln!(writer)?;
            writeln!(writer, "failures:")?;
            for name in failures.keys() {
                writeln!(writer, "    {name}")?;
            }
        }
        writeln!(writer)?;
        let finished = if let Some(elapsed_s) = elapsed_s {
            format!("; finished in {elapsed_s}")
        } else {
            "".to_owned()
        };
        writeln!(
                    writer,
                    "test result: {summary_style}{summary}{summary_style:#}. {num_passed} passed; {num_failed} failed; {num_ignored} ignored; \
                        {num_filtered_out} filtered out{finished}",
                )?;
        writeln!(writer)?;

        Ok(())
    }
}

impl super::Notifier for Summary {
    fn notify(&mut self, event: Event) -> std::io::Result<()> {
        match event {
            Event::DiscoverStart(_) => {}
            Event::DiscoverCase(inner) => {
                if inner.selected {
                    self.num_run += 1;
                } else {
                    self.num_filtered_out += 1;
                }
            }
            Event::DiscoverComplete(_) => {}
            Event::RunStart(_) => {}
            Event::CaseStart(_) => {}
            Event::CaseComplete(inner) => {
                self.status.insert(inner.name.clone(), inner);
            }
            Event::RunComplete(inner) => {
                self.elapsed_s = inner.elapsed_s;
            }
        }
        Ok(())
    }
}
