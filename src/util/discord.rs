use poise::serenity_prelude::Member;

pub fn get_user_read_name(member: &Member) -> &str {
    if let Some(name) = &member.nick {
        return name;
    }
    if let Some(name) = &member.user.global_name {
        return name;
    }
    &member.user.name
}

pub fn is_human(member: &Member) -> bool {
    !member.user.bot && !member.user.system
}
