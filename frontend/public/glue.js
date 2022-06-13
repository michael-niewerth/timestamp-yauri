const invoke = window.__TAURI__.invoke
export async function invokeBtn(id) {
    return await invoke("btn",{id:id});
}