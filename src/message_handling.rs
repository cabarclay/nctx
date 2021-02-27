use std::convert::Infallible;
use teloxide::prelude::*;

use crate::states::Dialogue;

type In = DialogueWithCx<Message, Dialogue, Infallible>;

pub async fn run() {
    // create new bot
    let bot = Bot::builder().build();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |DialogueWithCx { cx, dialogue }: In| async move {
                let dialogue = dialogue.expect("std::convert::Infallible");
                handle_message(cx, dialogue).await.expect("fuck")
            },
        ))
        .dispatch()
        .await;
}

async fn handle_message(cx: UpdateWithCx<Message>, dialogue: Dialogue) -> TransitionOut<Dialogue> {
    match cx.update.text_owned() {
        None => next(dialogue),
        Some(ans) => dialogue.react(cx, ans).await,
    }
}
