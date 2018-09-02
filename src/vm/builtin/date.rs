

use chrono::{ self, Utc, Local, NaiveDateTime, NaiveDate, FixedOffset, TimeZone };
use chrono::Datelike;
use chrono::Timelike;
use chrono::Weekday;

use value::Value;
use value::String;
use value::Number;
use value::Object;
use error::Error;


use std::fmt;
use std::cmp;
use std::hash;
use std::string;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};


// 分数
// 值       符号  名称
// 10−1 s   ds   分秒
// 10−2 s   cs   厘秒
// 10−3 s   ms   毫秒
// 10−6 s   µs   微秒
// 10−9 s   ns   纳秒
// 10−12 s  ps   皮秒
// 10−15 s  fs   飞秒
// 10−18 s  as   阿秒
// 10−21 s  zs   仄秒
// 10−24 s  ys   幺秒

// 倍数
// 值      符号   名称
// 101 s   das   十秒
// 102 s   hs    百秒
// 103 s   ks    千秒
// 106 s   Ms    兆秒
// 109 s   Gs    吉秒
// 1012 s  Ts    太秒
// 1015 s  Ps    拍秒
// 1018 s  Es    艾秒
// 1021 s  Zs    泽秒
// 1024 s  Ys    尧秒

const NANOS_PER_SEC: u32   = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const NANOS_PER_MICRO: u32 = 1_000;

const MILLIS_PER_SEC: u64  = 1_000;
const MICROS_PER_SEC: u64  = 1_000_000;



pub fn duration_to_milliseconds(secs: u64, subsec_nanos: u32) -> u128 {
    (secs as u128 * MILLIS_PER_SEC as u128) + (subsec_nanos / NANOS_PER_MILLI) as u128
}

pub fn duration_to_microseconds(secs: u64, subsec_nanos: u32) -> u128 {
    (secs as u128 * MICROS_PER_SEC as u128) + (subsec_nanos / NANOS_PER_MICRO) as u128
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Date {
    utc: NaiveDateTime,
    offset: FixedOffset, // -86_400 < offset < 86_400
}


impl Date {

    pub fn now() -> Result<u128, Object> {
        SystemTime::now().duration_since(UNIX_EPOCH)
            .map(|duration| {
                duration.as_millis()
            })
            // FIXME: Return JSValue ( RangeError )
            .map_err(|e| panic!("SystemTime before UNIX EPOCH!") )
    }

    pub fn UTC(year: u32, month: u32, date: u32, hrs: u32, min: u32, sec: u32, ms: u32) -> Result<u128, Object> {
        // https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC
        let year = match year {
            n @ 0 ... 99 => (1900 + n) as i32,
            _ => year as i32,
        };

        if year < 1900 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("year must between 0 and 99 or >= 1900."));
        }
        if month > 11 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("month must between 0 and 11."));
        }
        if date < 1 || date > 31 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("date must between 1 and 31."));
        }

        if hrs > 23 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("hours must between 0 and 23."));
        }
        if min > 59 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("minus must between 0 and 59."));
        }
        if sec > 59 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("seconds must between 0 and 59."));
        }

        if ms > 999 {
            // FIXME: Return JSValue ( RangeError )
            return Err(panic!("milliseconds must between 0 and 999."));
        }

        let month = month + 1;
        let utc: NaiveDateTime = NaiveDate::from_ymd(year, month, date)
                                        .and_hms_milli(hrs, min, sec, ms);

        let secs = utc.timestamp() as u64;
        let subsec_nanos = utc.timestamp_subsec_nanos();

        Ok(duration_to_milliseconds(secs, subsec_nanos))
    }

    pub fn parse(s: &str, fmt: Option<&str>) -> Number {
        let dt = if fmt.is_some() {
            chrono::DateTime::parse_from_str(s, fmt.unwrap())
        } else {
            chrono::DateTime::parse_from_rfc3339(s)
        };

        match dt {
            Ok(dt) => {
                let utc = dt.naive_utc();
                let offset = dt.offset().to_owned();

                let date = Date { utc: utc, offset: offset };
                
                date.getTime()
            }
            Err(e) => {
                use std::f64;
                f64::NAN.into()
            }
        }
    }

    pub fn format(&self, fmt: &str) -> String {
        // Example: "%Y %m %d %H:%M:%S%.3f %z"
        format!("{}", self.offset.from_utc_datetime(&self.utc).format(fmt)).into()
    }


    pub fn getTimezoneOffset(&self) -> Number {
        // 单位: 分钟
        self.offset.local_minus_utc().into()
    }

    fn get_milliseconds(&self) -> u128 {
        let secs: u64 = self.utc.timestamp() as u64;
        let subsec_nanos: u32 = self.utc.timestamp_subsec_nanos();

        duration_to_milliseconds(secs, subsec_nanos)
    }

    pub fn getTime(&self) -> Number {
        // FIXME: 需要处理溢出问题
        (self.get_milliseconds() as  u64).into()
    }
    pub fn setTime(&mut self, ms: Number) {
        let mut milliseconds: f64 = ms.into();
        milliseconds -= (self.offset.local_minus_utc() * 60 * 1000) as f64;

        let temp = milliseconds / MILLIS_PER_SEC as f64;
        
        let secs = temp.floor();
        let subsec_nanos = ((temp - secs) * MILLIS_PER_SEC as f64).floor() as u32;

        let ndt = NaiveDateTime::from_timestamp(secs as i64, subsec_nanos);

        self.utc = ndt;
    }


    pub fn getDate(&self) -> Number {
        self.offset.from_utc_datetime(&self.utc).day().into()
    }
    pub fn getDay(&self) -> Number {
        match self.offset.from_utc_datetime(&self.utc).weekday() {
            Weekday::Mon => 1u8.into(),
            Weekday::Tue => 2u8.into(),
            Weekday::Wed => 3u8.into(),
            Weekday::Thu => 4u8.into(),
            Weekday::Fri => 5u8.into(),
            Weekday::Sat => 6u8.into(),
            Weekday::Sun => 0u8.into(),
        }
    }
    pub fn getFullYear(&self) -> Number {
        self.offset.from_utc_datetime(&self.utc).year().into()
    }
    pub fn getMonth(&self) -> Number {
        self.offset.from_utc_datetime(&self.utc).month().into()
    }

    pub fn getHours(&self) -> Number {
        self.offset.from_utc_datetime(&self.utc).hour().into()
    }
    pub fn getMinutes(&self) -> Number {
        self.offset.from_utc_datetime(&self.utc).minute().into()
    }
    pub fn getSeconds(&self) -> Number {
        self.offset.from_utc_datetime(&self.utc).second().into()
    }
    pub fn getMilliseconds(&self) -> Number {
        unimplemented!()
    }

    pub fn getUTCFullYear(&self) -> Number {
        self.utc.year().into()
    }
    pub fn getUTCMonth(&self) -> Number {
        self.utc.month().into()
    }
    pub fn getUTCDate(&self) -> Number {
        self.utc.day().into()
    }
    pub fn getUTCDay(&self) -> Number {
        match self.utc.weekday() {
            Weekday::Mon => 1u8.into(),
            Weekday::Tue => 2u8.into(),
            Weekday::Wed => 3u8.into(),
            Weekday::Thu => 4u8.into(),
            Weekday::Fri => 5u8.into(),
            Weekday::Sat => 6u8.into(),
            Weekday::Sun => 0u8.into(),
        }
    }
    
    pub fn getUTCHours(&self) -> Number {
        self.utc.hour().into()
    }
    pub fn getUTCMinutes(&self) -> Number {
        self.utc.minute().into()
    }
    pub fn getUTCSeconds(&self) -> Number {
        self.utc.second().into()
    }
    pub fn getUTCMilliseconds(&self) -> Number {
        unimplemented!()
    }
    
    pub fn setFullYear(&mut self) {
        unimplemented!()
    }
    pub fn setMonth(&mut self) {
        unimplemented!()
    }
    pub fn setDate(&mut self) {
        unimplemented!()
    }

    pub fn setHours(&mut self) {
        unimplemented!()
    }
    pub fn setMinutes(&mut self) {
        unimplemented!()
    }
    pub fn setSeconds(&mut self) {
        unimplemented!()
    }
    pub fn setMilliseconds(&mut self) {
        unimplemented!()
    }
    
    pub fn setUTCFullYear(&mut self) {
        unimplemented!()
    }
    pub fn setUTCDate(&mut self) {
        unimplemented!()
    }
    pub fn setUTCMonth(&mut self) {
        unimplemented!()
    }

    pub fn setUTCHours(&mut self) {
        unimplemented!()
    }
    pub fn setUTCMinutes(&mut self) {
        unimplemented!()
    }
    pub fn setUTCSeconds(&mut self) {
        unimplemented!()
    }
    pub fn setUTCMilliseconds(&mut self) {
        unimplemented!()
    }


    pub fn toUTCString(&self) -> String {
        self.toISOString()
    }
    pub fn toDateString(&self) -> String {
        // "Fri Jun 22 2018"
        format!("{}", self.offset.from_utc_datetime(&self.utc).format("%a %b %m %Y")).into()
    }
    pub fn toTimeString(&self) -> String {
        // "08:54:54 GMT+0800 (CST)"
        format!("{}", self.offset.from_utc_datetime(&self.utc).format("%H:%M:%S GMT%z (%Z)")).into()
    }
    pub fn toGMTString(&self) -> String {
        self.toISOString()
    }

    pub fn toISOString(&self) -> String {
        self.offset.from_utc_datetime(&self.utc).to_rfc3339().into()
    }
    
    pub fn toJSON(&self) -> String {
        self.toISOString()
    }
    
    pub fn toLocaleDateString(&self) -> String {
        // "6/22/2018"
        self.toDateString()
    }
    pub fn toLocaleTimeString(&self) -> String {
        // "8:54:54 AM"
        self.toTimeString()
    }

    pub fn toString(&self) -> String {
        // "Fri Jun 22 2018 08:54:54 GMT+0800 (CST)"
        format!("{} {}", self.toDateString(), self.toTimeString()).into()
    }

    pub fn toLocaleString(&self) -> String {
        // "6/22/2018, 8:54:54 AM"
        format!("{} {}", self.toLocaleDateString(), self.toLocaleTimeString()).into()
    }
}


impl hash::Hash for Date {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.get_milliseconds().hash(state)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> cmp::Ordering {
        self.get_milliseconds().cmp(&other.get_milliseconds())
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.toISOString())
    }
}


#[test]
fn test() {
    
}
