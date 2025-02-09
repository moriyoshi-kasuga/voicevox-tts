pub mod dict;
pub mod join;
pub mod leave;

macro_rules! only_guild {
    ($ctx:ident,$var:ident) => {
        only_guild!($ctx, $var, $var);
    };
    ($ctx:ident,$func:ident,$var:ident) => {
        let Some($var) = $ctx.$func() else {
            $ctx.reply("Guild内でしか使えません").await?;
            return Ok(());
        };
    };
}

pub(super) use only_guild;
