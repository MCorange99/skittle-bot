use color_eyre::Result;
use serenity::model::Timestamp;


#[allow(dead_code)]
pub fn get_time_len(duration: &str) -> Result<Timestamp> {
    let duration = duration.to_lowercase();
    if duration.ends_with('s') {
        let duration = duration.strip_suffix('s').unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs)?
        )
    }
    if duration.ends_with('m') {
        let duration = duration.strip_suffix('m').unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60)?
        )
    }

    if duration.ends_with('h') {
        let duration = duration.strip_suffix('h').unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60 * 60)?
        )
    }

    if duration.ends_with('d') {
        let duration = duration.strip_suffix('d').unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60 * 60 * 24)?
        )
    }

    if duration.ends_with('w') {
        let duration = duration.strip_suffix('w').unwrap();
        let secs = duration.parse::<i64>()?;
        return Ok(
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + secs * 60 * 60 * 24 * 7)?
        )
    }

    Err(color_eyre::eyre::eyre!("No time detected"))
}

pub fn parse_user_id(str_id: &String) -> Result<i64> {
    use color_eyre::eyre::bail;
    if str_id.len() == 18 { // id
        let parsed = str_id.parse::<u64>();
        match parsed {
            Ok(id) => Ok(id as i64),
            Err(_) => bail!("Failed to parse user id"),
        }

    } else if str_id.len() == 21 {// Mention
        let parsed = str_id[2..str_id.len()-1].parse::<u64>();
        match parsed {
            Ok(id) => Ok(id as i64),
            Err(_) => bail!("Failed to parse user mention id"),
        }

    } else {
        bail!("Failed to parse user id (bad len)")
    }
}