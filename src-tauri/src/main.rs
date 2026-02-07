#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Runtime, Window, WindowBuilder, WindowUrl};
use window_vibrancy::{apply_blur, apply_mica, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct InteropPayload {
    channel: String,
    data: serde_json::Value,
}

/// 统一应用玻璃/毛玻璃特效
fn apply_glass_effect<R: Runtime>(window: &Window<R>) {
    #[cfg(target_os = "macos")]
    let _ = apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None);

    #[cfg(target_os = "windows")]
    {
        // 优先尝试 Mica (Win11)，如果失败则尝试 Acrylic 或 Blur
        if let Err(_) = apply_mica(window, None) {
            let _ = apply_blur(window, Some((18, 18, 18, 125)));
        }
    }
    
    // Linux 暂不支持原生毛玻璃特效，通常依赖合成器（Compositor）处理透明度
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 获取主窗口
            if let Some(main_window) = app.get_window("main") {
                apply_glass_effect(&main_window);
                
                #[cfg(any(windows, target_os = "linux"))]
                let _ = set_shadow(&main_window, true);
            }

            let app_handle = app.handle();
            
            // 全局消息总线：监听所有窗口发送的广播并同步到所有窗口
            app.listen_global("omni-broadcast", move |event| {
                if let Ok(payload) = serde_json::from_str::<InteropPayload>(event.payload().unwrap_or("{}")) {
                    let _ = app_handle.emit_all("omni-sync", payload);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_app_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn create_app_window<R: Runtime>(
    app_handle: tauri::AppHandle<R>, 
    app_id: String, 
    url: String, 
    title: String
) -> Result<(), String> {
    let window_url = WindowUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?);
    
    let window = WindowBuilder::new(
        &app_handle,
        &app_id,
        window_url
    )
    .title(title)
    .inner_size(800.0, 600.0)
    .transparent(true)
    .decorations(false) // 实现自定义标题栏视觉
    .build()
    .map_err(|e| format!("Failed to build window: {}", e))?;

    apply_glass_effect(&window);
    
    #[cfg(any(windows, target_os = "linux"))]
    let _ = set_shadow(&window, true);
    
    Ok(())
}
