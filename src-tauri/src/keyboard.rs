use tokio::sync::mpsc::UnboundedSender;
use willhook;

use crate::UpdateType;

pub async fn init(tx: &mut UnboundedSender<UpdateType>) {
    let kb_hook = willhook::willhook().unwrap();

    loop {
        if let Ok(ie) = kb_hook.try_recv() {
            match ie {
                willhook::InputEvent::Keyboard(ke) => {
                    let pressed = ke.pressed;
                    match pressed {
                        willhook::KeyPress::Up(_) => {
                            //let sessions_data = get_sessions_data(manager_clone).unwrap();
                        },
                        willhook::KeyPress::Down(_) => {
                            if ke.key.is_some() {
                                let key = ke.key.unwrap();

                                match key {
                                    willhook::KeyboardKey::Other(173) => {tx.send(UpdateType::KeyboardUpdate).ok();}
                                    willhook::KeyboardKey::Other(174) => {tx.send(UpdateType::KeyboardUpdate).ok();},
                                    willhook::KeyboardKey::Other(175) => {tx.send(UpdateType::KeyboardUpdate).ok();},
                                    willhook::KeyboardKey::Other(176) => {tx.send(UpdateType::KeyboardUpdate).ok();},
                                    willhook::KeyboardKey::Other(177) => {tx.send(UpdateType::KeyboardUpdate).ok();},
                                    willhook::KeyboardKey::Other(178) => {tx.send(UpdateType::KeyboardUpdate).ok();},
                                    willhook::KeyboardKey::Other(179) => {tx.send(UpdateType::KeyboardUpdate).ok();},
                                    _ => {}
                                }
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        } else {
            std::thread::yield_now();
        }
    }
}