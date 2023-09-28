
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;
use std::fmt::Write;
use clap::{Parser, arg};
use diesel::*;

use crate::db::{schema::core_users::dsl::*, models::CoreUsers};
use crate::{modules::{SkittleModuleCommand, SkittleModuleCommandBuilder, Stdout}, db::models::UserRole, util::parse_user_id};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("ping", "check if bot is responsive")
        ])
        .dev_only(false)
        .required_user_roles(vec![
            UserRole::Administrator
        ])
        .required_bot_permissions(vec![])
        .build()
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short='u', long="user", help="User id or mention")]
    user: String,
    #[arg(short='a', long="add", help="Add a role to be added")]
    add: Vec<String>,
    #[arg(short='r', long="remove", help="Add role to be removed")]
    remove: Vec<String>
}

pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        let mut stdout = Stdout::new(&msg, &ctx);
        let args = try_parse_args!(Args, msg, ctx, args);


        let uid = match parse_user_id(&args.user){
            Ok(i) => i,
            Err(e) => {
                writeln!(stdout, "{e}")?;
                stdout.flush().await?;
                return Ok(());
            }
        };


        let user: CoreUsers = {
            let mut conn = get_db!(ctx);

            core_users
                .select(CoreUsers::as_select())
                .find(uid)
                .first(&mut conn)?
        };

        let mut uroles = user.user_role;

        let dc_user = ctx.http.get_member(msg.guild_id.unwrap().0, uid as u64).await?;

        for add in args.add {
            let role: UserRole = add.try_into()?;
            uroles = UserRole::add(uroles, role);

            writeln!(stdout, "Added {:?} to {}({})", role, dc_user.display_name(), uid)?;
        }


        for remove in args.remove {
            let role: UserRole = remove.try_into()?;
            uroles = UserRole::remove(uroles, role);

            writeln!(stdout, "Removed {:?} from {}({})", role, dc_user.display_name(), uid)?;
        }

        
        {
            let mut conn = get_db!(ctx);
            
            diesel::update(core_users.filter(user_id.eq(uid)))
                .set(user_role.eq(uroles))
                .execute(&mut conn)?;
        }

        writeln!(stdout, "User saved!")?;
        stdout.flush().await?;
        Ok(())
    }.boxed()
}