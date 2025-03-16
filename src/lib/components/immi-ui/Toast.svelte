<script lang="ts">
  import { fly } from "svelte/transition";
  import { sineOut } from "svelte/easing";

  let toasts: Array<{
    id: number;
    message: string;
    backgroundColor: string;
    duration: number;
    visible: boolean;
  }> = [];
  let nextId = 0;

  export function showToast(options: {
    msg: string;
    time?: number;
    bgColor?: string;
  }): void {
    const id = nextId++;
    const duration = options.time ?? 4000;
    const backgroundColor = options.bgColor ?? "rgba(0, 200, 0, 0.7)";

    toasts = [
      ...toasts,
      { id, message: options.msg, backgroundColor, duration, visible: true },
    ];

    setTimeout(() => {
      toasts = toasts.filter((toast) => toast.id !== id);
    }, duration);
  }
</script>

<div class="ToastContainer">
  {#each toasts as toast, index (toast.id)}
    <div
      class="Box Toast Center"
      style="background-color: {toast.backgroundColor};"
      in:fly={{ y: 100, duration: 300, easing: sineOut }}
      out:fly={{ y: 100, duration: 300, easing: sineOut }}
    >
      {toast.message}
    </div>
  {/each}
</div>

<style>
  .ToastContainer {
    position: fixed;
    z-index: 1000;
    bottom: 0px;
    width: 100%;
    display: flex;
    justify-content: center;
    flex-direction: column;
  }

  .Toast {
    position: relative;
    margin-bottom: 10px;
    padding: 10px;
    border-radius: 5px;
    color: white;
    font-size: 14px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    display: flex;
    justify-content: center;
    align-items: center;
    max-width: fit-content;
  }
</style>
