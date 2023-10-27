mod classes;
mod clients;
mod common;
mod enums;
mod functions;
mod variants;

use super::context::Context;

pub(super) fn validate(ctx: &mut Context<'_>) {
    enums::validate(ctx);
    classes::validate(ctx);
    variants::validate(ctx);
    functions::validate(ctx);
    clients::validate(ctx);
}