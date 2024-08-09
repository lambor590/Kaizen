import { render } from "preact";
import { invoke } from "@tauri-apps/api";
import App from "./App";
import "./index.css";

render(<App />, document.getElementById("root")!)
invoke('close_splash')