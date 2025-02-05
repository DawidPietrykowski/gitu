use crossterm::event::{self, KeyCode, KeyModifiers};

use strum::EnumIter;
use KeyCode::*;
use Op::*;
use SubmenuOp::*;
use TargetOp::*;

pub(crate) struct Keybind {
    pub submenu: SubmenuOp,
    pub mods: KeyModifiers,
    pub key: KeyCode,
    pub op: Op,
}

impl Keybind {
    const fn nomod(submenu: SubmenuOp, key: KeyCode, op: Op) -> Self {
        Self {
            submenu,
            mods: KeyModifiers::NONE,
            key,
            op,
        }
    }

    const fn ctrl(submenu: SubmenuOp, key: KeyCode, op: Op) -> Self {
        Self {
            submenu,
            mods: KeyModifiers::CONTROL,
            key,
            op,
        }
    }

    const fn shift(submenu: SubmenuOp, key: KeyCode, op: Op) -> Self {
        Self {
            submenu,
            mods: KeyModifiers::SHIFT,
            key,
            op,
        }
    }

    pub(crate) fn format_key(&self) -> String {
        let modifiers = self
            .mods
            .iter_names()
            .map(|(name, _)| match name {
                "CONTROL" => "C-",
                "SHIFT" => "",
                _ => unimplemented!("format_key mod {}", name),
            })
            .collect::<String>();

        modifiers
            + &match self.key {
                KeyCode::Enter => "ret".to_string(),
                KeyCode::Left => "←".to_string(),
                KeyCode::Right => "→".to_string(),
                KeyCode::Up => "↑".to_string(),
                KeyCode::Down => "↓".to_string(),
                KeyCode::Tab => "tab".to_string(),
                KeyCode::Delete => "del".to_string(),
                KeyCode::Insert => "ins".to_string(),
                KeyCode::F(n) => format!("F{}", n),
                KeyCode::Char(c) => if self.mods.contains(KeyModifiers::SHIFT) {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
                .to_string(),
                KeyCode::Esc => "esc".to_string(),
                _ => "???".to_string(),
            }
    }
}

pub(crate) const KEYBINDS: &[Keybind] = &[
    // Generic
    Keybind::nomod(Any, Char('q'), Quit),
    Keybind::nomod(Any, Esc, Quit),
    Keybind::nomod(None, Char('g'), Refresh),
    Keybind::nomod(None, Tab, ToggleSection),
    // Navigation
    Keybind::nomod(None, Char('k'), SelectPrevious),
    Keybind::nomod(None, Char('p'), SelectPrevious),
    Keybind::nomod(None, KeyCode::Up, SelectPrevious),
    Keybind::nomod(None, Char('j'), SelectNext),
    Keybind::nomod(None, Char('n'), SelectNext),
    Keybind::nomod(None, KeyCode::Down, SelectNext),
    Keybind::ctrl(None, Char('u'), HalfPageUp),
    Keybind::ctrl(None, Char('d'), HalfPageDown),
    // Help
    Keybind::nomod(None, Char('h'), Submenu(Help)),
    // Branch
    Keybind::nomod(None, Char('b'), Submenu(Branch)),
    Keybind::nomod(Branch, Char('b'), Target(Checkout)),
    Keybind::nomod(Branch, Char('c'), CheckoutNewBranch),
    // Commit
    Keybind::nomod(None, Char('c'), Submenu(SubmenuOp::Commit)),
    Keybind::nomod(SubmenuOp::Commit, Char('c'), Op::Commit),
    Keybind::nomod(SubmenuOp::Commit, Char('a'), CommitAmend),
    Keybind::nomod(SubmenuOp::Commit, Char('f'), Target(CommitFixup)),
    // Fetch
    Keybind::nomod(None, Char('f'), Submenu(Fetch)),
    Keybind::nomod(Fetch, Char('a'), FetchAll),
    // Log
    Keybind::nomod(None, Char('l'), Submenu(Log)),
    Keybind::nomod(Log, Char('l'), LogCurrent),
    Keybind::nomod(Log, Char('o'), Target(LogOther)),
    // Pull
    Keybind::shift(None, Char('F'), Submenu(SubmenuOp::Pull)),
    Keybind::nomod(SubmenuOp::Pull, Char('p'), Op::Pull),
    // Push
    Keybind::shift(None, Char('P'), Submenu(SubmenuOp::Push)),
    Keybind::nomod(SubmenuOp::Push, Char('p'), Op::Push),
    // Rebase
    Keybind::nomod(None, Char('r'), Submenu(Rebase)),
    Keybind::nomod(Rebase, Char('i'), Target(RebaseInteractive)),
    Keybind::nomod(Rebase, Char('a'), RebaseAbort),
    Keybind::nomod(Rebase, Char('c'), RebaseContinue),
    Keybind::nomod(Rebase, Char('f'), Target(RebaseAutosquash)),
    // Reset
    Keybind::nomod(None, Char('x'), Submenu(Reset)),
    Keybind::nomod(Reset, Char('s'), Target(ResetSoft)),
    Keybind::nomod(Reset, Char('m'), Target(ResetMixed)),
    Keybind::nomod(Reset, Char('h'), Target(ResetHard)),
    // Show refs
    Keybind::nomod(None, Char('y'), ShowRefs),
    // Discard
    Keybind::shift(None, Char('K'), Submenu(SubmenuOp::Discard)),
    Keybind::nomod(SubmenuOp::Discard, Char('y'), Target(TargetOp::Discard)),
    Keybind::nomod(SubmenuOp::Discard, Char('n'), Quit),
    // Target actions
    Keybind::nomod(None, Enter, Target(Show)),
    Keybind::nomod(None, Char('s'), Target(Stage)),
    Keybind::nomod(None, Char('u'), Target(Unstage)),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Op {
    CheckoutNewBranch,
    Commit,
    CommitAmend,
    FetchAll,
    HalfPageDown,
    HalfPageUp,
    LogCurrent,
    Pull,
    Push,
    Quit,
    RebaseAbort,
    RebaseContinue,
    Refresh,
    SelectNext,
    SelectPrevious,
    ShowRefs,
    Submenu(SubmenuOp),
    Target(TargetOp),
    ToggleSection,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SubmenuOp {
    Any,
    Branch,
    Commit,
    Discard,
    Fetch,
    Help,
    Log,
    None,
    Pull,
    Push,
    Rebase,
    Reset,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub(crate) enum TargetOp {
    Checkout,
    CommitFixup,
    Discard,
    LogOther,
    RebaseAutosquash,
    RebaseInteractive,
    ResetSoft,
    ResetMixed,
    ResetHard,
    Show,
    Stage,
    Unstage,
}

pub(crate) fn op_of_key_event(pending: SubmenuOp, key: event::KeyEvent) -> Option<Op> {
    KEYBINDS
        .iter()
        .find(|keybind| {
            (keybind.submenu, keybind.mods, keybind.key) == (pending, key.modifiers, key.code)
                || (keybind.submenu, keybind.mods, keybind.key) == (Any, key.modifiers, key.code)
        })
        .map(|keybind| keybind.op)
}

pub(crate) fn list(pending: &SubmenuOp) -> impl Iterator<Item = &Keybind> {
    let expected = if pending == &Help { None } else { *pending };

    KEYBINDS
        .iter()
        .filter(move |keybind| keybind.submenu == expected)
}
