use crate::states::*;

use sqlx::sqlite::SqlitePool;
use sqlx::{Pool, Sqlite};
use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use teloxide::prelude::*;
use teloxide::types::ForwardKind::NonChannel;
use teloxide::types::MessageKind::Common;
use teloxide_macros::teloxide;

#[teloxide(subtransition)]
async fn ready(_state: ReadyState, cx: TransitionIn, ans: String) -> TransitionOut<Dialogue> {
    let ans = ans.as_str();
    match ans {
        "/nocontext" | "/nctx" => {
            log::info!("recieved /nctx");

            let pool = db().await;

            if let Err(_) = pool {
                log::error!("error connecting to pool");
            }

            let pool = pool.unwrap();

            let t = sqlx::query!("SELECT * FROM info").fetch_one(&pool).await;

            if let Err(_) = t {
                log::error!("error reading time");
            }

            let t = t.unwrap();

            let curr_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => panic!("time before unix epoch"),
            };

            let rate_limit = 2u64;

            log::info!("last message {}, curr time {}", t.last_message, curr_time);

            if curr_time < rate_limit + t.last_message as u64 {
                log::info!("attempt to send messages too fast");
                return next(ReadyState);
            }

            if let Err(_) = sqlx::query!("DELETE FROM info").execute(&pool).await {
                panic!("failed to clear info");
            }

            let ct = &(curr_time as i64);

            if let Err(_) = sqlx::query!("INSERT INTO info (last_message) VALUES (?1)", ct)
                .execute(&pool)
                .await
            {
                panic!("failed to insert info")
            }

            let row = sqlx::query!("SELECT * FROM messages ORDER BY RANDOM() LIMIT 1;")
                .fetch_one(&pool)
                .await;

            match row {
                Ok(row) => {
                    log::info!(
                        "forwarding from {} msg {}",
                        row.from_chat_id,
                        row.message_id
                    );
                    cx.bot
                        .forward_message(
                            cx.chat_id(),
                            row.from_chat_id as i64,
                            row.message_id as i32,
                        )
                        .disable_notification(true)
                        .send()
                        .await?
                }
                Err(_) => {
                    cx.answer_str("lol dun fucked up idfk at this point :)))")
                        .await?
                }
            };
        }
        _ => {
            if let Common(ref k) = cx.update.kind {
                if let NonChannel(_k) = &k.forward_kind {
                    let from_chat_id = cx.update.chat_id();
                    let message_id = cx.update.id;

                    log::info!("rec from {} fwd msg {}", from_chat_id, message_id);

                    let pool = db().await;

                    if let Err(_) = pool {
                        log::error!("error connecting to pool");
                    }

                    let pool = pool.unwrap();

                    let res = sqlx::query!(
                        "INSERT INTO messages (from_chat_id, message_id) VALUES (?1, ?2)",
                        from_chat_id,
                        message_id
                    )
                    .execute(&pool)
                    .await;

                    if let Err(_e) = res {
                        log::error!("failed to insert msg^");
                    } else {
                        log::info!("inserted msg");
                    }
                }
            }
        }
    }

    next(ReadyState)
}

async fn db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool =
        SqlitePool::connect(&env::var("DATABASE_URL").expect("could not find $DATABASE_URL")).await;

    if let Err(e) = pool {
        log::error!("error connecting to db");
        return Err(e);
    }

    let pool = pool.unwrap();

    Ok(pool)
}
