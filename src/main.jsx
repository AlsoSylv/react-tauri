import React from "react";
import ReactDOM from "react-dom/client";
import "./style.css";
import "./logic"
import { Selects, Page } from "./logic";

function Base(props) {
    const element = (
    <div id="base">
      <Selects />
      <Page />
    </div>
  );
  return element;
}


const root = ReactDOM.createRoot(document.getElementById('root'));
const element = <Base />;
root.render(element)
