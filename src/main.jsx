import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";

document.addEventListener('contextmenu', event => {
  if(window.location.hostname === "tauri.localhost") {
    event.preventDefault();
  }
})

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/*" element={<App/>}/>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);