#[cfg(not(target_arch = "wasm32"))]
pub type TimeStamp = std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub type TimeStamp = f32; // 用秒或毫秒表示，如果你习惯用秒就转换成秒

/// 获取当前时间戳
#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> TimeStamp {
    std::time::Instant::now()
}

#[cfg(target_arch = "wasm32")]
pub fn now() -> TimeStamp {
    (web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
        / 1000.) as f32
}
