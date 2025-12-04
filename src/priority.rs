//! Priority enums for process, IO, and memory priority management.
//!
//! These enums provide type-safe representations of Windows priority levels
//! with conversion to/from strings and Windows constants.

use windows::Win32::System::Threading::{
    ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, MEMORY_PRIORITY, MEMORY_PRIORITY_BELOW_NORMAL,
    MEMORY_PRIORITY_LOW, MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_NORMAL, MEMORY_PRIORITY_VERY_LOW, NORMAL_PRIORITY_CLASS, PROCESS_CREATION_FLAGS,
    REALTIME_PRIORITY_CLASS,
};

/// Process priority levels corresponding to Windows priority classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}

impl ProcessPriority {
    const TABLE: &'static [(Self, &'static str, Option<PROCESS_CREATION_FLAGS>)] = &[
        (Self::None, "none", None),
        (Self::Idle, "idle", Some(IDLE_PRIORITY_CLASS)),
        (Self::BelowNormal, "below normal", Some(BELOW_NORMAL_PRIORITY_CLASS)),
        (Self::Normal, "normal", Some(NORMAL_PRIORITY_CLASS)),
        (Self::AboveNormal, "above normal", Some(ABOVE_NORMAL_PRIORITY_CLASS)),
        (Self::High, "high", Some(HIGH_PRIORITY_CLASS)),
        (Self::Realtime, "real time", Some(REALTIME_PRIORITY_CLASS)),
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("fail as str")
    }

    pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).and_then(|(_, _, val)| *val)
    }

    pub fn from_str(s: &str) -> Self {
        Self::TABLE
            .iter()
            .find(|(_, name, _)| s.to_lowercase() == *name)
            .map(|(v, _, _)| *v)
            .unwrap_or(Self::None)
    }
}

/// IO priority levels for process I/O operations.
///
/// Note on High and Critical:
/// - High (3): Requires SeIncreaseBasePriorityPrivilege AND admin elevation.
///   Error 0xC0000061 (STATUS_PRIVILEGE_NOT_HELD) if not elevated.
/// - Critical (4): Reserved for kernel/system use only.
///   Error 0xC000000D (STATUS_INVALID_PARAMETER) - not valid from user mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High, // Requires admin elevation (STATUS_PRIVILEGE_NOT_HELD without it)
          // Critical, // Reserved for kernel use (STATUS_INVALID_PARAMETER)
}

impl IOPriority {
    const TABLE: &'static [(Self, &'static str, Option<u32>)] = &[
        (Self::None, "none", None),
        (Self::VeryLow, "very low", Some(0)),
        (Self::Low, "low", Some(1)),
        (Self::Normal, "normal", Some(2)),
        (Self::High, "high", Some(3)), // Requires admin elevation
                                       // (Self::Critical, "critical", Some(4)), // Kernel only
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("fail as str")
    }

    pub fn as_win_const(&self) -> Option<u32> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, _, val)| *val).unwrap_or(None)
    }

    pub fn from_str(s: &str) -> Self {
        Self::TABLE
            .iter()
            .find(|(_, name, _)| s.to_lowercase() == *name)
            .map(|(v, _, _)| *v)
            .unwrap_or(Self::None)
    }
}

/// Memory priority information structure for SetProcessInformation.
#[repr(C)]
#[derive(PartialEq)]
pub struct MemoryPriorityInformation(pub u32);

/// Memory priority levels for process memory management.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}

impl MemoryPriority {
    const TABLE: &'static [(Self, &'static str, Option<MEMORY_PRIORITY>)] = &[
        (Self::None, "none", None),
        (Self::VeryLow, "very low", Some(MEMORY_PRIORITY_VERY_LOW)),
        (Self::Low, "low", Some(MEMORY_PRIORITY_LOW)),
        (Self::Medium, "medium", Some(MEMORY_PRIORITY_MEDIUM)),
        (Self::BelowNormal, "below normal", Some(MEMORY_PRIORITY_BELOW_NORMAL)),
        (Self::Normal, "normal", Some(MEMORY_PRIORITY_NORMAL)),
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, s, _)| *s).unwrap_or("none")
    }

    pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, _, val)| *val).unwrap_or(None)
    }

    pub fn from_str(s: &str) -> Self {
        Self::TABLE.iter().find(|(_, str, _)| *str == s).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }
}
