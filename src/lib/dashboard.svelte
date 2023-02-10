<script lang="ts">
  import { Route } from 'tinro'; 
  import { SlideToggle } from '@skeletonlabs/skeleton';
  import { tauri } from '@tauri-apps/api';
  import { invoke } from '@tauri-apps/api/tauri'
  import { emit, listen } from '@tauri-apps/api/event';
  import { appWindow, PhysicalSize } from '@tauri-apps/api/window';
  import { moveWindow, Position } from "tauri-plugin-positioner-api";
  // import { exists, BaseDirectory, writeFile } from "@tauri-apps/api/fs";

  // const appDataDir = BaseDirectory.AppData;

  // function checkFileExists(path: string) {
  //   return exists(path, { dir: appDataDir });
  // };

  // function createNewSettings() {
  //   writeFile('settings.ini', "Momentum Settings\n-------------------\nwhitelist: false\nhyperfocus: false\nforceexit: false",{ dir: appDataDir });
  // }

  // async function main() {
  //   const filePath = 'settings.ini'
  //   const fileExists = await checkFileExists(filePath);
  //   if (!fileExists) {
  //     console.log("Settings file not found! Creating new settings!");
  //     createNewSettings();
  //     console.log("'settings.ini' has been created!");
  //     console.log(`It can be found in Roaming!`);
  //   }
  // }

  // main();
  appWindow.setSize(new PhysicalSize(1372, 632));
  moveWindow(Position.Center);

  invoke('new_start')

  let wlMode: boolean = false;
  let hfMode: boolean = false;
  let feMode: boolean = false;

  // const wtlist_toggle = document.getElementById("wtlist_toggle");
  // wtlist_toggle?.addEventListener("click", function() {
  //   console.log("Click!")
  //   invoke(
  //         'listen_for_toggle_change',
  //         { params: "whitelist", boolean: wlMode }
  //     );
  // });

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


  console.log("Fully loaded!");
</script>

<svelte:head>
  <link href="https://fonts.googleapis.com/css?family=Lexend" rel="stylesheet">
  <link href="https://fonts.googleapis.com/css?family=Oswald" rel="stylesheet">
</svelte:head>

<div class="container">
  <div class="Dashboard">
    <div class="LogoSettings">
      <div class="Banner" ><section><p2>MOMENTUM</p2></section></div>
      <div class="QSettings"><section2><SlideToggle bind:checked={wlMode} on:click={wltoggle} size='sm' disabled=true id="wtlist_toggle"><p4>Whitelist Mode</p4></SlideToggle><SlideToggle bind:checked={hfMode} on:click={hftoggle} size='sm' accent="bg-error-600" disabled={feMode} id="hf_toggle"><p4>Hyper-Focus Mode <span style="color: red">(!)</span></p4></SlideToggle><SlideToggle bind:checked={feMode} on:click={fetoggle} size='sm' accent="bg-error-600" disabled={!hfMode} id="felist_toggle"><p4>Force Exit Mode <span style="color: red">(!)</span></p4></SlideToggle></section2></div>
    </div>
    <div class="Data"></div>
  </div>
  <div class="NavButtons">
    <div class="Panel-A">
      <div class="Panel-A1"><a style="color: inherit; text-decoration: none;" href="/blockList"><section><img src="lock.png" height="100" width="100" alt="lock"><p>Block List</p></section></a></div>
      <div class="Panel-A2">
        <div class="Panel-A2-1"><section><img src="user.png" height="100" width="100" alt="user"><p style="color:#a0a0a0">Preferences &</p> <p style="color:#a0a0a0">Blocker Settings</p></section></div>
        <div class="Panel-A2-2"><section><img src="bell.png" height="100" width="100" alt="bell"><p style="color:#a0a0a0">Notification Settings</p></section></div>
      </div>
    </div>
    <div class="Panel-B">
      <div class="Panel-B1"><section><img src="notebook.png" height="100" width="100" alt="notebook"><p style="color:#a0a0a0">Session Annotations</p></section></div>
      <div class="Panel-B2">
        <div class="Panel-B2-1"><section><img src="table.png" height="100" width="100" alt="table"><p style="color:#a0a0a0">Interface Settings</p></section></div>
        <div class="Panel-B2-2"><section><img src="database.png" height="100" width="100" alt="database"><p style="color:#a0a0a0">Database Settings</p></section></div>
      </div>
    </div>
    <div class="Panel-C">
      <div class="Panel-C1"><section><img src="graph.png" height="100" width="100" alt="graph"><p style="color:#a0a0a0">Statistics</p></section></div>
      <div class="Panel-C2">
        <div class="Panel-C2-1"><section><img src="search.png" height="100" width="100" alt="search"><p style="color:#a0a0a0">Troubleshooting</p></section></div>
        <div class="Panel-C2-2"><section><img src="help.png" height="100" width="100" alt="help"><p style="color:#a0a0a0">Tutorials</p></section></div>
      </div>
    </div>
    <div class="Start"><a style="color: inherit; text-decoration: none;" href="/timer"><section><p1>Start using</p1><p3>MOMENTUM</p3></section></a></div>
  </div>
</div>

<style>
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
  p {
    font-family: Oswald;
  }
  p1 {
    color: #e0e0e0;
    font-family: Oswald;
  }
  p2 {
    color: #e0e0e0;
    font-size: 40px;
    font-family: Lexend;
  }
  p3 {
    color: #e0e0e0;
    font-size: 25px;
    font-family: Lexend;
  }
  p4 {
    font-family: Oswald;
    font-size: 20px;
  }
  .container {  display: grid;
  margin: 0px;
  grid-template-columns: 1fr;
  grid-template-rows: 239px 393fr;
  grid-auto-columns: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Dashboard"
    "NavButtons";
  width: 1372px;
  height: 632px;
}

.Dashboard {  display: grid;
  grid-template-columns: 393px 979px;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "LogoSettings Data";
  grid-area: Dashboard;
}

.LogoSettings {  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 89px 1fr;
  grid-auto-flow: row;
  grid-template-areas:
    "Banner"
    "QSettings";
  grid-area: LogoSettings;
}

.Banner { grid-area: Banner; background-image: linear-gradient(#0081FF, #9585FF); margin: 0px 5px 5px 0px; border-radius: 0px 0px 13px 0px;}

.QSettings { grid-area: QSettings; background-color: white; margin: 5px; border-radius: 13px 13px 0px 0px;}

.Data { grid-area: Data; background-color: white; margin: 0px 10px 5px 5px;}

.NavButtons {  display: grid;
  grid-template-columns: 393px 393px 393px 192px;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-A Panel-B Panel-C Start";
  grid-area: NavButtons;
}

.Panel-A {  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 196px 197px;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-A1"
    "Panel-A2";
  grid-area: Panel-A;
}

.Panel-A1 { grid-area: Panel-A1; background-color: white; margin: 5px;}
.Panel-A1:hover {background-color: #e0e0e0}

.Panel-A2 {  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-A2-1 Panel-A2-2";
  grid-area: Panel-A2;
}

.Panel-A2-1 { grid-area: Panel-A2-1; background-color: white; margin: 5px;}
.Panel-A2-1:hover {background-color: #e0e0e0}

.Panel-A2-2 { grid-area: Panel-A2-2; background-color: white; margin: 5px;}
.Panel-A2-2:hover {background-color: #e0e0e0}

.Panel-B {  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 196px 197px;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-B1"
    "Panel-B2";
  grid-area: Panel-B;
}

.Panel-B1 { grid-area: Panel-B1; background-color: white; margin: 5px;}
.Panel-B1:hover {background-color: #e0e0e0}

.Panel-B2 {  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-B2-1 Panel-B2-2";
  grid-area: Panel-B2;
}

.Panel-B2-1 { grid-area: Panel-B2-1; background-color: white; margin: 5px;}
.Panel-B2-1:hover {background-color: #e0e0e0}

.Panel-B2-2 { grid-area: Panel-B2-2; background-color: white; margin: 5px;}
.Panel-B2-2:hover {background-color: #e0e0e0}

.Panel-C {  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 196px 197px;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-C1"
    "Panel-C2";
  grid-area: Panel-C;
}

.Panel-C1 { grid-area: Panel-C1; background-color: white; margin: 5px;}
.Panel-C1:hover {background-color: #e0e0e0}

.Panel-C2 {  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Panel-C2-1 Panel-C2-2";
  grid-area: Panel-C2;
}

.Panel-C2-1 { grid-area: Panel-C2-1; background-color: white; margin: 5px;}
.Panel-C2-1:hover {background-color: #e0e0e0}

.Panel-C2-2 { grid-area: Panel-C2-2; background-color: white; margin: 5px;}
.Panel-C2-2:hover {background-color: #e0e0e0}

.Start { grid-area: Start; stroke: #9585FF; stroke-width: 2; background-image: linear-gradient(60deg, #7560FF, #3372ff); margin: 5px 10px 5px 5px;}
.Start:hover {background-image: linear-gradient(250deg, #7560FF, #3372ff); transition: 1s;}

</style>