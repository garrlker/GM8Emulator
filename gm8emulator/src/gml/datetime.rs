use crate::{gml::Value, math::Real};
use chrono::{self, Datelike, NaiveDate, NaiveDateTime, Timelike};
use std::time;

/// Sleep for T minus 1 millisecond, and busywait for the rest of the duration.
pub fn sleep(dur: time::Duration) {
    // TODO: find a more precise way to sleep?
    let begin = time::Instant::now();
    if let Some(sleep_time) = dur.checked_sub(time::Duration::from_millis(1)) {
        std::thread::sleep(sleep_time);
    }
    while time::Instant::now() < begin + dur {}
}

fn epoch() -> NaiveDateTime {
    NaiveDate::from_ymd(1899, 12, 30).and_hms(0, 0, 0)
}

pub struct DateTime(NaiveDateTime);

impl DateTime {
    pub fn now_or_nanos(nanos: Option<u128>) -> Self {
        if let Some(nanos) = nanos {
            Self(NaiveDateTime::from_timestamp((nanos / 1_000_000_000) as i64, (nanos % 1_000_000_000) as u32))
        } else {
            Self(chrono::Local::now().naive_local())
        }
    }

    pub fn date(&self) -> Self {
        Self(self.0.date().and_hms(0, 0, 0))
    }

    pub fn time(&self) -> Self {
        Self(epoch().date().and_time(self.0.time()))
    }

    pub fn from_ymd(y: i32, m: i32, d: i32) -> Option<Self> {
        // GM doesn't support BC so we won't either
        if y > 0 && m > 0 && d > 0 {
            NaiveDate::from_ymd_opt(y, m as u32, d as u32).map(|d| Self(d.and_hms(0, 0, 0)))
        } else {
            None
        }
    }

    pub fn from_hms(h: i32, m: i32, s: i32) -> Option<Self> {
        if h >= 0 && m >= 0 && s >= 0 {
            epoch().date().and_hms_opt(h as u32, m as u32, s as u32).map(|dt| Self(dt))
        } else {
            None
        }
    }

    pub fn from_ymdhms(y: i32, mo: i32, d: i32, h: i32, mi: i32, s: i32) -> Option<Self> {
        if y >= 0 && mo >= 0 && d >= 0 && h >= 0 && mi >= 0 && s >= 0 {
            NaiveDate::from_ymd_opt(y, mo as u32, d as u32)
                .and_then(|d| d.and_hms_opt(h as u32, mi as u32, s as u32).map(Self))
        } else {
            None
        }
    }

    pub fn year(&self) -> i32 {
        self.0.date().year()
    }

    pub fn month(&self) -> u32 {
        self.0.date().month()
    }

    pub fn day(&self) -> u32 {
        self.0.date().day()
    }

    pub fn day_of_year(&self) -> u32 {
        self.0.date().ordinal()
    }

    pub fn hour_of_year(&self) -> u32 {
        (self.day_of_year() - 1) * 24 + self.0.time().hour()
    }

    pub fn minute_of_year(&self) -> u32 {
        self.hour_of_year() * 60 + self.0.time().minute()
    }

    pub fn second_of_year(&self) -> u32 {
        self.minute_of_year() * 60 + self.0.time().second()
    }

    pub fn hour(&self) -> u32 {
        self.0.time().hour()
    }

    pub fn minute(&self) -> u32 {
        self.0.time().minute()
    }

    pub fn second(&self) -> u32 {
        self.0.time().second()
    }

    pub fn week(&self) -> u32 {
        self.0.iso_week().week()
    }

    pub fn weekday(&self) -> u32 {
        self.0.weekday().number_from_sunday()
    }
}

impl From<DateTime> for Real {
    fn from(dt: DateTime) -> Self {
        // calculate the ipart and fpart separately for maybe better precision?
        let ipart = Real::from((dt.0 - epoch()).num_days() as f64);
        let fpart = Real::from((dt.time().0 - epoch()).num_milliseconds() as f64) / Real::from(86400000);
        // the time part is the abs(fract()) of the datetime so that part increases backwards before the epoch
        if dt.0 >= epoch() { ipart + fpart } else { ipart - 1.into() - fpart }
    }
}

impl From<DateTime> for Value {
    fn from(dt: DateTime) -> Self {
        Real::from(dt).into()
    }
}

impl From<Real> for DateTime {
    fn from(dt: Real) -> Self {
        let days = chrono::Duration::days(dt.trunc().round().into());
        let ms = chrono::Duration::milliseconds((dt.fract() * Real::from(86400000)).floor().round().into());
        // negate the time (see the inverse function for explanation)
        Self(epoch() + days + if dt > 0.into() { ms } else { -ms })
    }
}
