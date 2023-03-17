use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect;
