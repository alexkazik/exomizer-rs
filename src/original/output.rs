use core::fmt::Arguments;

/// Logging and progress information
pub trait Output {
    /// Output log of type dump, without a newline.
    fn log_dump(&mut self, args: Arguments);

    /// Output log of type dump.
    fn log_dump_ln(&mut self, args: Arguments);

    /// Output log of type debug, without a newline.
    fn log_debug(&mut self, args: Arguments);

    /// Output log of type debug.
    fn log_debug_ln(&mut self, args: Arguments);

    /// Output log of type normal.
    fn log_normal_ln(&mut self, args: Arguments);

    /// Output log of type brief.
    fn log_brief_ln(&mut self, args: Arguments);

    /// Output log of type error.
    fn log_error_ln(&mut self, args: Arguments);

    /// The length of the progress bar.
    ///
    /// If <2 then 2 is used.
    fn progress_bar_length(&self) -> usize;

    /// Initialize the progress bar.
    ///
    /// On a terminal simply write the string.
    ///
    /// Though the string may be ignored and other means of initialisation used.
    fn progress_init(&mut self, s: &str);

    /// One step of progress.
    ///
    /// Display the char or other kind of progress.
    ///
    /// This will be called [`Self::progress_bar_length`] times.
    fn progress_bump(&mut self, c: char);

    /// End of the progress bar.
    ///
    /// If the init string was printed, print now a newline.
    fn progress_end(&mut self);
}

impl Output for () {
    #[inline]
    fn log_dump(&mut self, _: Arguments) {}

    #[inline]
    fn log_dump_ln(&mut self, _: Arguments) {}

    #[inline]
    fn log_debug(&mut self, _: Arguments) {}

    #[inline]
    fn log_debug_ln(&mut self, _: Arguments) {}

    #[inline]
    fn log_normal_ln(&mut self, _: Arguments) {}

    #[inline]
    fn log_brief_ln(&mut self, _: Arguments) {}

    #[inline]
    fn log_error_ln(&mut self, _: Arguments) {}

    fn progress_bar_length(&self) -> usize {
        64
    }

    #[inline]
    fn progress_init(&mut self, _: &str) {}

    #[inline]
    fn progress_bump(&mut self, _: char) {}

    #[inline]
    fn progress_end(&mut self) {}
}

#[cfg(feature = "std")]
pub(super) mod feature_std {
    use super::{Arguments, Output};
    use std::io::Write;

    /// Levels of logging.
    ///
    /// Logging of "dump" level is not possible this way as it slows the process too much down.
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub enum Level {
        Debug,
        Normal,
        Brief,
        /// Please note that [`Level::Warning`] is currently not used and thus identical to [`Level::Error`].
        Warning,
        Error,
    }

    /// [`Output`] logs to [`Stdout`](std::io::Stdout), or any other writer.
    pub struct Logger<W> {
        pub level: Level,
        pub writer: W,
        pub progress_bar_length: usize,
    }

    impl Logger<std::io::Stdout> {
        /// Create a new [`Logger`] for stdout.
        ///
        /// Defaults:
        /// - `level`: [`Level::Normal`]
        /// - `writer`: [`Stdout`](std::io::Stdout),
        /// - `progress_bar_length`: `64`
        #[must_use]
        #[inline]
        pub fn stdout() -> Self {
            Self {
                level: Level::Normal,
                writer: std::io::stdout(),
                progress_bar_length: 64,
            }
        }
    }

    impl<W: Write> Logger<W> {
        /// Create a new [`Logger`].
        ///
        /// Defaults:
        /// - `level`: [`Level::Normal`]
        /// - `progress_bar_length`: `64`
        #[inline]
        pub fn with_writer(writer: W) -> Self {
            Self {
                level: Level::Normal,
                writer,
                progress_bar_length: 64,
            }
        }

        /// Set the `level`.
        #[inline]
        pub fn level(&mut self, level: Level) -> &mut Self {
            self.level = level;
            self
        }

        /// Set the `progress_bar_length`.
        #[inline]
        pub fn progress_bar_length(&mut self, progress_bar_length: usize) -> &mut Self {
            self.progress_bar_length = progress_bar_length;
            self
        }
    }

    impl<W: Write> Output for Logger<W> {
        #[inline]
        fn log_dump(&mut self, _: Arguments) {}

        #[inline]
        fn log_dump_ln(&mut self, _: Arguments) {}

        #[inline]
        fn log_debug(&mut self, args: Arguments) {
            if self.level <= Level::Debug {
                _ = write!(self.writer, "{args}");
            }
        }

        #[inline]
        fn log_debug_ln(&mut self, args: Arguments) {
            if self.level <= Level::Debug {
                _ = writeln!(self.writer, "{args}");
            }
        }

        #[inline]
        fn log_normal_ln(&mut self, args: Arguments) {
            if self.level <= Level::Normal {
                _ = writeln!(self.writer, "{args}");
            }
        }

        #[inline]
        fn log_brief_ln(&mut self, args: Arguments) {
            if self.level <= Level::Brief {
                _ = writeln!(self.writer, "{args}");
            }
        }

        #[inline]
        fn log_error_ln(&mut self, args: Arguments) {
            if self.level <= Level::Error {
                _ = writeln!(self.writer, "{args}");
            }
        }

        #[inline]
        fn progress_bar_length(&self) -> usize {
            self.progress_bar_length
        }

        #[inline]
        fn progress_init(&mut self, s: &str) {
            if self.level <= Level::Normal {
                _ = write!(self.writer, "{s}");
                _ = self.writer.flush();
            }
        }

        #[inline]
        fn progress_bump(&mut self, c: char) {
            if self.level <= Level::Normal {
                _ = write!(self.writer, "{c}");
                _ = self.writer.flush();
            }
        }

        #[inline]
        fn progress_end(&mut self) {
            if self.level <= Level::Normal {
                _ = writeln!(self.writer);
            }
        }
    }

    /// [`Output`] logs everything, including dump level, to [`Stdout`](std::io::Stdout), or any other writer.
    pub struct LoggerDump<W> {
        pub writer: W,
    }

    impl<W: Write> Output for LoggerDump<W> {
        fn log_dump(&mut self, args: Arguments) {
            _ = write!(self.writer, "{args}");
        }

        fn log_dump_ln(&mut self, args: Arguments) {
            _ = writeln!(self.writer, "{args}");
        }

        fn log_debug(&mut self, args: Arguments) {
            _ = write!(self.writer, "{args}");
        }

        fn log_debug_ln(&mut self, args: Arguments) {
            _ = writeln!(self.writer, "{args}");
        }

        fn log_normal_ln(&mut self, args: Arguments) {
            _ = writeln!(self.writer, "{args}");
        }

        fn log_brief_ln(&mut self, args: Arguments) {
            _ = writeln!(self.writer, "{args}");
        }

        fn log_error_ln(&mut self, args: Arguments) {
            _ = writeln!(self.writer, "{args}");
        }

        fn progress_bar_length(&self) -> usize {
            64
        }

        fn progress_init(&mut self, s: &str) {
            _ = write!(self.writer, "{s}");
            _ = self.writer.flush();
        }

        fn progress_bump(&mut self, c: char) {
            _ = write!(self.writer, "{c}");
            _ = self.writer.flush();
        }

        fn progress_end(&mut self) {
            _ = writeln!(self.writer);
        }
    }
}
