
pub enum Message {
    Snapshot,
    Input
}

pub struct Snapshot {

}

pub struct Input {

}

// fn send(stream, &Message) {
//     serialize + frame + write
// }           

// fn recv(stream) -> Message {
//     read frame + deserialize
// }