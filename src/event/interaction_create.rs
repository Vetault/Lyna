use crate::{command::ping::Ping, Context};
use anyhow::Result;
use sparkle_convenience::{
    error::IntoError,
    interaction::{extract::InteractionExt, InteractionHandle},
    reply::Reply,
};
use twilight_gateway::Latency;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::interaction::Interaction;

#[derive(Debug)]
pub struct InteractionContext<'ctx> {
    pub context: &'ctx Context,
    pub handle: InteractionHandle<'ctx>,
    pub interaction: Interaction,
}

impl Context {
    pub async fn interaction_create(
        &self,
        interaction: Interaction,
        latency: Latency,
    ) -> Result<()> {
        let handle = &self.bot.interaction_handle(&interaction);
        let interaction_ctx = InteractionContext {
            context: self,
            handle: handle.clone(),
            interaction,
        };

        let res = match interaction_ctx.interaction.name().ok()? {
            Ping::NAME => interaction_ctx.execute_ping(latency).await,
            _ => Ok(()),
        };

        /*         if let Err(err) = res {
            if err.ignore() {
                return Ok(());
            }

            if let Err(_) = handle.followup(error_reply(&err)).await {
                if let Err(Some(internal_err)) = handle
                    .reply(error_reply(&err))
                    .await
                    .map_err(|_err| None /* err.internal() */)
                {
                    return Err(internal_err);
                }
            }
        } */
        Ok(())
    }
}

trait Test {
    fn embed(&self) -> String;
}

impl Test for Reply {
    fn embed(&self) -> String {
        "test".to_string()
    }
}
