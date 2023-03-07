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
    if let EventPayload::UnknownEvent(ref e) = payload {
        send_message_to_channel(
            "ik8",
            "general",
            serde_json::to_string(&payload).unwrap_or("payload review failed".to_string()),
        );

        // let pull_request = e["pull_request"].clone();
        // let review = e["review"].clone();
        // let repository = e["repository"].clone();
        // send_message_to_channel(
        //     "ik8",
        //     "step_1",
        //     serde_json::to_string(&pull_request)
        //         .unwrap_or("parsing pull request failed".to_string()),
        // );
        // send_message_to_channel(
        //     "ik8",
        //     "step_2",
        //     serde_json::to_string(&review).unwrap_or("parsing review failed".to_string()),
        // );
        // send_message_to_channel(
        //     "ik8",
        //     "step_3",
        //     serde_json::to_string(&repository).unwrap_or("parse repository failed".to_string()),
        // );

        // let raw_data: serde_json::Value = serde_json::from_slice(payload).unwrap();

        // let review_id = raw_data["review"]["id"].as_i64().unwrap_or(0);
        // let pr_id = raw_data["pull_request"]["id"].as_i64().unwrap_or(0);
        // let pull_number = raw_data["pull_request"]["number"].as_i64().unwrap_or(0);
        // let owner = "jaykchen";
        // let repo = "vitesse-lite";

        // let comments = get_comments(
        //     owner,
        //     repo,
        //     &pull_number.to_string(),
        //     &review_id.to_string(),
        // );

        // send_message_to_channel("ik8", "step_1", comments.unwrap());
        // let review_comments_url = e.pull_request.review_comments_url;

        // let url = "https://api.github.com/repos/jaykchen/vitesse-lite/pulls/7/comments";

        // let octocrab = get_octo(Some(String::from("jaykchen")));

        // let pr = octocrab.pulls("jaykchen", "vitesse-lite");

        // let pr_id = 7;
        // let comment_page = pr.list_reviews(pr_id).await.unwrap();
        // let comments = comment_page
        //     .items
        //     .into_iter()
        //     .map(|c| c.body_text.unwrap())
        //     .collect::<Vec<String>>()
        //     .join("");
        // send_message_to_channel("ik8", "step_4", comments);

        // let temp = comments.iter().filter(|c| c..contains("lgtm"));

        // if temp.len() > 0 {
        //     let pr = pr.get(pr_id).send().await;
        //     let merge = pr.merge().send().await;
        // };
        // let temp = comment_page
        //     .into_iter()
        //     .map(|c| c.body_text.unwrap())
        //     .collect::<Vec<String>>()
        //     .join("");

        // .contains("lgtm")).map(|ch| if )
        //     .filter(|c| c.body_text.unwrap().contains("lgtm")).map(|ch| if )
        //     .collect::<Vec<String>>()
        //     .join("");
        // let text = serde_json::json!(comments);
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
