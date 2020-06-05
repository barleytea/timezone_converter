use chrono::{NaiveDateTime, TimeZone, Datelike, Timelike};
use chrono_tz::Tz;
use failure::Error;
use structopt::StructOpt;
use atty::Stream;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "TIME")]
    pub time: Option<String>,
    #[structopt(short = "f", long = "fromtz")]
    pub fromtz: Option<String>,
    #[structopt(short = "t", long = "totz")]
    pub totz: Option<String>,
}

fn is_stdin(input: Option<&String>) -> bool {
    let is_request = match input {
        Some(i) if i == "-" => true,
        _ => false,
    };

    let is_pipe =! atty::is(Stream::Stdin);
    is_request || is_pipe
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let time = opt.time;
    let fromtz = opt.fromtz;
    let totz = opt.totz;

    if (time.is_none() || fromtz.is_none() || totz.is_none())
            && (! is_stdin(time.as_ref()) || ! is_stdin(fromtz.as_ref()) || ! is_stdin(totz.as_ref())) {
        Opt::clap().print_help()?;
        std::process::exit(1);
    }

    Ok(println!("{}", convert(&time.unwrap(), &fromtz.unwrap(), &totz.unwrap())?))
}

fn convert(date: &str, from_tz_str: &str, to_tz_str: &str) -> Result<String> {
    let from_tz: Tz = from_tz_str.parse::<Tz>().unwrap();
    let to_tz: Tz = to_tz_str.parse::<Tz>().unwrap();
    let native_date: NaiveDateTime = NaiveDateTime::parse_from_str(date, "%Y/%m/%d %H:%M:%S").unwrap();
    let local_date = from_tz.ymd(native_date.year(), native_date.month(), native_date.day())
        .and_hms(native_date.hour(), native_date.minute(), native_date.second());
    Ok(local_date.with_timezone(&to_tz).format("%Y/%m/%d %H:%M:%S").to_string())
}

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn tokyo_to_london_ok() {
        let expected = "2019/12/07 10:31:28";
        let actual = convert("2019/12/07 19:31:28", "Asia/Tokyo", "Europe/London").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn london_to_tokyo_ok() {
        let expected = "2019/12/07 19:31:28";
        let actual = convert("2019/12/07 10:31:28", "Europe/London", "Asia/Tokyo").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokyo_to_chicago_ok() {
        // out of summer time in Chicago
        let expected = "2019/12/07 04:31:28";
        let actual = convert("2019/12/07 19:31:28", "Asia/Tokyo", "America/Chicago").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn chicago_to_tokyo_ok() {
        // out of summer time in Chicago
        let expected = "2019/12/07 19:31:28";
        let actual = convert("2019/12/07 04:31:28", "America/Chicago", "Asia/Tokyo").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn tokyo_to_invalid_ng() {
        convert("2019/12/07 19:31:28", "Asia/Tokyo", "Hoge/Fuga").unwrap();
    }
}
