import type { Component } from 'solid-js';

import styles from './StartMenu.module.css';

const StartMenu: Component = () => {
  return (
    <header class={[styles.backdrop, styles.header].join(" ")}>
      <button onclick={
        () => {
          window.location.href = "./decode"
        }
      }>Decode</button>
      <button onclick={
        () => {
          window.location.href = "./encode"
        }
      }>Encode</button>
    </header>
  );
};

export default StartMenu;
