#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

impl Period {
    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
}

impl ::std::str::FromStr for Period {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use self::Period::*;

        match s {
            "d" => Ok(Day),
            "w" => Ok(Week),
            "m" => Ok(Month),
            "y" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl ::std::fmt::Display for Period {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Period::*;

        let s = match *self {
            Day => "d",
            Week => "w",
            Month => "m",
            Year => "y",
        };

        f.write_str(s)?;

        Ok(())
    }
}

impl ::std::ops::Add<::chrono::NaiveDate> for Period {
    type Output = ::chrono::NaiveDate;

    fn add(self, rhs: Self::Output) -> Self::Output {
        use self::Period::*;
        use chrono::Datelike;

        let mut y = rhs.year();
        let mut m = rhs.month();
        let mut d = rhs.day();

        match self {
            Year => y += 1,
            Month => m += 1,
            Week => d += 7,
            Day => d += 1,
        };

        let max_days = if m == 2 {
            if Period::is_leap_year(y) {
                29
            } else {
                28
            }
        } else if [1, 3, 5, 7, 8, 10, 12].contains(&m) {
            31
        } else {
            30
        };

        if d > max_days {
            m += 1;
            d -= max_days;
        }

        if m > 12 {
            y += 1;
            m -= 12;
        }

        ::chrono::NaiveDate::from_ymd(y, m, d)
    }
}
