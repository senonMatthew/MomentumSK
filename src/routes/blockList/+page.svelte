<script>
  import Blocklist from "$lib/blocklist.svelte";
  import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
  import '@skeletonlabs/skeleton/styles/all.css';
  import "../../app.postcss";
  import { appWindow } from '@tauri-apps/api/window'

  function titleMinimize() {
    appWindow.minimize()
  }

  function titleClose() {
    appWindow.hide()
  }

</script>

<div data-tauri-drag-region="true" class="titlebar" id="titlebar-bar">
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="titlebar-button" id="titlebar-minimize" on:click="{titleMinimize}">
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="titlebar-button" id="titlebar-close" on:click="{titleClose}">
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
</div>
<main class="layoutContainer">
  <Blocklist />
</main>
<style>
  .layoutContainer {
    background-color: #ececec;
  }
  .titlebar {
    height: 30px;
    background: rgba(255, 255, 255, 0);
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    padding-right: 10px;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
  }
  .titlebar-button:hover {
    background: rgba(133, 133, 133, 0.404);
  }
</style>
