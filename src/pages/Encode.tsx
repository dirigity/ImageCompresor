import { invoke } from '@tauri-apps/api';
import { Component, createSignal } from 'solid-js';
import styles from './StartMenu.module.css';

const StartMenu: Component = () => {

    const [ready, setReady] = createSignal(true);


    return (
        <header class={styles.header}>

            <div class="canvasHolder">
                <canvas id="src_canvas" ></canvas>
                <canvas id="dst_canvas" ></canvas>
            </div>

            <div id="buttonHolder">
                <button onclick={
                    () => {
                        invoke('pick_file', {}).then(console.log)
                    }
                }>Load file</button>
                <button disabled={ready()} >Start conversion</button>

            </div>



        </header>
    );
};

import { emit, listen } from '@tauri-apps/api/event'

listen("draw_src", (e) => {

    console.log("hemos recivido un draw src",e)
})


export default StartMenu;
