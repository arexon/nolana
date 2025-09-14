use std::{borrow::Cow, error, fmt, ops};

use miette::{Diagnostic as MietteDiagnostic, LabeledSpan, Severity, SourceCode};

pub type Error = miette::Error;

/// Alias for a `Result` with the error type as [`Diagnostic`].
pub type Result<T> = std::result::Result<T, Diagnostic>;

/// Describes an error or warning that occurred during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Boxed to make [`Diagnostic`] 8 bytes so that `Result` is small.
    /// This is due to rust not supporting [return value optimization].
    ///
    /// [return value optimization]: <https://users.rust-lang.org/t/does-rust-have-return-value-optimization/10389>
    inner: Box<DiagnosticInner>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticInner {
    pub message: Cow<'static, str>,
    pub labels: Option<Vec<LabeledSpan>>,
    pub help: Option<Cow<'static, str>>,
    pub severity: Severity,
}

impl Diagnostic {
    /// Creates a new error-level [`Diagnostic`].
    pub fn error(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            inner: Box::new(DiagnosticInner {
                message: message.into(),
                labels: None,
                help: None,
                severity: Severity::Error,
            }),
        }
    }

    /// Creates a new warning-level [`Diagnostic`].
    pub fn warning(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            inner: Box::new(DiagnosticInner {
                message: message.into(),
                labels: None,
                help: None,
                severity: Severity::Warning,
            }),
        }
    }

    /// Sets a possible suggestion for a problem to the user.
    pub fn with_help(mut self, help: impl Into<Cow<'static, str>>) -> Self {
        self.inner.help = Some(help.into());
        self
    }

    /// Sets a label covering the problematic portion of the source code.
    ///
    /// Existing labels will be removed. Use [`Diagnostic::add_label`] to append
    /// labels instead.
    pub fn with_label(mut self, label: impl Into<LabeledSpan>) -> Self {
        self.inner.labels = Some(vec![label.into()]);
        self
    }

    /// Appends a label to this diagnostic without affecting previous ones.
    pub fn add_label(mut self, label: impl Into<LabeledSpan>) -> Self {
        let mut labels = self.inner.labels.unwrap_or_default();
        labels.push(label.into());
        self.inner.labels = Some(labels);
        self
    }

    /// Adds a source to this diagnostic and converts it into an [`Error`].
    pub fn with_source_code(self, code: impl SourceCode + 'static) -> Error {
        Error::from(self).with_source_code(code)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for Diagnostic {}

impl ops::Deref for Diagnostic {
    type Target = DiagnosticInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl MietteDiagnostic for Diagnostic {
    fn severity(&self) -> Option<Severity> {
        Some(self.severity)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.help.as_ref().map(Box::new).map(|help| help as Box<dyn fmt::Display>)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        self.labels
            .as_ref()
            .map(|labels| labels.iter().cloned())
            .map(Box::new)
            .map(|labels| labels as Box<dyn Iterator<Item = LabeledSpan>>)
    }
}
