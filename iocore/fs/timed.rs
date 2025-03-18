use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
        write!(f, "{}", self.human_friendly(None))
    }
}

impl std::fmt::Debug for PathDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", &self.t)
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
}

impl PathDateTime {
    pub fn year(&self) -> u16 {
        u16::from_str(&self.t.format("%Y").to_string()).unwrap()
    }

    pub fn month(&self) -> u16 {
        u16::from_str(&self.t.format("%m").to_string()).unwrap()
    }

    pub fn day(&self) -> u16 {
        u16::from_str(&self.t.format("%d").to_string()).unwrap()
    }

    pub fn hours(&self) -> u16 {
        u16::from_str(&self.t.format("%H").to_string()).unwrap()
    }

    pub fn minutes(&self) -> u16 {
        u16::from_str(&self.t.format("%H").to_string()).unwrap()
    }

    pub fn seconds(&self) -> u16 {
        u16::from_str(&self.t.format("%H").to_string()).unwrap()
    }

    pub fn to_usize<'a>(&self) -> [u16; 6] {
        [
            self.year(),
            self.month(),
            self.day(),
            self.hours(),
            self.minutes(),
            self.seconds(),
        ]
    }
}
impl std::cmp::PartialEq for PathDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.to_usize().eq(&other.to_usize())
    }
}
impl std::cmp::Eq for PathDateTime {}
impl std::cmp::PartialOrd for PathDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_usize().partial_cmp(&other.to_usize())
    }
}
impl std::cmp::Ord for PathDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

impl std::hash::Hash for PathDateTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_usize().hash(state);
    }
}
