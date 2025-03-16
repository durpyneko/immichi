<script lang="ts">
  import Divider from "$lib/components/immi-ui/Divider.svelte";
  import Toast from "$lib/components/immi-ui/Toast.svelte";

  let entry: string;
  let toastRef: Toast;

  let FolderEnties = [
    "D:\\homework\\Hiroi Kikuri",
    "D:\\homework\\Nekomata Okayu\\",
  ];

  async function add_entry() {
    if (entry != "") {
      if (FolderEnties.some((e) => e === entry.trim())) {
        toastRef.showToast({
          msg: "Duplicate entry!",
          bgColor: "rgba(255, 0, 0, 0.4)",
        });
        entry = "";
        return;
      }

      FolderEnties = [...FolderEnties, entry.trim()];
      toastRef.showToast({
        msg: "Added entry!",
        bgColor: "rgba(0, 200, 0, 0.4)",
      });
      entry = "";
    }
  }

  function remove_entry(target: string) {
    FolderEnties = FolderEnties.filter((FolderEntry) => FolderEntry !== target);
    toastRef.showToast({
      msg: "Removed entry!",
      bgColor: "rgba(0, 200, 0, 0.4)",
    });
  }
</script>

<Toast bind:this={toastRef} />

<div>
  <div class="VStack">
    <span style="font-size: 40px; user-select: none;">Settings</span>
    <span style="color: grey; user-select: none;"
      >Adjust your preferences here</span
    >
    <div style="padding-bottom: 20px;"></div>
    <span style="font-size: 20px; padding-bottom: 4px; user-select: none;"
      >Folders</span
    >
    <div class="Box VStack" style="padding: 6px; gap: 6px">
      <div class="HStack" style="gap: 6px">
        <input
          class="Input"
          placeholder="Directory..."
          style="width: 100%;"
          bind:value={entry}
        />
        <button class="Button" onclick={add_entry}
          ><span class="Icon material-symbols-rounded" style="font-size: 22px;">
            save
          </span></button
        >
      </div>
      {#each FolderEnties as FolderEntry}
        <div class="FolderContainer">
          <div class="FolderEntry Box HStack" style="gap: 6px">
            <span
              class="Icon material-symbols-rounded"
              style="user-select: none;"
            >
              folder
            </span>
            <div
              style="display: flex; justify-content: space-between; width: 100%; align-items: center;"
            >
              <span class="FolderEntryText" style="user-select: text;"
                >{FolderEntry}</span
              >
              <button
                class="Button FolderEntryButton"
                style="padding: 4px;"
                onclick={() => remove_entry(FolderEntry)}
              >
                <span
                  class="Icon material-symbols-rounded"
                  style="font-size: 22px;"
                >
                  delete
                </span>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
  <Divider style="margin-bottom: 4px;" />
</div>

<style>
  .FolderEntry {
    display: flex;
    align-items: center;
    /* justify-content: space-between; */
    padding: 4px;
    padding-top: 5px;
    padding-bottom: 5px;
    padding-left: 8px;
  }
  .FolderEntryButton:hover {
    background-color: rgba(255, 0, 0, 0.75);
    color: black;
  }
</style>
