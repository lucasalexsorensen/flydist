use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn into_response(self, payload: Payload) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            body: Body {
                in_reply_to: self.body.msg_id,
                msg_id: self.body.msg_id,
                payload,
            },
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate {},
    GenerateOk {
        id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_deserialization() {
        let raw_msg = r#"
        {
            "src": "c1",
            "dest": "n1",
            "body": {
                "type": "echo",
                "msg_id": 1,
                "echo": "Please echo 35"
            }
        }
        "#;

        let msg: Message = serde_json::from_str(raw_msg).unwrap();
        assert_eq!(msg.src, "c1");
        assert_eq!(msg.dest, "n1");
        assert_eq!(msg.body.msg_id, Some(1));
        assert_eq!(msg.body.in_reply_to, None);
        assert_eq!(
            msg.body.payload,
            Payload::Echo {
                echo: "Please echo 35".to_string()
            }
        );
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message {
            src: "n1".to_string(),
            dest: "c1".to_string(),
            body: Body {
                msg_id: Some(1),
                in_reply_to: Some(1),
                payload: Payload::EchoOk {
                    echo: "Please echo 35".to_string(),
                },
            },
        };

        let raw_msg = serde_json::to_string(&msg).unwrap();
        assert_eq!(
            raw_msg,
            r#"{"src":"n1","dest":"c1","body":{"msg_id":1,"in_reply_to":1,"type":"echo_ok","echo":"Please echo 35"}}"#
        );
    }
}
