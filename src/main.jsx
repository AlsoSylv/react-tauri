import React from "react";
import ReactDOM from "react-dom/client";
import "./style.css";
import "./logic"
import { Selects, Page } from "./logic";

function Base(props) {
    const element = (
    <div id="base">
      <h1>This is {props.prompt}!</h1>
      <h2>{props.prompt} is very painful</h2>
      <Selects />
      <Page />
    </div>
  );
  return element;
}


const root = ReactDOM.createRoot(document.getElementById('root'));
const element = <Base prompt="React"/>;
root.render(element)
