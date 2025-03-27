use super::super::common::common_trait::ToValue;

enum StoreType {
    PostGres,
    MySQL,
    SQLite3,
    MongoDB,
    Redis
}

impl ToValue for StoreType {
    /// Get docker container image from story type enum
    ///
    /// # Examples
    /// ```
    /// let store = StoreType.PostGres.to_value();
    /// println!("{}", store);
    /// ```
    fn to_value(&self) -> String {
        match self {
            Self::PostGres => return String::from("postgres"),
            Self::MySQL => return String::from("mysql"),
            // TODO: Check if container correct
            Self::SQLite3 => return String::from("keinos/sqlite3"), 
            Self::MongoDB => return String::from("mongo"),
            Self::Redis => return String::from("redis"),
            _ => return String::from("")
        }
    }
}
