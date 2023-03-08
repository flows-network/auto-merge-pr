use dotenv::dotenv;
use github_flows::{get_octo, listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;
use std::env;
use tokio::*;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner = "jaykchen";
    let repo = "vitesse-lite";
    let lead_reviewer_list = vec!["jaykchen".to_string(), "amiiiiii830".to_string()];

    listen_to_event(
        owner,
        repo,
        vec!["pull_request", "pull_request_review"],
        |payload| handler(owner, repo, payload, &lead_reviewer_list),
    )
    .await;

    Ok(())
}

// pull_request_url = https://github.com/jaykchen/vitesse-lite/pull/17

async fn handler(owner: &str, repo: &str, payload: EventPayload, lead_reviewer_list: &Vec<String>) {
    let mut pull_number = 0;

    match payload {
        EventPayload::PullRequestEvent(e) => {
            send_message_to_channel("ik8", "step_1", "a pr was filed".to_string());
        }

        EventPayload::PullRequestReviewEvent(e) => {
            pull_number = e.pull_request.number;
        }
        EventPayload::PullRequestReviewCommentEvent(e) => {
            pull_number = e.pull_request.number;
        }

        _ => {
            send_message_to_channel("ik8", "step_4", "unknow payload".to_string());
        }
    }
    let mut count = 0;
    let octo = get_octo(Some(String::from(owner)));
    let review_page = octo
        .pulls(owner, repo)
        .list_reviews(pull_number)
        .await
        .unwrap();

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
            let _ = octo.pulls(owner, repo).merge(pull_number);
            send_message_to_channel("ik8", "step_3", "pr merged".to_string());
            return;
        }
    }
}
