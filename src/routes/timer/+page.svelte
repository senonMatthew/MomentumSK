<script>
  import Timer from "$lib/timer.svelte";
    import { app } from "@tauri-apps/api";
  import { appWindow } from '@tauri-apps/api/window'
  
</script>
 
<main data-tauri-drag-region="true" class="windowContainer">
  <Timer />
</main>

<style>
  .windowContainer {
    
  }
</style>
  