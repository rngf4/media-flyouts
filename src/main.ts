import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

window.addEventListener("DOMContentLoaded", async () => {
  let listener = await listen("event", (event) => {
    console.log(event);
  });
});
