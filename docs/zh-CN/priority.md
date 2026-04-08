# Priority 模块文档

优先级级别枚举和 Windows API 映射。

## 概述

本模块定义了所有 Windows 优先级类型的枚举，支持双向转换到/自 Windows 常量和字符串表示。

## 调用者

- [config.rs](config.md) - 从配置解析优先级字符串
- [apply.rs](apply.md) - 转换到 Windows API 常量
- [scheduler.rs](scheduler.md) - 线程优先级操作
- [logging.rs](logging.md) - 转换回字符串以输出

## 枚举

### ProcessPriority

Windows 进程优先级类。

```rust
pub enum ProcessPriority {
    None,           // 不更改
    Idle,           // IDLE_PRIORITY_CLASS
    BelowNormal,    // BELOW_NORMAL_PRIORITY_CLASS
    Normal,         // NORMAL_PRIORITY_CLASS
    AboveNormal,    // ABOVE_NORMAL_PRIORITY_CLASS
    High,           // HIGH_PRIORITY_CLASS
    Realtime,       // REALTIME_PRIORITY_CLASS
}
```

**字符串值：**
| 枚举 | 字符串 | Windows 常量 |
|------|--------|------------------|
| `None` | `"none"` | `None` |
| `Idle` | `"idle"` | `IDLE_PRIORITY_CLASS` |
| `BelowNormal` | `"below normal"` | `BELOW_NORMAL_PRIORITY_CLASS` |
| `Normal` | `"normal"` | `NORMAL_PRIORITY_CLASS` |
| `AboveNormal` | `"above normal"` | `ABOVE_NORMAL_PRIORITY_CLASS` |
| `High` | `"high"` | `HIGH_PRIORITY_CLASS` |
| `Realtime` | `"real time"` | `REALTIME_PRIORITY_CLASS` |

**方法：**

```rust
// 转换为显示字符串
pub fn as_str(&self) -> &'static str

// 转换为 Windows API 常量
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>

// 从字符串解析（不区分大小写）
pub fn from_str(s: &str) -> Self

// 从 Windows 常量值转换
pub fn from_win_const(val: u32) -> &'static str
```

**示例：**
```rust
let p = ProcessPriority::from_str("High");        // → ProcessPriority::High
p.as_str();                                        // → "high"
p.as_win_const();                                  // → Some(HIGH_PRIORITY_CLASS)

ProcessPriority::from_str("invalid");              // → ProcessPriority::None
ProcessPriority::from_win_const(0x8000);           // → "normal"
```

### IOPriority

I/O 优先级级别。

```rust
pub enum IOPriority {
    None,       // 不更改
    VeryLow,    // 优先级 0
    Low,        // 优先级 1
    Normal,     // 优先级 2
    High,       // 优先级 3（需要管理员 + SeIncreaseBasePriorityPrivilege）
}
```

**字符串值：**
| 枚举 | 字符串 | 值 |
|------|--------|-------|
| `None` | `"none"` | `None` |
| `VeryLow` | `"very low"` | `Some(0)` |
| `Low` | `"low"` | `Some(1)` |
| `Normal` | `"normal"` | `Some(2)` |
| `High` | `"high"` | `Some(3)` |

**特权要求：**
- `High` 需要：
  - 管理员令牌
  - `SeIncreaseBasePriorityPrivilege`

**方法：**与 `ProcessPriority` 模式相同

### MemoryPriority

内存页面优先级级别。

```rust
pub enum MemoryPriority {
    None,         // 不更改
    VeryLow,      // MEMORY_PRIORITY_VERY_LOW
    Low,          // MEMORY_PRIORITY_LOW
    Medium,       // MEMORY_PRIORITY_MEDIUM
    BelowNormal,  // MEMORY_PRIORITY_BELOW_NORMAL
    Normal,       // MEMORY_PRIORITY_NORMAL
}
```

**字符串值：**
| 枚举 | 字符串 | Windows 常量 |
|------|--------|------------------|
| `None` | `"none"` | `None` |
| `VeryLow` | `"very low"` | `MEMORY_PRIORITY_VERY_LOW` |
| `Low` | `"low"` | `MEMORY_PRIORITY_LOW` |
| `Medium` | `"medium"` | `MEMORY_PRIORITY_MEDIUM` |
| `BelowNormal` | `"below normal"` | `MEMORY_PRIORITY_BELOW_NORMAL` |
| `Normal` | `"normal"` | `MEMORY_PRIORITY_NORMAL` |

**注意：**内存优先级影响页面替换 - 较低优先级的页面在内存压力下首先被分页。

**结构包装器：**
```rust
#[repr(C)]
pub struct MemoryPriorityInformation(pub u32);
```

与 `GetProcessInformation`/`SetProcessInformation` 一起使用。

### ThreadPriority

线程优先级级别。

```rust
pub enum ThreadPriority {
    None,                // 不更改
    ErrorReturn,         // 0x7FFFFFFF（错误指示器）
    ModeBackgroundBegin, // 0x00010000（仅当前线程）
    ModeBackgroundEnd,   // 0x00020000（仅当前线程）
    Idle,                // -15
    Lowest,              // -2
    BelowNormal,         // -1
    Normal,              // 0
    AboveNormal,         // 1
    Highest,             // 2
    TimeCritical,        // 15
}
```

**字符串值：**
| 枚举 | 字符串 | 值 |
|------|--------|-------|
| `None` | `"none"` | `None` |
| `ErrorReturn` | `"error"` | `Some(0x7FFFFFFF)` |
| `ModeBackgroundBegin` | `"background begin"` | `Some(0x00010000)` |
| `ModeBackgroundEnd` | `"background end"` | `Some(0x00020000)` |
| `Idle` | `"idle"` | `Some(-15)` |
| `Lowest` | `"lowest"` | `Some(-2)` |
| `BelowNormal` | `"below normal"` | `Some(-1)` |
| `Normal` | `"normal"` | `Some(0)` |
| `AboveNormal` | `"above normal"` | `Some(1)` |
| `Highest` | `"highest"` | `Some(2)` |
| `TimeCritical` | `"time critical"` | `Some(15)` |

**特殊值：**
- `ErrorReturn` (0x7FFFFFFF) - 指示 `GetThreadPriority` 出错
- `ModeBackgroundBegin/End` - 只能在调用线程上使用（用于后台模式）

**方法：**

```rust
// 标准转换
pub fn as_str(&self) -> &'static str
pub fn as_win_const(&self) -> Option<i32>
pub fn from_str(s: &str) -> Self
pub fn from_win_const(val: i32) -> Self

// 返回下一个更高的优先级级别，上限为 Highest
pub fn boost_one(&self) -> Self

// 转换为 Windows THREAD_PRIORITY 结构
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

**boost_one 层次结构：**
```
None → None
Idle → Lowest
Lowest → BelowNormal
BelowNormal → Normal
Normal → AboveNormal
AboveNormal → Highest
Highest → Highest
TimeCritical → TimeCritical
(ErrorReturn, ModeBackground*) → 相同
```

**示例：**
```rust
let p = ThreadPriority::Normal;
p.boost_one();  // → ThreadPriority::AboveNormal

ThreadPriority::Highest.boost_one();  // → ThreadPriority::Highest（上限）

// 解析并转换用于 API
let p = ThreadPriority::from_str("above normal");
SetThreadPriority(handle, p.to_thread_priority_struct());
```

## 设计模式

所有枚举遵循相同的模式：

1. **静态查找表：**
```rust
const TABLE: &'static [(Self, &'static str, Option<NativeType>)] = &[
    (Self::Variant, "string", Some(NATIVE_VALUE)),
    ...
];
```

2. **双向转换：**
- `as_str()` - 用于显示/日志
- `as_win_const()` - 用于 API 调用
- `from_str()` - 用于配置解析（不区分大小写）
- `from_win_const()` - 用于读取当前值

3. **None 变体：**
所有枚举都有一个 `None` 变体，字符串表示为 `"none"`，映射到 `None` Windows 常量。这表示"不更改此设置"。

## 依赖

- `windows::Win32::System::Threading` - Windows 优先级常量

## 平台说明

### 优先级类效果

| 优先级类 | 基础优先级 | 动态范围 |
|----------------|---------------|---------------|
| Idle | 4 | 1-6 |
| Below Normal | 6 | 1-8 |
| Normal | 8 | 1-15 |
| Above Normal | 10 | 1-15 |
| High | 13 | 1-15 |
| Realtime | 24 | 16-31 |

### I/O 优先级映射

I/O 优先级独立于 CPU 优先级：
- 大多数进程以 Normal I/O 优先级运行
- VeryLow 用于后台任务
- High 用于时间关键型 I/O

### 内存优先级效果

较低的内存优先级导致页面在压力下优先被移动到修改/待机列表。适用于：
- 后台进程（设置较低）
- 交互式进程（设置较高）
