use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

pub struct OnlinePlugin;

#[derive(Serialize, Deserialize)]
enum Event {
    ChatMessage(String),
}

#[derive(Serialize, Deserialize)]
struct EventPacket {
    event: Event,
}

impl Plugin for OnlinePlugin {
    fn build(&self, app: &mut App) {
        let socket = MatchboxSocket::new_reliable("ws://127.0.0.1:3536");
        let room = Room { socket };
        app.insert_resource(room)
            .add_systems(Update, (send_events, receive_events, update_room));
    }
}

#[derive(Resource)]
struct Room {
    socket: MatchboxSocket<SingleChannel>,
}

fn send_events(mut _room: ResMut<Room>) {}

fn receive_events(mut room: ResMut<Room>) {
    if !room.is_ok() {
        return;
    }
    for EventPacket { event } in room.receive().iter() {
        match event {
            Event::ChatMessage(message) => info!(message),
        }
    }
}

fn update_room(mut room: ResMut<Room>) {
    for (peer, state) in room.socket.update_peers() {
        match state {
            PeerState::Connected => info!("player {peer:?} connected"),
            PeerState::Disconnected => info!("player {peer:?} disconnected"),
        }
    }
}

impl Room {
    fn send(&mut self, event: Event) {
        let event_packet = EventPacket { event };
        let peers: Vec<_> = self.socket.connected_peers().collect();
        for peer in peers {
            self.socket.send(
                bincode::serialize(&event_packet)
                    .expect("failed to convert event_packet to binary encoding")
                    .into(),
                peer,
            );
        }
    }

    fn receive(&mut self) -> Vec<EventPacket> {
        self.socket
            .receive()
            .into_iter()
            .filter_map(|(_, packet)| bincode::deserialize(&packet[..]).ok())
            .collect()
    }

    fn is_ok(&self) -> bool {
        self.socket.connected_peers().count() > 0
    }
}
