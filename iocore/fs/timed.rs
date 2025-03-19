use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;
use std::time::SystemTime;

use chrono::format::SecondsFormat;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

#[derive(Clone)]
pub struct PathDateTime {
    t: DateTime<Local>,
}

impl From<SystemTime> for PathDateTime {
    fn from(st: SystemTime) -> PathDateTime {
        PathDateTime { t: st.into() }
    }
}

impl Display for PathDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

impl std::fmt::Debug for PathDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PathDateTime[{}]", self.to_rfc3339())
    }
}

impl Serialize for PathDateTime {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.t.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for PathDateTime {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PathDateTime::from(DateTime::<Local>::deserialize(de)?))
    }
}
impl From<DateTime<Local>> for PathDateTime {
    fn from(t: DateTime<Local>) -> PathDateTime {
        PathDateTime { t }
    }
}

impl Into<DateTime<Local>> for PathDateTime {
    fn into(self) -> DateTime<Local> {
        (&self).t.clone()
    }
}

impl PathDateTime {
    pub fn human_friendly(&self, t: Option<DateTime<Local>>) -> String {
        let day = self.t.format("%e").to_string().trim().to_string();
        let fmt = if self.t.format("%Y").to_string()
            == t.unwrap_or(Local::now()).format("%Y").to_string()
        {
            format!("%h {: <2} %H:%M", day)
        } else {
            format!("%h {: <2} %Y", day)
        }
        .to_string();
        self.t.format(fmt.as_str()).to_string()
    }

    pub fn to_rfc3339(&self) -> String {
        self.t.to_rfc3339_opts(SecondsFormat::Nanos, true)
    }

    pub fn from_datetime_utc(datetime: &DateTime<Utc>) -> PathDateTime {
        PathDateTime::from_datetime(&datetime.with_timezone(&Local))
    }

    pub fn from_datetime_fixed_offset(datetime: &DateTime<FixedOffset>) -> PathDateTime {
        PathDateTime::from_datetime(&datetime.with_timezone(&Local))
    }

    pub fn from_datetime(t: &DateTime<Local>) -> PathDateTime {
        let t = t.clone();
        PathDateTime { t }
    }

    pub fn from_timestamp(secs: i64, nsecs: u32) -> PathDateTime {
        PathDateTime::from_datetime_utc(
            &DateTime::from_timestamp(secs, nsecs)
                .expect(&format!("chrono::DateTime<Utc> from {} secs and {} nsecs", secs, nsecs)),
        )
    }

    pub fn parse_from_str(s: &str, fmt: &str) -> Result<PathDateTime, Error> {
        Ok(PathDateTime::from_datetime_utc(
            &NaiveDateTime::parse_from_str(s, fmt)
                .map_err(|error| {
                    Error::ParseError(format!(
                        "error parsing '{}' with format '{}': {}",
                        s, fmt, error
                    ))
                })?
                .and_utc(),
        ))
    }

    pub fn local_datetime(&self) -> DateTime<Local> {
        self.t.with_timezone(&Local)
    }

    pub fn utc_datetime(&self) -> DateTime<Utc> {
        self.t.with_timezone(&Utc)
    }

    pub fn to_array(&self) -> [u16; 6] {
        [
            u16::from_str(&self.utc_datetime().format("%Y").to_string()).unwrap(),
            u16::from_str(&self.utc_datetime().format("%m").to_string()).unwrap(),
            u16::from_str(&self.utc_datetime().format("%d").to_string()).unwrap(),
            u16::from_str(&self.utc_datetime().format("%H").to_string()).unwrap(),
            u16::from_str(&self.utc_datetime().format("%M").to_string()).unwrap(),
            u16::from_str(&self.utc_datetime().format("%S").to_string()).unwrap(),
        ]
    }

    pub fn to_bytes<'a>(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        for data in self.to_array() {
            bytes.extend(data.to_be_bytes())
        }
        bytes
    }
}
impl std::cmp::PartialEq for PathDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.utc_datetime().eq(&other.utc_datetime())
    }
}
impl std::cmp::Eq for PathDateTime {}
impl std::cmp::PartialOrd for PathDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.utc_datetime().partial_cmp(&other.utc_datetime())
    }
}
impl std::cmp::Ord for PathDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.utc_datetime().cmp(&other.utc_datetime())
    }
}

impl std::hash::Hash for PathDateTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.utc_datetime().hash(state);
        self.local_datetime().hash(state);
        self.to_bytes().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset, Local, NaiveDate, TimeZone};

    use crate::PathDateTime;

    #[test]
    fn test_from_datetime_utc() {
        let datetime = DateTime::from_timestamp(1742346763, 0).unwrap();
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).to_string(),
                "2025-03-19T01:12:43.000000000Z"
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).to_string(),
                "2025-03-18T22:12:43.000000000-03:00"
            );
        }
    }
    #[test]
    fn test_from_datetime_fixedoffset() {
        let datetime = FixedOffset::east_opt(0)
            .unwrap()
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 3, 19)
                    .unwrap()
                    .and_hms_nano_opt(1, 12, 43, 0)
                    .unwrap(),
            )
            .unwrap();
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime_fixed_offset(&datetime).to_string(),
                "2025-03-19T01:12:43.000000000Z"
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime_fixed_offset(&datetime).to_string(),
                "2025-03-18T22:12:43.000000000-03:00"
            );
        }
    }
    #[test]
    fn test_from_datetime() {
        let datetime = DateTime::from_timestamp(1742346763, 0).unwrap().with_timezone(&Local);
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime(&datetime).to_string(),
                "2025-03-19T01:12:43.000000000Z"
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime(&datetime).to_string(),
                "2025-03-18T22:12:43.000000000-03:00"
            );
        }
    }
    #[test]
    fn test_from_timestamp() {
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_timestamp(1742346763, 0).to_string(),
                "2025-03-19T01:12:43.000000000Z"
            );
        } else {
            assert_eq!(
                PathDateTime::from_timestamp(1742346763, 0).to_string(),
                "2025-03-18T22:12:43.000000000-03:00"
            );
        }
    }
    #[test]
    fn test_parse_from_str() {
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::parse_from_str("2025-03-19T01:12:43", "%Y-%m-%dT%H:%M:%S")
                    .unwrap()
                    .to_string(),
                "2025-03-19T01:12:43.000000000Z"
            );
            assert_eq!(
                PathDateTime::parse_from_str("2025-03-19T01:12:43", "%Y-%m-%dT%H:%M:%S")
                    .unwrap()
                    .to_string(),
                "2025-03-19T01:12:43.000000000Z"
            );
            assert_eq!(
                PathDateTime::parse_from_str(
                    "2025-03-19T01:12:43.123456789Z",
                    "%Y-%m-%dT%H:%M:%S.%fZ"
                )
                .unwrap()
                .to_string(),
                "2025-03-19T01:12:43.123456789Z"
            );
        } else {
            assert_eq!(
                PathDateTime::parse_from_str("2025-03-19T01:12:43", "%Y-%m-%dT%H:%M:%S")
                    .unwrap()
                    .to_string(),
                "2025-03-18T22:12:43.000000000-03:00"
            );

            assert_eq!(
                PathDateTime::parse_from_str("2025-03-19 01:12:43", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .to_string(),
                "2025-03-18T22:12:43.000000000-03:00"
            );
            assert_eq!(
                PathDateTime::parse_from_str(
                    "2025-03-19T01:12:43.123456789Z",
                    "%Y-%m-%dT%H:%M:%S.%fZ"
                )
                .unwrap()
                .to_string(),
                "2025-03-18T22:12:43.123456789-03:00"
            );
        }
    }
    #[test]
    fn test_to_array() {
        let datetime = DateTime::from_timestamp(1742346763, 0).unwrap();
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).to_array(),
                [2025, 3, 19, 1, 12, 43]
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).to_array(),
                [2025, 3, 19, 1, 12, 43]
            );
        }
    }
    #[test]
    fn test_to_bytes() {
        let datetime = DateTime::from_timestamp(1742346763, 0).unwrap();
        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).to_bytes(),
                vec![7, 233, 0, 3, 0, 19, 0, 1, 0, 12, 0, 43],
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).to_bytes(),
                vec![7, 233, 0, 3, 0, 19, 0, 1, 0, 12, 0, 43],
            );
        }
    }
    #[test]
    fn test_local_datetime() {
        let datetime = DateTime::from_timestamp(1742346763, 0).unwrap();

        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).local_datetime().to_string(),
                "2025-03-19 01:12:43 +00:00"
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).local_datetime().to_string(),
                "2025-03-18 22:12:43 -03:00"
            );
        }
    }
    #[test]
    fn test_utc_datetime() {
        let datetime = DateTime::from_timestamp(1742346763, 0).unwrap();

        if std::env::var("TZ").unwrap_or_default() == "UTC" {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).utc_datetime().to_string(),
                "2025-03-19 01:12:43 UTC"
            );
        } else {
            assert_eq!(
                PathDateTime::from_datetime_utc(&datetime).utc_datetime().to_string(),
                "2025-03-19 01:12:43 UTC"
            );
        }
    }
}
