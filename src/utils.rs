use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::prelude::*;

pub async fn get_send_time() -> String {
    let fmt = "%Y-%m-%d %H:%M:%S";

    let now: DateTime<Local> = Local::now();

    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string();
    // 2021-01-04 20:02:09
    str_date
}

pub fn get_unix() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}

pub fn get_unix_nano() -> i128 {
    time::OffsetDateTime::now_utc().unix_timestamp_nanos()
}
