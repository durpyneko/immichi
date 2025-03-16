<script lang="ts">
  import { toggleState } from "$lib/stores/toggleBgStore";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function toggle() {
    toggleState.update((state) => {
      const newState = state === "toggle_off" ? "toggle_on" : "toggle_off";

      document.body.style.opacity = "0"; // out

      setTimeout(() => {
        document.body.classList.toggle("bg-on", newState === "toggle_on");
        document.body.style.opacity = "1"; // in
      }, 50);

      return newState;
    });
  }

  onMount(() => {
    toggleState.subscribe((state) => {
      document.body.classList.toggle("bg-on", state === "toggle_on");
    });
  });

  let win: Awaited<ReturnType<typeof getCurrentWindow>>;

  async function close_app() {
    await getCurrentWindow().close();
  }
  async function expand_app() {
    win = getCurrentWindow();
    (await win.isMaximized()) ? await win.unmaximize() : await win.maximize();
  }

  async function minimize_app() {
    await getCurrentWindow().minimize();
  }
</script>

<div class="Titlebar" data-tauri-drag-region>
  <span data-tauri-drag-region>Immichi</span>
  <div
    class="HStack"
    style="align-items: center;flex-direction: row-reverse; gap: 10px;"
  >
    <button class="unstyled Icon" onclick={close_app}>
      <span class="material-symbols-rounded close-btn"> close </span>
    </button>
    <button class="unstyled Icon" onclick={expand_app}>
      <span class="material-symbols-rounded expand-btn"> expand_content </span>
    </button>
    <button class="unstyled Icon" onclick={minimize_app}>
      <span class="material-symbols-rounded minimize-btn">
        horizontal_rule
      </span>
    </button>
    <button class="unstyled" onclick={toggle}>
      <!-- .Button style="padding: 2px;" -->
      <span class="material-symbols-rounded tgl-bg-span">
        {$toggleState === "toggle_off" ? "toggle_off" : "toggle_on"}
      </span>
    </button>
    <span style="padding-right: 10px" data-tauri-drag-region
      >System &thinsp;<span class="dot green" data-tauri-drag-region
      ></span></span
    >
  </div>
</div>

<style>
  .Titlebar {
    position: fixed;
    top: 0;
    z-index: 1000;
    box-sizing: border-box;
    background-color: var(--bg-parent);
    border: var(--border);
    height: 40px;
    width: 100%;
    user-select: none;
    align-items: center;
    padding-left: 10px;
    display: flex;
    justify-content: space-between;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }

  .close-btn:hover {
    background-color: red;
  }
</style>
