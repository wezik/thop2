use crate::domain::{
    error::{self, DomainError, DomainErrorTemplate},
    template::{self, Template},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TmuxTemplate {
    pub is_created: bool,
    pub session: TmuxSession,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TmuxSession {
    pub name: String,
    pub windows: TmuxWindows,
}

pub type TmuxWindows = Vec<TmuxWindow>;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TmuxWindow {
    pub name: String,
    pub panes: TmuxPanes,
}

pub type TmuxPanes = Vec<TmuxPane>;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TmuxPane {
    pub name: String,
}

impl TryFrom<Template> for TmuxTemplate {
    type Error = DomainError;

    fn try_from(template: Template) -> Result<Self, Self::Error> {
        if template.engine != template::Engine::Tmux {
            return Err(TMUX_TEMPLATE_NOT_VALID.with_attr("template", template.path.0));
        }

        Ok(TmuxTemplate {
            is_created: false,
            session: TmuxSession {
                name: "default".to_string(),
                windows: vec![TmuxWindow {
                    name: "default".to_string(),
                    panes: vec![TmuxPane {
                        name: "default".to_string(),
                    }],
                }],
            },
        })
    }
}

pub const TMUX_TEMPLATE_NOT_VALID: DomainErrorTemplate =
    error::new("TMUX_TEMPLATE_NOT_VALID", "Tmux template is not valid");
