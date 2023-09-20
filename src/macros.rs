#[macro_export]
macro_rules! read_type_map {
    ($type:ty, $ctx:expr) => {
        {
            let data_read = $ctx.data.try_write()?;
            data_read.get::<$type>().unwrap().clone()
        }
    };
}

#[macro_export]
macro_rules! get_type_map_data_ro_cloned {
    ($type:ty, $ctx:expr) => {
        {
            let lock = {
                let data_read = $ctx.data.read().await;
                data_read.get::<$type>().unwrap().clone()
            };
            
            let data = lock.read().await;
            data.clone()
        }
    };
}


#[macro_export]
macro_rules! with_db {
    ($ctx:expr, $body:expr) => {
        {   
            use crate::read_type_map;
            use crate::db::Database;

            let db_lock = read_type_map!(Database, $ctx);
            let db_lock = db_lock.lock();
            let db = match db_lock {
                Ok(db) => db,
                Err(_) => {
                    log::error!("Unable to access database lock");
                    return Ok(());
                }
            };

            let mut conn = match db.connection_pool.get() {
                Ok(p) => p,
                Err(_) => {
                    log::error!("Unable to access db connection from pool");
                    return Ok(());
                }
            };

            match $body(&mut conn) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into())
            }
        }
    };
}


#[macro_export]
macro_rules! try_parse_args {
    ($type:ty, $msg:expr, $ctx:expr, $args:expr) => {
        {
            match <$type>::try_parse_from($args) {
                Ok(r) => r,
                Err(e) => {
                    $msg.reply_ping(&$ctx.http, format!("```{}```", e.render())).await?;
                    return Ok(());
                }
            }

        }
    };
}


#[macro_export]
macro_rules! internal_error {
    ($msg:expr, $ctx:expr, $err:expr) => {
        $msg.reply_ping(&$ctx.http, format!("{}{}{}",concat!(
            "```md\n",
            "# Internal error\n",
            "If you are an administrator, please check the logs for more info.\n",
            "Otherwise, contact MCorange (@mcorange)\n\n",
            "## Debug info\n"),
            $err.to_string(),
            "```")).await?;
        log::error!("Internal error: {}", $err.to_string());
        return Ok(())
    };
}


#[macro_export]
macro_rules! get_guild_members {
    ($ctx:expr, $guild:expr) => {
        {
            let mut last_member: Option<UserId> = None;
            
            let mut member_list: Vec<Member> = Vec::new();
            loop {
                let mut members = $guild.members(&$ctx.http, None, last_member).await?;
                if members.is_empty() {
                    break;
                }
                member_list.append(&mut members);
                last_member = Some(member_list.last().unwrap().user.id);
            }
            member_list
        }
    };
}
            