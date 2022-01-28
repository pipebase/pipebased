use pipebased_common::{
    grpc::daemon::{ListAppResponse, ListCatalogsResponse, ListPipeResponse},
    Result,
};
use std::{fmt, io::Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const DISPLAY_ID_WIDTH: usize = 12;
const DISPLAY_NAMESPACE_WIDTH: usize = 12;
const DISPLAY_VERSION_WIDTH: usize = 12;
const DISPLAY_LOAD_STATE_WIDTH: usize = 12;
const DISPLAY_ACTIVE_STATE_WIDTH: usize = 12;
const DISPLAY_SUBSTATE_STATE_WIDTH: usize = 12;

pub trait PrintRecords {
    fn print_records(&self);
}

impl PrintRecords for ListAppResponse {
    fn print_records(&self) {
        // print header
        println!(
            "{col0:<col0_width$}{col1:<col1_width$}{col2:<col2_width$}",
            col0 = "Namespace",
            col1 = "Id",
            col2 = "Version",
            col0_width = DISPLAY_NAMESPACE_WIDTH,
            col1_width = DISPLAY_ID_WIDTH,
            col2_width = DISPLAY_VERSION_WIDTH,
        );
        for app in &self.apps {
            println!(
                "{col0:<col0_width$}{col1:<col1_width$}{col2:<col2_width$}",
                col0 = app.namespace,
                col1 = app.id,
                col2 = app.version,
                col0_width = DISPLAY_NAMESPACE_WIDTH,
                col1_width = DISPLAY_ID_WIDTH,
                col2_width = DISPLAY_VERSION_WIDTH,
            );
        }
    }
}

impl PrintRecords for ListCatalogsResponse {
    fn print_records(&self) {
        // print header
        println!(
            "{col0:<col0_width$}{col1:<col1_width$}{col2:<col2_width$}",
            col0 = "Namespace",
            col1 = "Id",
            col2 = "Version",
            col0_width = DISPLAY_NAMESPACE_WIDTH,
            col1_width = DISPLAY_ID_WIDTH,
            col2_width = DISPLAY_VERSION_WIDTH,
        );
        for catalogs in &self.catalogss {
            println!(
                "{col0:<col0_width$}{col1:<col1_width$}{col2:<col2_width$}",
                col0 = catalogs.namespace,
                col1 = catalogs.id,
                col2 = catalogs.version,
                col0_width = DISPLAY_NAMESPACE_WIDTH,
                col1_width = DISPLAY_ID_WIDTH,
                col2_width = DISPLAY_VERSION_WIDTH,
            );
        }
    }
}

impl PrintRecords for ListPipeResponse {
    fn print_records(&self) {
        // print header
        println!(
            "{col0:<col0_width$}{col1:<col1_width$}{col2:<col2_width$}{col3:<col3_width$}",
            col0 = "Id",
            col1 = "Load",
            col2 = "Active",
            col3 = "Sub",
            col0_width = DISPLAY_ID_WIDTH,
            col1_width = DISPLAY_LOAD_STATE_WIDTH,
            col2_width = DISPLAY_ACTIVE_STATE_WIDTH,
            col3_width = DISPLAY_SUBSTATE_STATE_WIDTH,
        );
        for pipe in &self.pipes {
            println!(
                "{col0:<col0_width$}{col1:<col1_width$}{col2:<col2_width$}{col3:<col3_width$}",
                col0 = pipe.id,
                col1 = pipe.load_state,
                col2 = pipe.active_state,
                col3 = pipe.sub_state,
                col0_width = DISPLAY_ID_WIDTH,
                col1_width = DISPLAY_LOAD_STATE_WIDTH,
                col2_width = DISPLAY_ACTIVE_STATE_WIDTH,
                col3_width = DISPLAY_SUBSTATE_STATE_WIDTH,
            );
        }
    }
}

pub(crate) struct Printer {
    stderr: StandardStream,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            stderr: StandardStream::stderr(ColorChoice::Auto),
        }
    }

    pub fn print(
        &mut self,
        status: &dyn fmt::Display,
        message: Option<&dyn fmt::Display>,
        color: Color,
    ) -> Result<()> {
        self.stderr.reset()?;
        self.stderr
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))?;
        // write status
        write!(self.stderr, "{:>12}", status)?;
        // write message
        self.stderr.reset()?;
        match message {
            Some(message) => writeln!(self.stderr, " {}", message)?,
            None => write!(self.stderr, " ")?,
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn status<T: fmt::Display, U: fmt::Display>(
        &mut self,
        status: T,
        message: U,
    ) -> Result<()> {
        self.print(&status, Some(&message), Color::Green)
    }

    pub fn error<T: fmt::Display>(&mut self, message: T) -> Result<()> {
        self.print(&"Error", Some(&message), Color::Red)
    }

    #[allow(dead_code)]
    pub fn warning<T: fmt::Display>(&mut self, message: T) -> Result<()> {
        self.print(&"Warning", Some(&message), Color::Yellow)
    }
}
