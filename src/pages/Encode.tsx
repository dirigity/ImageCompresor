import { invoke } from '@tauri-apps/api';
import { Component, createSignal } from 'solid-js';
import styles from './StartMenu.module.css';

const [ready, setReady] = createSignal(false);
const [srcUrl, setSrcUrl] = createSignal("");
const [resUrl, setResUrl] = createSignal("");

const StartMenu: Component = () => {



    return (
        <div class={[styles.backdrop, styles.topDownFlex].join(" ")}>

            <div class={styles.canvasHolder}>
                <img class={styles.imgDisplay} alt={srcUrl()} src={srcUrl()} ></img>
                <img class={styles.imgDisplay} alt={resUrl()} src={resUrl()} ></img>
            </div>

            <div class={styles.buttonHolder}>
                <button onclick={
                    () => {
                        invoke('pick_file', {}).then(console.log)
                    }
                }>Load file</button>
                <button disabled={!ready()} onclick={
                    () => {
                        if (ready())
                            invoke('encode', {}).then(console.log)
                    }
                }>Start Encoding</button>
                <button onclick={
                    () => {
                        window.location.href = "./"
                    }
                }>Back</button>

            </div>

        </div>
    );
};

import { emit, listen } from '@tauri-apps/api/event'

listen("draw_src", (e: any) => {

    console.log("hemos recivido un draw_src", e)
    setReady(true);
    setSrcUrl("./src-tauri/" + e.payload)
})

listen("draw_res", (e: any) => {

    console.log("hemos recivido un draw_res", e)
    setResUrl("./src-tauri/" + e.payload)
})

export default StartMenu;
