-- Add migration script here
create table issue_delivery_queue(
  newsletter_issue_id uuid not null references newsletter_issues(newsletter_issue_id),
  subscriber_email text not null,
  PRIMARY KEY(newsletter_issue_id, subscriber_email)
);
