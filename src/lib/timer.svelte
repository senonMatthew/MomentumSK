<script lang="ts">
    import { beforeNavigate } from '$app/navigation';
  import { app } from '@tauri-apps/api';
    import { invoke } from '@tauri-apps/api/tauri';
  import { appWindow, LogicalPosition, PhysicalSize, WebviewWindow } from '@tauri-apps/api/window'
    import { handle_promise } from 'svelte/internal';
    import timerBackground from '/static/timerBackground.jpg'

  let buttonClick = false;
  let timerState = false;

  // async function bgImg() {
  //   let bg = await document.getElementById('mainWindow');
    
  //   bg.style.backgroundImage = timerBackground;
  // }

  // bgImg();

  appWindow.setSize(new PhysicalSize(470, 516));
  appWindow.setPosition(new LogicalPosition(1415, 493));

  async function hideApp() {
    appWindow.hide();
  }

  async function autoHide() {
    let secs = 3;
    document.getElementById("textSmall").innerHTML = "hiding in.."
    document.getElementById("textLarge").innerHTML = `${secs}`
    var timer = setInterval(function() {
      secs -= 1;
      document.getElementById("textLarge").innerHTML = `${secs}`
    }, 1000)
    setTimeout(function() {
      clearInterval(timer);
      document.getElementById("textSmall").innerHTML = "keep your"
      document.getElementById("textLarge").innerHTML = "Focus"
    }, 3250);
    setTimeout(hideApp, 3000);
  }
  async function timer(){
    console.log(`State is at ${timerState}`)
    document.getElementById("navbutton").setAttribute('href', 'javascript:await hideApp()')
    var min = 0.5;
    var sec = min * 60;
    var secClone = sec;

    if (!timerState) {
      timerState = true;
      autoHide();
      let x = invoke('start_timer', { time: sec });
      console.log(`Switching on!`)
      
      if (x) {
        console.log("true")
      } else {
        console.log("false")
      };

      var timer = setInterval(function(){
        var minutes = Math.floor((sec-1) / 60);
        var seconds = Math.floor((sec-1) % 60);

        document.getElementById('timer').innerHTML=`${minutes}`.padStart(2, '0') + ":" + `${seconds}`.padStart(2, '0');
        sec--;
        console.log(`${minutes}:${seconds} - ${sec}`)
          if (sec < 0) {
              document.getElementById("navbutton").setAttribute('href', '..')
              clearInterval(timer);
              timerState = false;
              
              var minutesClone = Math.floor((secClone-1) / 60);
              var secondsClone = Math.floor((secClone-1) % 60);
              document.getElementById('timer').innerHTML=`${minutesClone}`.padStart(2, '0') + ":" + `${secondsClone}`.padStart(2, '0');
              document.getElementById("textSmall").innerHTML = "start"
          }
      }, 1000);
    }
  }



</script>

<svelte:head>
  <link href="https://fonts.googleapis.com/css?family=Lexend" rel="stylesheet">
  <link href="https://fonts.googleapis.com/css?family=Oswald" rel="stylesheet">
</svelte:head>

<div class="mainWindow" id="mainWindow">
  <div class="container">
    <div class="Nav">
      <section><a href=".." id="navbutton"><div class="back_button"><img src="backarrow.png" alt="Go back" height="60" width="60"/></div></a></section>
    </div>
    <div class="FocusButton">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="top_coin" id="timer_button" on:click={timer}><div class="face buttonOuter"id="buttonOuter" ><div class="face buttonInner"id="buttonInner" ><section id="textButton" ><p2 class="textSmall" id="textSmall">start</p2><p3 class="textLarge" id="textLarge">Focus</p3></section></div></div></div>
    </div>
    <div class="Timer">
      <section><p4 id="timer">0:30</p4></section>

    </div>
  </div>
</div>

<style>
    p4 {
      font-family: 'Lexend';
      color: #FFFFFF;
      font-size: 90px;
      margin: 0px;
    }
    section {
      margin-top: 10px;
      align-items: center;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      justify-content: center;
      height: 100%;
      user-select: none;
    }

    .mainWindow {
      height: 516px;
      width: 470px;
      background: 0px -10px/155% 125% url('/static/timerBackground.jpg');
      /* background-image: 0px -10px/155% 125% url('/static/timerBackground.jpg'); */
      border-radius: 35px;
    }

    .container {  display: grid;
      grid-template-rows: 0.1fr 0.4fr 2.8fr 0.10fr 0.6fr;
      grid-auto-columns: 1fr;
      gap: 0px 0px;
      grid-auto-flow: row;
      width: 470px;
      height: 516px;
    }

    .Nav {
      grid-area: 2 / 1 / 3 / 2;
      display: flex;
      align-items: flex-end;
      justify-content: right;
      padding-right: 10px;
    }
    
    .back_button {
      align-items: center;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      justify-content: center;
    }

    .back_button:hover {
      border-radius: 50%;
      background-color: rgba(0, 0, 0, 0.12);
    }

    .FocusButton {
      grid-area: 3 / 1 / 4 / 2;
      align-items: center;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      justify-content: center;
      height: 100%;
      position: relative;
    }

    .buttonOuter {
      align-items: center;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      justify-content: center; 
      box-shadow:0px 0px 0px 3px #8785ff;
      background-color: #FFFFFF;
      border-radius: 50%;
      height: 280px;
      width: 280px;

      animation: 10s;
    }
    .buttonInner {
      box-shadow:0px 0px 0px 1px #8785ff;
      background-color: #FFFFFF;
      border-radius: 50%;
      height: 240px;
      width: 240px;
    }

    .top_coin:active .buttonOuter {
      align-items: center;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      justify-content: center; 
      box-shadow:0px 5px 26px 2px rgba(0, 0, 0, 0.02) inset;
      background-color: #966eff;
      border-radius: 50%;
      height: 280px;
      width: 280px;

      animation: 10s;
    }

    .top_coin:active .buttonInner {
      box-shadow:0px 3px 3px 2px rgba(0, 0, 0, 0.12) inset;
      background-color: #595093;
      border-radius: 50%;
      height: 240px;
      width: 240px;
    }

    .textSmall {
      font-family: 'Lexend';
      font-size: 35px;
      margin: 0px;
      line-height: 0px;
    }
    .textLarge {
      font-family: 'Lexend';
      font-size: 60px;
      margin: 0px;
    }
    .textClicked {
      color: #FFFFFF;
    }
    .top_coin:hover .buttonOuter {
      animation-name: outerAnim;
      animation-duration: 200ms;
      animation-fill-mode: forwards;
    }
    .top_coin:hover .buttonInner {
      animation-name: innerAnim;
      animation-duration: 200ms;
      animation-fill-mode: forwards;
    }
    .top_coin:hover section {
      animation-name: textAnim;
      animation-duration: 200ms;
      animation-fill-mode: forwards;
    }

    @keyframes outerAnim {
      from {
        background-color: #FFFFFF;
        box-shadow:0px 0px 0px 3px #8785ff;
      } to {
        background-color: #c0b6ff;
        box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.02) inset;
      }
    }

    @keyframes innerAnim {
      from {
        background-color: #FFFFFF;
        box-shadow:0px 0px 0px 1px #8785ff;
      } to {
        background-color: #9585ff;
        box-shadow: 0px 5px 4px rgba(0 0 0 / 0.2) inset;
      }
    }

    @keyframes textAnim {
      from {
        color: #000000;
      } to {
        color: #FFFFFF;
      }
    }

    

    .Timer {
      grid-area: 4 / 1 / 5 / 2;
    }

</style>