import React, { useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { wrap } from "comlink";

function App() {
  const worker = new Worker(new URL("./halo-worker", import.meta.url), {
    name: "halo-worker",
    type: "module",
  });
  const workerApi = wrap<import("./halo-worker").HaloWorker>(worker);
  // const [ans, setAns] = useState(0);

  async function test() {
    console.log(
      "navigator.hardwareConcurrency: ",
      navigator.hardwareConcurrency
    );

    const start = performance.now();
    const diff_js = await workerApi.get_play_diff();
    console.log("in between", diff_js);
    const t_get_play_diff = performance.now() - start;
    console.log("t_get_play_diff: ", t_get_play_diff);
    const proof = await workerApi.prove_play();
    const t_prove_play = performance.now() - t_get_play_diff;
    console.log("t_prove_play: ", t_prove_play);
    console.log("outside proof", proof);
    const verification = await workerApi.verify_play(proof, diff_js);
    const t_verify_play = performance.now() - t_prove_play;
    console.log("t_prove_play: ", t_verify_play);
    console.log("verified", verification);
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        {/* {ans} */}
        <button onClick={test}>test</button>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
