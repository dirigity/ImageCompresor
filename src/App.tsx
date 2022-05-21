import { Route, Routes } from "solid-app-router";
import { Component, lazy } from "solid-js";

const StartMenu = lazy(() => import("./pages/StartMenu"));
const Encode = lazy(() => import("./pages/Encode"));
const Decode = lazy(() => import("./pages/Decode"));

const App: Component = () => {
    return (
        <Routes>
            <Route path="/" element={<StartMenu />} />
            <Route
                path="/encode"
                element={<Encode />}
            /><Route
                path="/decode"
                element={<Decode />}
            />
        </Routes>
    );
};

export default App;
