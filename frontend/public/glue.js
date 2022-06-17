const invoke = window.__TAURI__.invoke
export async function invokeBtn(id, activeId) {
   return await invoke("btn",{id:id, activeId:activeId});
}