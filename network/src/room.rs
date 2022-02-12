use std::collections::HashMap;

use super::message::{ServerMessage, UserId};
use super::proto::deserialize_client_message;
use super::proto::message::client_send::ClientSendOneof;
use super::proto::message::DisplayResponse;
use actix::Recipient;
use xlrs_controller::controller::display::DisplayRequest;
use xlrs_controller::controller::edit_action::ActionEffect;
use xlrs_controller::controller::Controller;

pub struct Room {
    pub users: HashMap<UserId, Recipient<ServerMessage>>,
    pub wb: Controller,
    pub history: Vec<u32>, // transactions
}

impl Room {
    pub fn get_response(&mut self, msg: ClientSendOneof) -> Option<ClientResponse> {
        match msg {
            ClientSendOneof::Transaction(t) => {
                let undoable = t.undoable;
                let action = t.into();
                log!("Get Action from client: {:?}", &action);
                if let Some(e) = self.wb.handle_action(action, undoable) {
                    Some(ClientResponse::ActionEffect(e))
                } else {
                    todo!()
                }
            }
            ClientSendOneof::DisplayRequest(req) => {
                let idx = req.sheet_idx as usize;
                let dq = DisplayRequest {
                    sheet_idx: idx,
                    version: req.version,
                };
                let res = self.wb.get_display_response(dq);
                let r = DisplayResponse::from(res);
                Some(ClientResponse::Display(r))
            }
        }
    }

    pub fn get_client_msg_response(&mut self, content: Vec<u8>) -> Option<ClientResponse> {
        match deserialize_client_message(content) {
            Ok(msg) => match msg.client_send_oneof {
                Some(m) => match m {
                    ClientSendOneof::Transaction(t) => {
                        let undoable = t.undoable;
                        let action = t.into();
                        log!("Get Action from client: {:?}", &action);
                        if let Some(e) = self.wb.handle_action(action, undoable) {
                            Some(ClientResponse::ActionEffect(e))
                        } else {
                            todo!()
                        }
                    }
                    ClientSendOneof::DisplayRequest(req) => {
                        let idx = req.sheet_idx as usize;
                        let dq = DisplayRequest {
                            sheet_idx: idx,
                            version: req.version,
                        };
                        let res = self.wb.get_display_response(dq);
                        let r = DisplayResponse::from(res);
                        Some(ClientResponse::Display(r))
                    }
                },
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn has_user(&self, user: &UserId) -> bool {
        self.users.contains_key(user)
    }
}

pub enum ClientResponse {
    Display(DisplayResponse),
    ActionEffect(ActionEffect),
}