mod crud;
mod comments;
mod labels;
mod links;
mod transitions;
mod watchers;

// Re-export all handlers
pub use crud::{
    create_issue_handler,
    update_issue_handler,
    search_issues_handler,
    get_issue_handler,
};

pub use comments::{
    add_comment_handler,
    get_comments_handler,
    update_comment_handler,
    delete_comment_handler,
};

pub use labels::{
    add_label_handler,
    remove_label_handler,
};

pub use links::{
    link_issues_handler,
    delete_issue_link_handler,
    list_link_types_handler,
};

pub use transitions::{
    get_transitions_handler,
    transition_issue_handler,
    assign_issue_handler,
};

pub use watchers::{
    add_watcher_handler,
    remove_watcher_handler,
    get_watchers_handler,
};
