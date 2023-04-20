use std::collections::{HashMap};
use std::convert::Into;
use std::sync::Arc;
use std::time::Duration;
use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use actix_web_lab::__reexports::futures_util::future::join_all;
use actix_web_lab::sse;
use actix_web_lab::sse::{Data, Event};
use bytestring::ByteString;
use chrono::{Utc};
use diesel::{insert_into, PgConnection, r2d2, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use crate::db::models::Message;
use crate::db::schema::messages::dsl::messages;

const PING: sse::Event = Event::Comment(ByteString::from_static("ping"));
const HEARTBEAT: Duration = Duration::from_secs(5);
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(actix::Message)]
#[rtype("()")]
pub struct SendMessage(pub String, pub usize, pub String);

#[derive(actix::Message)]
#[rtype("()")]
pub struct Connect(pub Session);

#[derive(actix::Message)]
#[rtype("()")]
pub struct Disconnect(pub usize, pub String, pub Uuid);

#[derive(Serialize)]
pub struct Session{
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub user_id: usize,
    pub name: String,
    #[serde(skip_serializing)]
    pub(crate) channel: sse::Sender
}

impl Session {
    async fn send_event(&self, event: impl Into<sse::Event>) -> Result<(),sse::SendError>{
        self.channel.send(event).await
    }
    fn is_alive(&self) -> bool{
        if !matches!(self.channel.try_send(PING), Err(_)){
            true
        } else { false }
    }
}

#[derive(Clone)]
pub struct ChatBroadcaster{
    pub sessions: HashMap<usize,Vec<Arc<Session>>>,
    pub db: DbPool,
}

impl Actor for ChatBroadcaster {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }
}

impl Handler<Connect> for ChatBroadcaster{
    type Result = <Connect as actix::Message>::Result;

    fn handle(&mut self, Connect(session): Connect, ctx: &mut Self::Context) -> Self::Result {
        if self.sessions.contains_key(&session.user_id){
            self.sessions
                .get_mut(&session.user_id)
                .unwrap()
                .push(Arc::new(session));
        } else {
            let event = sse::Event::Data(Data::new_json(&session).unwrap().event("Connection"));
            self.broadcast(ctx, event);
            self.sessions.insert(session.user_id,vec![Arc::new(session)]);
        }
    }
}

impl Handler<Disconnect> for ChatBroadcaster{
    type Result = <Connect as actix::Message>::Result;

    fn handle(&mut self, Disconnect(user_id, user_name, session_uuid): Disconnect, ctx: &mut Self::Context) -> Self::Result {
        let mut is_empty = false;
        if let Some(session) = self.sessions.get_mut(&user_id){
            session.retain(|x| x.id != session_uuid);
            is_empty = session.is_empty();
        }
        if is_empty{
            if let Some(_) = self.sessions.remove(&user_id){
                let event = sse::Event::Data(Data::new_json(json!{ {"user_id": user_id, "name": user_name} }).unwrap().event("Disconnection"));
                self.broadcast(ctx, event);
            }
        }
    }
}

impl Handler<SendMessage> for ChatBroadcaster{
    type Result = <Connect as actix::Message>::Result;

    fn handle(&mut self, SendMessage(text, user_id, user_name): SendMessage, ctx: &mut Self::Context) -> Self::Result {
        let mut db = self.db.get().unwrap();

        if let Ok(message) = insert_into(messages)
            .values(Message {
                id: None,
                creation_date: Utc::now().naive_utc(),
                user_id: user_id as i32,
                user_name,
                text
            })
            .get_result::<Message>(&mut db)
        {
            let event = sse::Event::Data(Data::new_json(&message).unwrap().event("Message"));
            self.broadcast(ctx, event);
        }
    }
}

impl ChatBroadcaster {

    fn broadcast(&self, ctx: &mut <Self as Actor>::Context, event: sse::Event){
        let broadcaster = self.clone();

        ctx.spawn(async move {
            join_all(broadcaster
                .sessions
                .values()
                .flatten()
                .map(|session| session.send_event(event.clone()))
            )
            .await;
        }.into_actor(self));
    }

    fn heartbeat(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT, |act, ctx| {
            let addr = ctx.address();
            act
                .sessions
                .values()
                .flatten()
                .filter(|session| !session.is_alive())
                .for_each(|session| addr.do_send(Disconnect(session.user_id, session.name.clone(), session.id)));
        });
    }
}

