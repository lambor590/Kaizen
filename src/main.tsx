import { render } from "preact";
import { invoke } from "@tauri-apps/api";
import { BrowserRouter } from "react-router-dom";
import App from "./App";
import "./index.css";

render(
    <BrowserRouter>
        <App />
    </BrowserRouter>,
    document.getElementById("root")!
);
invoke('close_splash');