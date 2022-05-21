// /* @refresh reload */
// import { render } from 'solid-js/web';

import './index.css';
// import StartMenu from './StartMenu';

// render(() => <StartMenu />, document.getElementById('root') as HTMLElement);


import { render } from "solid-js/web";
import { Router } from "solid-app-router";
import App from "./App";

render(
    () => (
        <Router>
            <App />
        </Router>
    ),
    document.getElementById("root") as HTMLElement
);

console.log("start done")