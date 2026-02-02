/*!
Priority enums and helpers for process, IO, memory, and thread priorities.

This module provides type-safe enums that map to Windows constants and
helper functions used by the service (for example, boosting a thread's
priority by one tier while avoiding TIME_CRITICAL promotions).

References:
- SetThreadPriority: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority
*/
use windows::Win32::System::Threading::{
    ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, MEMORY_PRIORITY, MEMORY_PRIORITY_BELOW_NORMAL,
    MEMORY_PRIORITY_LOW, MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_NORMAL, MEMORY_PRIORITY_VERY_LOW, NORMAL_PRIORITY_CLASS, PROCESS_CREATION_FLAGS,
    REALTIME_PRIORITY_CLASS, THREAD_PRIORITY,
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

    /// Human readable name for this variant.
    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("unknown")
    }

    /// Returns the Windows PROCESS_CREATION_FLAGS constant for this variant, if any.
    pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).and_then(|(_, _, val)| *val)
    }

    /// Parse from string (case-insensitive match against known names).
    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        Self::TABLE.iter().find(|(_, name, _)| *name == s.as_str()).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }

    /// Convert a Windows priority class constant (GetPriorityClass) to a human name.
    /// Keeps compatibility with logging code that expects a &str.
    pub fn from_win_const(val: u32) -> &'static str {
        Self::TABLE
            .iter()
            .find(|(_, _, const_opt)| const_opt.map(|c| c.0) == Some(val))
            .map(|(_, name, _)| *name)
            .unwrap_or("unknown")
    }
}

/// IO priority levels for process I/O operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High, // Requires SeIncreaseBasePriorityPrivilege + admin
}

impl IOPriority {
    const TABLE: &'static [(Self, &'static str, Option<u32>)] = &[
        (Self::None, "none", None),
        (Self::VeryLow, "very low", Some(0)),
        (Self::Low, "low", Some(1)),
        (Self::Normal, "normal", Some(2)),
        (Self::High, "high", Some(3)),
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("unknown")
    }

    pub fn as_win_const(&self) -> Option<u32> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).and_then(|(_, _, val)| *val)
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        Self::TABLE.iter().find(|(_, name, _)| *name == s.as_str()).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }

    pub fn from_win_const(val: u32) -> &'static str {
        Self::TABLE
            .iter()
            .find(|(_, _, const_opt)| const_opt.map(|c| c) == Some(val))
            .map(|(_, name, _)| *name)
            .unwrap_or("unknown")
    }
}

/// Memory priority information structure for SetProcessInformation.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
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
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("unknown")
    }

    pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).and_then(|(_, _, val)| *val)
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        Self::TABLE.iter().find(|(_, name, _)| *name == s.as_str()).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }

    pub fn from_win_const(val: u32) -> &'static str {
        Self::TABLE
            .iter()
            .find(|(_, _, const_opt)| const_opt.map(|c| c.0) == Some(val))
            .map(|(_, name, _)| *name)
            .unwrap_or("unknown")
    }
}

/// Thread priority abstraction and helpers.
///
/// Matches values from Microsoft SetThreadPriority documentation:
/// - THREAD_MODE_BACKGROUND_BEGIN = 0x00010000
/// - THREAD_MODE_BACKGROUND_END   = 0x00020000
/// - THREAD_PRIORITY_ABOVE_NORMAL = 1
/// - THREAD_PRIORITY_BELOW_NORMAL = -1
/// - THREAD_PRIORITY_HIGHEST      = 2
/// - THREAD_PRIORITY_IDLE         = -15
/// - THREAD_PRIORITY_LOWEST       = -2
/// - THREAD_PRIORITY_NORMAL       = 0
/// - THREAD_PRIORITY_TIME_CRITICAL= 15
///
/// Note: `GetThreadPriority` returns THREAD_PRIORITY_ERROR_RETURN (0x7FFFFFFF) on error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    None,
    ErrorReturn,         // 0x7FFFFFFF
    ModeBackgroundBegin, // 0x00010000 (use only for current thread)
    ModeBackgroundEnd,   // 0x00020000 (use only for current thread)
    Idle,                // -15
    Lowest,              // -2
    BelowNormal,         // -1
    Normal,              // 0
    AboveNormal,         // 1
    Highest,             // 2
    TimeCritical,        // 15
}

impl ThreadPriority {
    const TABLE: &'static [(Self, &'static str, Option<i32>)] = &[
        (Self::None, "none", None),
        (Self::ErrorReturn, "error", Some(0x7FFFFFFF_i32)),
        (Self::ModeBackgroundBegin, "background begin", Some(0x00010000_i32)),
        (Self::ModeBackgroundEnd, "background end", Some(0x00020000_i32)),
        (Self::Idle, "idle", Some(-15)),
        (Self::Lowest, "lowest", Some(-2)),
        (Self::BelowNormal, "below normal", Some(-1)),
        (Self::Normal, "normal", Some(0)),
        (Self::AboveNormal, "above normal", Some(1)),
        (Self::Highest, "highest", Some(2)),
        (Self::TimeCritical, "time critical", Some(15)),
    ];

    /// Human-readable name.
    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("unknown")
    }

    /// Convert enum to raw Win32 thread priority integer.
    pub fn as_win_const(&self) -> Option<i32> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).and_then(|(_, _, val)| *val)
    }

    /// Parse from string (case-insensitive match against known names).
    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        Self::TABLE.iter().find(|(_, name, _)| *name == s.as_str()).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }

    /// Create enum from raw GetThreadPriority/SetThreadPriority value.
    pub fn from_win_const(val: i32) -> Self {
        Self::TABLE
            .iter()
            .find(|(_, _, const_opt)| *const_opt == Some(val))
            .map(|(v, _, _)| *v)
            .unwrap_or(Self::None)
    }

    /// Return the next higher priority tier, capping at `Highest`.
    /// We intentionally avoid promoting to `TimeCritical` automatically.
    pub fn boost_one(&self) -> Self {
        match self {
            ThreadPriority::None => ThreadPriority::None,
            ThreadPriority::ErrorReturn => ThreadPriority::ErrorReturn,
            ThreadPriority::ModeBackgroundBegin => ThreadPriority::ModeBackgroundBegin,
            ThreadPriority::ModeBackgroundEnd => ThreadPriority::ModeBackgroundEnd,
            ThreadPriority::Idle => ThreadPriority::Lowest,
            ThreadPriority::Lowest => ThreadPriority::BelowNormal,
            ThreadPriority::BelowNormal => ThreadPriority::Normal,
            ThreadPriority::Normal => ThreadPriority::AboveNormal,
            ThreadPriority::AboveNormal => ThreadPriority::Highest,
            ThreadPriority::Highest => ThreadPriority::Highest,
            ThreadPriority::TimeCritical => ThreadPriority::TimeCritical,
        }
    }

    /// Convert to `THREAD_PRIORITY` wrapper used by `SetThreadPriority`.
    pub fn to_thread_priority_struct(&self) -> THREAD_PRIORITY {
        THREAD_PRIORITY(self.as_win_const().unwrap_or(0))
    }
}
