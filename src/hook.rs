use harness::{Compaction, Event, Observable};

use crate::Hook;

impl Observable for Hook {
    fn events(&self) -> Vec<Event> {
        match self {
            Hook::SessionStart(_) => vec![Event::Started],
            Hook::PermissionRequest(_) => vec![Event::PermissionAsked],
            Hook::PostToolUse(_) => vec![Event::ToolUsed],
            Hook::PostCompact(compact) => vec![Event::Compacted(Compaction {
                trigger: compact.trigger.clone(),
                tokens_option: None,
            })],
        }
    }
}
