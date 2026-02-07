/**
 * OmniShell Bridge SDK
 * 核心支持库：前端引用此文件即可获得原生能力
 */
const invoke = window.__TAURI__?.invoke || (() => console.warn('Not in OmniShell env'));
const { emit, listen } = window.__TAURI__?.event || { emit:()=>{}, listen:()=>{} };
const { appWindow } = window.__TAURI__?.window || {};

export const Omni = {
    window: {
        open: async (config) => {
            await invoke('create_app_window', {
                appId: config.id || `app-${Date.now()}`,
                url: config.url,
                title: config.title || 'Omni App'
            });
        },
        minimize: () => appWindow?.minimize(),
        toggleMaximize: async () => {
            const isMax = await appWindow?.isMaximized();
            isMax ? appWindow?.unmaximize() : appWindow?.maximize();
        },
        close: () => appWindow?.close(),
        startDrag: () => appWindow?.startDragging(),
    },
    bus: {
        emit: async (channel, data) => {
            await emit('omni-broadcast', { channel, data });
        },
        on: (targetChannel, callback) => {
            listen('omni-sync', (event) => {
                const payload = event.payload;
                if (payload && payload.channel === targetChannel) {
                    callback(payload.data);
                }
            });
        }
    }
};
