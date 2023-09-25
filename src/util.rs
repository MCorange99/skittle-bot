use color_eyre::Result;
use serenity::model::Timestamp;

pub fn get_time_len(duration: String) -> Result<Timestamp> {
    let duration = duration.to_lowercase();
    if duration.ends_with("s") {
        let duration = duration.strip_suffix("s").unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs)?
        )
    }
    if duration.ends_with("m") {
        let duration = duration.strip_suffix("m").unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60)?
        )
    }

    if duration.ends_with("h") {
        let duration = duration.strip_suffix("h").unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60 * 60)?
        )
    }

    if duration.ends_with("d") {
        let duration = duration.strip_suffix("d").unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60 * 60 * 24)?
        )
    }

    if duration.ends_with("w") {
        let duration = duration.strip_suffix("w").unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60 * 60 * 24 * 7)?
        )
    }

    Err(color_eyre::eyre::eyre!("No time detected"))
}

