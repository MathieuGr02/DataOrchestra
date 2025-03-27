use super::super::common::common_trait::ToValue;

enum ProcessLanguage {
    Rust,
    Go,
    Java,
    Python,
    CSharp,
    C,
    CPP
}

impl ToValue for ProcessLanguage {
    /// Get docker container image from process language enum
    ///
    /// # Examples 
    ///
    /// ```
    /// let language = ProcessLanguage.Rust.to_value();
    /// println!("{}", language);
    /// ```
    fn to_value(&self) -> String {
        match self {
            Self::Rust => String::from("rust"),
            Self::Go => String::from("golang"),
            Self::Java => String::from("java"),
            Self::Python => String::from("python"),
            Self::CSharp => String::from("csharp"),
            Self::C => String::from("gcc"),
            // TODO: Find C++ image
            Self::CPP => String::from(""),
            _ => return String::new()
        }
    }
}
