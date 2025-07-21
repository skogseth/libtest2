#[derive(Clone, Debug)]
#[cfg_attr(feature = "unstable-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "serde", serde(tag = "event"))]
pub enum Event {
    DiscoverStart,
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
    },
    DiscoverComplete {
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        elapsed_s: Option<Elapsed>,
    },
    SuiteStart,
    CaseStart {
        name: String,
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
        serde_json::to_string(self).expect("always valid json")
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
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum RunStatus {
    Ignored,
    Failed,
}

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
