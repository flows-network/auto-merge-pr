use dotenv::dotenv;
use github_flows::{get_octo, listen_to_event, EventPayload};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use slack_flows::send_message_to_channel;
use std::collections::HashMap;
use std::env;
use tokio::*;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let mut pr_review_tracker: HashMap<String, usize> = HashMap::new();
    let lead_reviewer_list = vec!["jaykchen".to_string(), "amiiiiii830".to_string()];

    listen_to_event(
        "jaykchen",
        "vitesse-lite",
        vec!["pull_request", "pull_request_review"],
        |payload| handler(payload, &mut pr_review_tracker, &lead_reviewer_list),
    )
    .await;

    Ok(())
}

pub fn track_lgtm(
    review_text: &str,
    reviewer_login: &str,
    lead_reviewer_list: &Vec<String>,
    pull_request_id: &str,
    pr_review_tracker: &mut HashMap<String, usize>,
) {
    if lead_reviewer_list.contains(&reviewer_login.to_string())
        && review_text.to_lowercase().contains("lgtm")
    {
        pr_review_tracker
            .entry(pull_request_id.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1usize);
    }
}

// comments_url = https://api.github.com/repos/jaykchen/vitesse-lite/issues/7/comments
async fn handler(
    payload: EventPayload,
    pr_review_tracker: &mut HashMap<String, usize>,
    lead_reviewer_list: &Vec<String>,
) {
    match payload {
        EventPayload::PullRequestEvent(e) => {
            send_message_to_channel("ik8", "step_1", "a pr was filed".to_string());
        }

        EventPayload::PullRequestReviewEvent(e) => {
            let pull_request_id: String = e.pull_request.id.to_string();
            let review = e.review;
            let review_text = review.body_text.unwrap_or("".to_string());
            let reviewer_login: String = review.user.expect("no user found ").login;

            track_lgtm(
                &review_text,
                &reviewer_login,
                lead_reviewer_list,
                &pull_request_id,
                pr_review_tracker,
            );
            let text: String = pr_review_tracker
                .iter()
                .map(|(a, b)| format!("{}: {}", a, b))
                .collect::<Vec<String>>()
                .join("; ");

            send_message_to_channel("ik8", "step_2", text);
            // send_message_to_channel(
            //     "ik8",
            //     "step_2",
            //     serde_json::to_string(&e).unwrap_or("pr review failed".to_string()),
            // );
        }

        EventPayload::PullRequestReviewCommentEvent(e) => {
            let pull_request_id: String = e.pull_request.id.to_string();
            let comment = e.comment;
            let review_text = comment.body_text.unwrap_or("".to_string());
            let reviewer_login: String = comment.user.login;

            track_lgtm(
                &review_text,
                &reviewer_login,
                lead_reviewer_list,
                &pull_request_id,
                pr_review_tracker,
            );

            let text: String = pr_review_tracker
                .iter()
                .map(|(a, b)| format!("{}: {}", a, b))
                .collect::<Vec<String>>()
                .join("; ");

            send_message_to_channel("ik8", "step_3", text);
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
