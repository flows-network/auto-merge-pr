use dotenv::dotenv;
use github_flows::{get_octo, listen_to_event, EventPayload};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use slack_flows::send_message_to_channel;
use std::env;
use tokio::*;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event(
        "jaykchen",
        "vitesse-lite",
        vec!["pull_request", "pull_request_review"],
        handler,
    )
    .await;

    Ok(())
}

// comments_url = https://api.github.com/repos/jaykchen/vitesse-lite/issues/7/comments
async fn handler(payload: EventPayload) {
    match payload {
        EventPayload::PullRequestEvent(e) => {
            send_message_to_channel(
                "ik8",
                "step_1",
                serde_json::to_string(&e).unwrap_or("pull failed".to_string()),
            );
        }

        EventPayload::PullRequestReviewEvent(e) => {
            send_message_to_channel(
                "ik8",
                "step_2",
                serde_json::to_string(&e).unwrap_or("pr review failed".to_string()),
            );
        }

        EventPayload::PullRequestReviewCommentEvent(e) => {
            send_message_to_channel(
                "ik8",
                "step_3",
                serde_json::to_string(&e).unwrap_or("pr review comment failed".to_string()),
            );
        }

        _ => (),
    }
}

// fn get_comments(owner: &str, repo: &str, pull_number: &str, review_id: &str) -> Option<String> {
//     dotenv().ok();
//     let api_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

//     let mut writer = Vec::new();

//     let url = format!(
//         "https://api.github.com/repos/{}/{}/pulls/{}/reviews/{}/comments",
// "https://api.github.com/repos/jaykchen/vitesse-lite/pulls/1259711866/reviews/1325364967/comments",
//         owner, repo, pull_number, review_id
//     );
//     let uri = Uri::try_from(&url).unwrap();

//     let bearer_token = format!("Bearer {}", api_token);

//     match Request::new(&uri)
//         .method(Method::GET)
//         .header("Authorization", &bearer_token)
//         .header("Content-Type", "application/json")
//         .send(&mut writer)
//     {
//         Ok(res) => {
//             if !res.status_code().is_success() {
//                 return Some(res.status_code().to_string());
//             }

//             // let raw: ChatResponse = serde_json::from_slice(&writer).unwrap();
//             // let answer = raw.choices[0].message.content.clone();
//             // return Some(answer);
//             return Some(String::from_utf8(writer).unwrap());
//         }
//         Err(_) => {}
//     };
// }
