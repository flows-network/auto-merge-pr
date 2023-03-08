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
    // let mut pr_review_tracker: HashMap<String, usize> = HashMap::new();
    let lead_reviewer_list = vec!["jaykchen".to_string(), "amiiiiii830".to_string()];
    let owner = "jaykchen";
    let repo = "vitesse-lite";

    listen_to_event(
        owner,
        repo,
        vec!["pull_request", "pull_request_review"],
        |payload| handler(payload, &lead_reviewer_list),
    )
    .await;

    Ok(())
}

// pub fn track_lgtm(
//     review_text: &str,
//     reviewer_login: &str,
//     lead_reviewer_list: &Vec<String>,
//     pull_request_id: &str,
//     pr_review_tracker: &mut HashMap<String, usize>,
// ) {
//     if lead_reviewer_list.contains(&reviewer_login.to_string())
//         && review_text.to_lowercase().contains("lgtm")
//     {
//         pr_review_tracker
//             .entry(pull_request_id.to_string())
//             .and_modify(|e| *e += 1)
//             .or_insert(1usize);
//     }
// }

// comments_url = https://api.github.com/repos/jaykchen/vitesse-lite/issues/7/comments
async fn handler(owner: &str, repo: &str, lead_reviewer_list: &Vec<String>, payload: EventPayload) {
    let mut count = 0;
    let octo = get_octo(Some(String::from(owner))).await;
    let review_page = octo.pulls(owner, repo).get(pull_number).await.unwrap();

    for item in review_page.items {
        let reviewer_login = item.user.unwrap().login;
        let review_text = item.body.unwrap();

        if lead_reviewer_list.contains(&reviewer_login.to_string())
            && review_text.to_lowercase().contains("lgtm")
        {
            count += 1;
        }
        if count >= 2 {
            // merge pr
            let merge_result = octo.pulls(owner, repo).merge(pull_number).await.unwrap();
            send_message_to_channel("ik8", "step_4", "pr merged".to_string());
        }
    }

    // let pr_first_message = page.body.unwrap();
    // let issue = octo..issues("jaykchen", "vitesse-lite").get(17).await.unwrap();
    // let pr_first_message = page.body.unwrap();

    match payload {
        EventPayload::PullRequestEvent(e) => {
            send_message_to_channel("ik8", "step_1", "a pr was filed".to_string());
        }

        EventPayload::PullRequestReviewEvent(e) => {
            let pull_request_id: String = e.pull_request.id.to_string();
            let review = e.review;
            let review_text = review.body_text.unwrap_or("".to_string());
            let reviewer_login: String = review.user.expect("no user found ").login;

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

            send_message_to_channel("ik8", "step_3", text);
        }

        _ => (),
    }
}
