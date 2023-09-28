use crate::db::models::UserRole;


pub fn missing_roles(roles: Vec<UserRole>) -> String {
    let mut s = concat!(
        "```md\n",
        "# Error\n",
        "You do not have the required roles:\n"
    ).to_string();

    for role in roles {
        s.push_str(&format!(" - {role:?}\n"));
    }
    s.push_str("\nIf you think this is an error contact @mcorange\n```");
    s
}


pub fn failed_parsing(command: &String) -> String {
    format!(concat!(
        "```md\n",
        "# Error\n",
        "Failed to parse command {:?}\n\n",
        "If you think this is an error contact @mcorange\n",
        "```\n"
    ), command)

}

pub const NOT_DEVELOPER: &str = concat!(
    "```md\n",
    "# Error\n",
    "You are not a developer\n\n",
    "If you think this is an error please contact @mcorange\n",
    "```\n"
);