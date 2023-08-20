use serde::Serialize;

#[derive(Debug, Serialize)]
struct SimpleDuration {
    hours: i64,
    minutes: i64,
    seconds: i64,
}

#[derive(Debug, Serialize)]
pub struct Duration {
    part: SimpleDuration,
    total: SimpleDuration,
}

impl From<time::Duration> for Duration {
    fn from(duration: time::Duration) -> Self {
        const MINUTES_PER_HOUR: i64 = 60;
        const SECONDS_PER_MINUTE: i64 = 60;
        const SECONDS_PER_HOUR: i64 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;

        let secs = duration.whole_seconds();
        let minutes = secs / SECONDS_PER_MINUTE;
        let hours = secs / SECONDS_PER_HOUR;

        let part = SimpleDuration {
            hours,
            minutes: minutes % MINUTES_PER_HOUR,
            seconds: secs % SECONDS_PER_MINUTE,
        };

        let total = SimpleDuration {
            hours,
            minutes,
            seconds: secs,
        };

        Duration { part, total }
    }
}
