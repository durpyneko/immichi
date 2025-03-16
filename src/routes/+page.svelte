<script lang="ts">
  import { toggleState } from "$lib/stores/toggleBgStore";
  import { onMount } from "svelte";

  import Titlebar from "$lib/components/Titlebar.svelte";
  import Settings from "$lib/components/Settings.svelte";

  let settingsOpen: boolean = false;

  onMount(() => {
    toggleState.subscribe((state) => {
      document.body.classList.toggle("bg-on", state === "toggle_on");
    });
  });

  let IMAGE_ARRAY = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
  ];

  function toggle_settings() {
    settingsOpen = !settingsOpen;
  }
</script>

<Titlebar />
<div style="margin-bottom: 42px;"></div>
<div
  class="VStack"
  style=" box-sizing: border-box; width: 100%; height: 100%; padding: 8px; gap: 10px; overflow-y: auto;"
>
  <div
    style="display: flex;
    align-items: center; height: 40px; gap: 8px"
  >
    <div
      style="display: flex;
    align-items: center; height: 40px; gap: 0px; width: 100%"
    >
      <input
        class="Input"
        placeholder="Search..."
        style="width: 100%; border-top-right-radius: 0px; border-bottom-right-radius: 0px; border-right-width: 0px"
      />
      <button
        class="Button"
        style="border-top-left-radius: 0px; border-bottom-left-radius: 0px; border-left-width: 0px"
      >
        <span class="Icon material-symbols-rounded"> search </span>
      </button>
    </div>
    <button class="Button" onclick={toggle_settings}>
      <span class="Icon material-symbols-rounded"> settings </span>
    </button>
  </div>
  {#if settingsOpen}
    <Settings />
  {/if}
  <div class="Gallery">
    <!-- Box  -->
    {#each IMAGE_ARRAY as image}
      <div class="Box Card Center" style="min-height: 200px; min-width: 150px;">
        {image}
      </div>
    {/each}
  </div>
</div>

<style>
</style>
