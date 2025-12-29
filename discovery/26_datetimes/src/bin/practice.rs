use chrono::prelude::{Local, Utc};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use chrono_tz::Indian::Comoro;
use std::ops::{Add, Sub};

fn main() {
    // creating a new naive date stuct
    let birthday = NaiveDate::from_ymd_opt(2003, 11, 22);
    println!("my birthdate is {:?}", birthday.unwrap());

    // parsing a &str to a naive date struct
    let str_date = "2003-11-22";
    println!(
        "{:?}",
        str_date
            .parse::<NaiveDate>()
            .expect("Date format incorrect")
    );

    // stating some time delta structs
    let five_seconds = TimeDelta::new(5, 0);
    let negative_five_seconds = TimeDelta::new(-5, 0);
    println!("{:?}", five_seconds.unwrap());
    println!("{:?}", negative_five_seconds.unwrap());

    // using other constructors for the time delta struct
    let five_minutes = TimeDelta::minutes(5);
    println!("{:?}", five_minutes);

    // using some associated functions of the structs
    let five_weeks = TimeDelta::weeks(5);
    println!("five weeks has {} days", five_weeks.num_days());

    // adding multiple time deltas
    println!(
        "five weeks plus five minutes has {:?} minutes",
        (five_weeks + five_minutes).num_minutes()
    );

    println!(
        "one week before my birthday is: {:?}",
        birthday.unwrap().sub(TimeDelta::weeks(1))
    );

    println!(
        "one week after my birthday is: {:?}",
        birthday.unwrap().add(TimeDelta::weeks(1))
    );

    // 24 hour clock used in Time
    let four_thrity_am = NaiveTime::from_hms_opt(4, 30, 0);
    println!("{:?}", four_thrity_am.unwrap());

    let birthdate_and_time = NaiveDateTime::new(birthday.unwrap(), four_thrity_am.unwrap());
    println!("{:?}", birthdate_and_time);
    println!(
        "3 days before my birthday: {:?}",
        birthdate_and_time - TimeDelta::days(3)
    );

    // getting sys time
    let system_time = Local::now();
    println!("system time is: {}", system_time);
    println!("system time is: {}", system_time.time());
    println!("system time is: {}", system_time.day());
    println!("naive time: {}", system_time.date_naive());

    // getting utc time
    let utc_time = Utc::now();
    println!("time in UTC is: {}", utc_time);
    println!("time in UTC is: {}", utc_time.time());
    println!("time in UTC is: {}", utc_time.month());
    println!("naive time: {}", utc_time.date_naive());

    // geting tz offset
    println!("diff from systime to utc is: {:?}", system_time.offset());

    // converting time using TZ
    println!("Indian time is: {}", system_time.with_timezone(&Comoro));

    // converting &str to DateTime
    let foo_day = "31-oct-1995 18:07:45 -0600";
    let dt = DateTime::parse_from_str(foo_day, "%d-%b-%Y %H:%M:%S %z");
    println!("{:?}", dt);

    // coverting DateTime to &str
    let system = Local::now();
    println!("{}", system.format("%m-%d-%Y %H:%M:%S"));
    println!("{}", system.format("%m/%d/%y"));
}
