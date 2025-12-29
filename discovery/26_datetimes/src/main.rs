use chrono::{DateTime, Utc};

fn main() {
    let event_data = vec![
        (
            "2025**04**19 !! 16:00:00 -04:00",
            "Started Rust study session",
        ),
        ("2025**04**20 !! 08:05:30 -04:00", "Made breakfast"),
        ("ERR", "ERR"),
        ("2025**04**22 !! 22:10:45 -04:00", "Went to bed"),
        ("ERR", "ERR"),
        ("2025**04**25 !! 09:00:03 -04:00", "Resumed Rust study"),
    ];

    // define format pattern
    let format = "%Y**%m**%d !! %H:%M:%S %z";

    // map event_data vector and filter when parsing errors out the value is removed from the array
    // when the value is correctly converted the value is added as an UTC datetime with its message
    // then the array is collected as an Vec<(DateTime<UTC>, &str)>
    let events = event_data
        .into_iter()
        .filter_map(|(timestamp, message)| {
            let parsed_datetime = DateTime::parse_from_str(timestamp, format);
            match parsed_datetime {
                Ok(datetime) => Some((datetime.with_timezone(&Utc), message)),
                Err(_) => None,
            }
        })
        .collect::<Vec<(DateTime<Utc>, &str)>>();

    // aux variables for the loop
    let display_format = "%Y-%m-%d %H:%M:%S";
    let mut previous: Option<DateTime<Utc>> = None;
    
    // for each iteration print the information like this
    for (utc_datetime, message) in events {
        let display_time = utc_datetime.format(display_format);
        println!("Event time: {}", display_time);
        println!("Event message: {}", message);

        // if previous is none this wont run
        if let Some(previous_event) = previous {
            let difference = utc_datetime - previous_event;
            let hours = difference.num_hours();
            let minutes = difference.num_minutes() % 60;
            let seconds = difference.num_seconds() % 60;
            println!(
                "Time since previous event: {}h {}m {}s",
                hours, minutes, seconds
            );
        }

        // define a value for previous after first iteration
        previous = Some(utc_datetime);
        println!();
    }
}
