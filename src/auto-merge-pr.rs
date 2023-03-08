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
        EventPayload::PullRequestEvent(e) => {
            send_message_to_channel("ik8", "step_1", "a pr was filed".to_string());
            return;
        }

        EventPayload::PullRequestReviewEvent(e) => {
            pull_number = e.pull_request.number;
        }
        EventPayload::PullRequestReviewCommentEvent(e) => {
            pull_number = e.pull_request.number;
        }
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
                    pull_number = possible_pull_number_str.parse::<u64>().unwrap();
                }
            }
            None => {
                send_message_to_channel(
                    "ik8",
                    "step_1",
                    "there is issue_comment on your repo, not about pr".to_string(),
                );
                return;
            }
        },
        _ => {
            send_message_to_channel("ik8", "step_3", "unknow payload".to_string());
            return;
        }
    }
    let mut count = 0;
    let review_page = octo.pulls(owner, repo).list_reviews(pull_number).await;

    match review_page {
        Err(e) => send_message_to_channel("ik8", "step_4", e.to_string()),
        Ok(items) => {
            for item in items {
                let reviewer_login: String = if item.user.is_some() {
                    item.user.unwrap().login as String
                } else {
                    "".to_string()
                };
                let review_text = item.body.unwrap_or("".to_string());

                if lead_reviewer_list.contains(&reviewer_login)
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
    }
}
