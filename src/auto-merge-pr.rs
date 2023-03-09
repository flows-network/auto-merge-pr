use github_flows::{get_octo, listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;
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
    let octo = get_octo(Some(String::from(owner)));

    match payload {
        EventPayload::IssueCommentEvent(e) => match e.issue.pull_request {
            Some(pr) => {
                let pull_request_url = pr.url;
                let possible_pull_number_str = pull_request_url
                    .path_segments()
                    .unwrap()
                    .collect::<Vec<_>>()
                    .pop()
                    .unwrap();

                if possible_pull_number_str.parse::<u64>().is_ok() {
                    pull_number = possible_pull_number_str.parse::<u64>().unwrap_or(0);
                }
            }
            None => {}
        },
        EventPayload::PullRequestReviewEvent(e) => {
            pull_number = e.pull_request.number;
        }
        EventPayload::PullRequestReviewCommentEvent(e) => {
            pull_number = e.pull_request.number;
        }
        EventPayload::UnknownEvent(e) => {
            let text = e.to_string();

            let val: serde_json::Value = serde_json::from_str(&text).unwrap();

            pull_number = val["pull_request"]["number"].as_u64().unwrap_or(0);
        }

        _ => (),
    }
    let mut count = 0;
    let review_page = octo.pulls(owner, repo).list_reviews(pull_number).await;

    match review_page {
        Err(_) => (),
        Ok(items) => {
            for item in items {
                let reviewer_login: String = if item.user.is_some() {
                    item.user.unwrap().login as String
                } else {
                    "".to_string()
                };
                let review_text = item.body.unwrap_or("".to_string().to_lowercase());

                if lead_reviewer_list.contains(&reviewer_login) && review_text.contains("lgtm") {
                    count += 1;
                }
                if count >= 2 {
                    // merge pr
                    let _ = octo.pulls(owner, repo).merge(pull_number).send().await;
                    let body = format!("PR #{} merged", pull_number);
                    send_message_to_channel("ik8", "general", body);
                    return;
                }
            }
        }
    }
}
