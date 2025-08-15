#[derive(Clone, Debug)]
#[cfg_attr(feature = "unstable-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", serde(tag = "event"))]
pub enum Event {
    DiscoverStart {
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    DiscoverCase {
        name: String,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "RunMode::is_default")
        )]
        mode: RunMode,
        /// Whether selected to be run by the user
        #[cfg_attr(
            feature = "serde",
            serde(default = "true_default", skip_serializing_if = "is_true")
        )]
        run: bool,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    DiscoverComplete {
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    SuiteStart {
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    CaseStart {
        name: String,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    CaseComplete {
        name: String,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "RunMode::is_default")
        )]
        mode: RunMode,
        /// `None` means success
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        status: Option<RunStatus>,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        message: Option<String>,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    SuiteComplete {
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
}

impl Event {
    #[cfg(feature = "json")]
    pub fn to_jsonline(&self) -> String {
        use json_write::JsonWrite as _;

        let mut buffer = String::new();
        buffer.open_object().unwrap();
        match self {
            Self::DiscoverStart { elapsed_s } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("discover_start").unwrap();

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
            Self::DiscoverCase {
                name,
                mode,
                run,
                elapsed_s,
            } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("discover_case").unwrap();

                buffer.val_sep().unwrap();
                buffer.key("name").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value(name).unwrap();

                if !mode.is_default() {
                    buffer.val_sep().unwrap();
                    buffer.key("mode").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(mode.as_str()).unwrap();
                }

                if !run {
                    buffer.val_sep().unwrap();
                    buffer.key("run").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(run).unwrap();
                }

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
            Self::DiscoverComplete { elapsed_s } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("discover_complete").unwrap();

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
            Self::SuiteStart { elapsed_s } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("suite_start").unwrap();

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
            Self::CaseStart { name, elapsed_s } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("case_start").unwrap();

                buffer.val_sep().unwrap();
                buffer.key("name").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value(name).unwrap();

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
            Self::CaseComplete {
                name,
                mode,
                status,
                message,
                elapsed_s,
            } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("case_complete").unwrap();

                buffer.val_sep().unwrap();
                buffer.key("name").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value(name).unwrap();

                if !mode.is_default() {
                    buffer.val_sep().unwrap();
                    buffer.key("mode").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(mode.as_str()).unwrap();
                }

                if let Some(status) = status {
                    buffer.val_sep().unwrap();
                    buffer.key("status").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(status.as_str()).unwrap();
                }

                if let Some(message) = message {
                    buffer.val_sep().unwrap();
                    buffer.key("message").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(message).unwrap();
                }

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
            Self::SuiteComplete { elapsed_s } => {
                buffer.key("event").unwrap();
                buffer.keyval_sep().unwrap();
                buffer.value("suite_complete").unwrap();

                if let Some(elapsed_s) = elapsed_s {
                    buffer.val_sep().unwrap();
                    buffer.key("elapsed_s").unwrap();
                    buffer.keyval_sep().unwrap();
                    buffer.value(String::from(*elapsed_s)).unwrap();
                }
            }
        }
        buffer.close_object().unwrap();

        buffer
    }
}

fn true_default() -> bool {
    true
}

fn is_true(yes: &bool) -> bool {
    *yes
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "unstable-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RunMode {
    #[default]
    Test,
    Bench,
}

impl RunMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Test => "test",
            Self::Bench => "bench",
        }
    }

    fn is_default(&self) -> bool {
        *self == Default::default()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "unstable-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RunStatus {
    Ignored,
    Failed,
}

impl RunStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ignored => "ignored",
            Self::Failed => "failed",
        }
    }
}

/// Time elapsed since process start
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "unstable-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Elapsed(pub std::time::Duration);

impl std::fmt::Display for Elapsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}s", self.0.as_secs_f64())
    }
}

impl std::str::FromStr for Elapsed {
    type Err = std::num::ParseFloatError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let secs = src.parse()?;
        Ok(Elapsed(std::time::Duration::from_secs_f64(secs)))
    }
}

impl TryFrom<String> for Elapsed {
    type Error = std::num::ParseFloatError;

    fn try_from(inner: String) -> Result<Self, Self::Error> {
        inner.parse()
    }
}

impl From<Elapsed> for String {
    fn from(elapsed: Elapsed) -> Self {
        elapsed.0.as_secs_f64().to_string()
    }
}

#[cfg(feature = "unstable-schema")]
#[test]
fn dump_event_schema() {
    let schema = schemars::schema_for!(Event);
    let dump = serde_json::to_string_pretty(&schema).unwrap();
    snapbox::assert_data_eq!(dump, snapbox::file!("../event.schema.json").raw());
}
