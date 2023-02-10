<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import { invoke } from '@tauri-apps/api/tauri'
    import { select_options } from 'svelte/internal';
    import { event } from '@tauri-apps/api';
    import { SlideToggle } from '@skeletonlabs/skeleton';

  let list = ''
  let process_btn
  let proc_optn

  let wlMode: boolean = false;
  let hfMode: boolean = false;
  let feMode: boolean = false;

  async function updateConfigOnStartup() {
    wlMode = await invoke('update_bools', {categ: "preferences", params: "whitelist"})
    hfMode = await invoke('update_bools', {categ: "preferences", params: "hyperfocus"})
    feMode = await invoke('update_bools', {categ: "preferences", params: "forceExit"})
  }

  updateConfigOnStartup()

  async function wltoggle() {
    let get_bool = !wlMode
    console.log("Toggle pressed!")
    await invoke(
      'listen_for_toggle_change',
      {categ: "preferences", params: "whitelist", boolean: get_bool }
    )
  }

  async function hftoggle() {
    let get_bool = !hfMode
    console.log("Toggle pressed!")
    await invoke(
      'listen_for_toggle_change',
      {categ: "preferences", params: "hyperfocus", boolean: get_bool }
    )
  }

  async function fetoggle() {
    let get_bool = !feMode
    console.log("Toggle pressed!")
    await invoke(
      'listen_for_toggle_change',
      {categ: "preferences", params: "forceExit", boolean: get_bool }
    )
  }

  function removeOptions(selectElement) {
   var i, L = selectElement.options.length - 1;
   for(i = L; i >= 0; i--) {
      selectElement.remove(i);
   }
}

  async function getList() {
    var list = await invoke('print_blocklist')
  }

  async function getProcs() {
    var procs = await invoke('print_process')
    var opts = document.getElementById("procOptions")
    
    for(var i = 0; i < procs.length; i++) {
      var opt = procs[i];
      var el = document.createElement("option");
      el.textContent = opt;
      el.value = opt;
      
      opts?.appendChild(el);
    } 

    process_btn = await document.querySelector('#process_btn');
    proc_optn = await document.querySelector('#procOptions');
  }

  async function saveEntry() {
    console.log(proc_optn.value);
    await invoke('save_entry', { entry: proc_optn.value })
    await updateBlocklist()
  };

  let block_optn
  let block_btn
  
  async function updateBlocklist() {
    var bList = await invoke('print_blocklist')
    var opts = document.getElementById('procBlocklist')

    removeOptions(opts)

    for(var i = 0; i < bList.length; i++) {
      var opt = bList[i];
      var el = document.createElement("option");
      el.textContent = opt;
      el.value = opt;

      opts?.appendChild(el);
    }

    block_btn = await document.querySelector('#block_btn');
    block_optn = await document.querySelector('#procBlocklist');
  }

  async function removeEntry() {
    const selectedValues = [].filter
      .call(block_optn, option => option.selected)
      .map(option => option.text);
      console.log(selectedValues);

    await invoke('remove_entry', { entries: selectedValues })
    await updateBlocklist()
  };

  updateBlocklist()
  getProcs()
</script>

<svelte:head>
  <link href="https://fonts.googleapis.com/css?family=Lexend" rel="stylesheet">
  <link href="https://fonts.googleapis.com/css?family=Oswald" rel="stylesheet">
</svelte:head>

<!-- <div class=screen>
  <div class=workspace>
    <div class="container">
      <div class="Blocklist">
        <select name="procBlocklist" id="procBlocklist" multiple>
        </select>
      </div>
      <div class="Navs">
        <a style="color: inherit; text-decoration: none;" href=".."><button class="variant-filled-primary btn-base" id="block_btn">
          Dashboard
        </button></a>
      </div>
      <div class="Processes">
        <select name="procOptions" id="procOptions">
        </select>
      </div>
      <div class="procButtons">
        <button class="btn variant-filled-primary btn-base" id="process_btn" on:click={saveEntry}>
          Save to Blocklist
        </button>
      </div>
      <div class="blockButtons">
        <button class="btn variant-filled-primary btn-base" id="block_btn" on:click={removeEntry}>
          Remove from Blocklist
        </button>
      </div>
    </div>
  </div>
</div> -->

<div class="container">
  <div class="Sidebar">
    <div class="Momentum"><section><a style="color: inherit; text-decoration: none;" href=".."><p2>MOMENTUM</p2></a></section></div>
    <div class="QuickSettings"><section2><SlideToggle bind:checked={wlMode} on:click={wltoggle} size='sm' id="wtlist_toggle"><p4>Whitelist Mode</p4></SlideToggle><SlideToggle bind:checked={hfMode} on:click={hftoggle} size='sm' accent="bg-error-600" disabled={feMode} id="hf_toggle"><p4>Hyper-Focus Mode <span style="color: red">(!)</span></p4></SlideToggle><SlideToggle bind:checked={feMode} on:click={fetoggle} size='sm' accent="bg-error-600" disabled={!hfMode} id="felist_toggle"><p4>Force Exit Mode <span style="color: red">(!)</span></p4></SlideToggle></section2></div>
    <div class="RunningProcs">
      <span><select name="procOptions" id="procOptions"></select></span>

      <span><button class="btn variant-filled-primary btn-base" id="process_btn" on:click={saveEntry}>
        Save to Blocklist
      </button></span>
    </div>
    <div class="RemoveFrom">
      <button class="btn variant-filled-primary btn-base" id="block_btn" on:click={removeEntry}>
      Remove from Blocklist
      </button>
    </div>
  </div>
  <div class="Blocklist">
    <div class="Titlebar"></div>
    <div class="MainArea">
      <select name="procBlocklist" id="procBlocklist" multiple></select>
    </div>
  </div>
</div>


<style>
  p2 {
    color: #e0e0e0;
    font-size: 40px;
    font-family: Lexend;
  }
  p4 {
    font-family: Oswald;
    font-size: 20px;
  }
  section {
    align-items: center;
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    justify-content: center;
    height: 100%;
    user-select: none;
  }
  section2 {
    padding-left: 20px;
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    justify-content: center;
    height: 100%;
    user-select: none;
  }
  .container {  display: grid;
    grid-template-columns: 0.5fr 1.5fr;
    grid-template-rows: 1fr;
    grid-auto-columns: 1fr;
    gap: 0px 0px;
    grid-auto-flow: row;
    grid-template-areas:
      "Sidebar Blocklist";
    width: 1372px;
    height: 632px;
    margin: 0px;
  }

  .Sidebar {  display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: 0.6fr 1.0fr 1.0fr 2.0fr;
    gap: 0px 0px;
    grid-auto-flow: row;
    grid-area: Sidebar;
  }

  .Momentum { grid-area: 1 / 1 / 2 / 2; background-image: linear-gradient(#0081FF, #9585FF); margin: 0px 5px 5px 0px; border-radius: 0px 0px 13px 0px; }

  .QuickSettings {
    grid-area: 2 / 1 / 3 / 2;
    background-color: white;
    margin: 10px;
    border-radius: 13px 13px 13px 13px;
  }

  .RunningProcs {
    display: flex;
    flex-wrap: wrap;
    grid-area: 3 / 1 / 4 / 2;
    background-color: white;
    margin: 10px;
    border-radius: 13px 13px 13px 13px;
    padding: 20px;
    justify-content: center;
  }

  .RemoveFrom {
    grid-area: 4 / 1 / 5 / 2;
    align-items: center;
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    justify-content: center;
    height: 100%;
    user-select: none;
    padding: 10px;
  }

  .RemoveFrom button {
    border-radius: 13px;
    width: 100%;
    height: 100%;
    font-size: 32px;
  }

  button {
    background-color: #dce1ee;
    color: #000;
    font-family: Oswald;
    font-size: 20px;
    word-wrap: break-word;
  }

  .Blocklist {  display: grid;
    background-color: white;
    grid-template-columns: 1fr;
    grid-template-rows: 0px 602px;
    gap: 0px 0px;
    grid-auto-flow: column;
    grid-template-areas:
      "Titlebar"
      "MainArea";
    grid-area: Blocklist;
  }

  .Titlebar { grid-area: Titlebar; }

  .MainArea {
    grid-area: MainArea;
    align-items: center;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    height: 100%;
    user-select: none;
    padding: 50px 0px 5px 10px;
  }

  .Blocklist select {
    box-sizing: border-box;
    appearance: none;
    height: 572px;
    border: transparent;
    margin: 0px;
    padding: 0px;
    border-radius: 0px;
    outline: none;
    background-color: #FFFFFF;
  }

  select {
    box-sizing: border-box;
    appearance: none;
    background-color: #FFFFFF;
    font-family: Oswald;
  }

  .RunningProcs select {
    text-align: center;
  }

  select[multiple]::focus {
    stroke: transparent;
    background-color: #FFFFFF;
  }
</style>