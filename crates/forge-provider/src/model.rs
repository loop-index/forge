use derive_more::derive::From;
use derive_setters::Setters;
use forge_tool::{Tool, ToolId};
use serde_json::Value;

#[derive(Default, Setters, Debug, Clone)]
pub struct Request {
    pub context: Vec<AnyMessage>,
    pub tools: Vec<Tool>,
    pub tool_result: Vec<ToolResult>,
}

impl Request {
    pub fn add_tool(mut self, tool: impl Into<Tool>) -> Self {
        let tool: Tool = tool.into();
        self.tools.push(tool);
        self
    }

    pub fn extend_tools(mut self, tools: Vec<impl Into<Tool>>) -> Self {
        self.tools.extend(tools.into_iter().map(Into::into));
        self
    }

    pub fn add_message(mut self, message: impl Into<AnyMessage>) -> Self {
        self.context.push(message.into());
        self
    }

    pub fn extend_messages(mut self, messages: Vec<impl Into<AnyMessage>>) -> Self {
        self.context.extend(messages.into_iter().map(Into::into));
        self
    }
}

#[derive(Debug, Clone)]
pub struct System;
#[derive(Debug, Clone)]
pub struct User;
#[derive(Debug, Clone)]
pub struct Assistant;

pub trait Role {
    fn name() -> String;
}

impl Role for System {
    fn name() -> String {
        "system".to_string()
    }
}

impl Role for User {
    fn name() -> String {
        "user".to_string()
    }
}

impl Role for Assistant {
    fn name() -> String {
        "assistant".to_string()
    }
}

#[derive(Setters, Debug, Clone)]
pub struct Message<R: Role> {
    pub content: String,
    pub role: R,
}

impl Message<System> {
    pub fn system(content: impl Into<String>) -> Self {
        Message { content: content.into(), role: System {} }
    }
}

impl Message<User> {
    pub fn user(content: impl Into<String>) -> Self {
        Message { content: content.into(), role: User {} }
    }
}

impl Message<Assistant> {
    pub fn assistant(content: impl Into<String>) -> Self {
        Message { content: content.into(), role: Assistant {} }
    }
}

#[derive(Debug, Clone, From)]
pub enum AnyMessage {
    System(Message<System>),
    User(Message<User>),
    Assistant(Message<Assistant>),
}

#[derive(Setters, Debug, Clone)]
pub struct Response {
    pub message: Message<Assistant>,
    pub tool_use: Vec<ToolUse>,
}

impl Response {
    pub fn new(message: String) -> Response {
        Response { message: Message::assistant(message), tool_use: vec![] }
    }

    pub fn add_call(mut self, call_tool: impl Into<ToolUse>) -> Self {
        self.tool_use.push(call_tool.into());
        self
    }

    pub fn extend_calls(mut self, calls: Vec<impl Into<ToolUse>>) -> Self {
        self.tool_use.extend(calls.into_iter().map(Into::into));
        self
    }
}

/// Unique identifier for a using a tool
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseId(String);

impl UseId {
    #[allow(unused)]
    pub(crate) fn new(id: String) -> UseId {
        UseId(id)
    }
}

#[derive(Setters, Debug, Clone)]
pub struct ToolUse {
    pub tool_use_id: UseId,
    pub tool_id: ToolId,
    pub input: Value,
}

#[derive(Setters, Debug, Clone)]
pub struct ToolResult {
    pub tool_use_id: UseId,
    pub content: Value,
}
