import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [delayHours, setDelayHours] = useState(0);
  const [delayMinutes, setDelayMinutes] = useState(0);
  const [delaySeconds, setDelaySeconds] = useState(0);
  const [delayMilliseconds, setDelayMilliseconds] = useState(100);
  const [repeatEnabled, setRepeatEnabled] = useState(false);
  const [repeatCount, setRepeatCount] = useState(1);
  const [button, setButton] = useState("left");
  const [clickType, setClickType] = useState("single");
  const [useCurrentLocation, setUseCurrentLocation] = useState(true);
  const [positionX, setPositionX] = useState(0);
  const [positionY, setPositionY] = useState(0);
  const [isClicking, setIsClicking] = useState(false);

  const getTotalDelay = () => {
    return (
      delayHours * 3600000 +
      delayMinutes * 60000 +
      delaySeconds * 1000 +
      delayMilliseconds
    );
  };

  const startClicking = async () => {
    setIsClicking(true);
    await invoke("click_custom", {
      position: useCurrentLocation ? null : { x: positionX, y: positionY },
      delayMs: getTotalDelay(),
      repeat: repeatEnabled ? repeatCount : null,
      button,
      clickType
    });
    setIsClicking(false);
  };

  const stopClicking = async () => {
    await invoke("stop_playing");
    setIsClicking(false);
  };

  return (
    <main style={{ padding: 20, fontFamily: "sans-serif", maxWidth: 400, margin: "0 auto" }}>
      <h2>Auto Clicker</h2>

      <fieldset style={{ marginBottom: 10 }}>
        <legend>Click interval</legend>
        <input type="number" value={delayHours} onChange={e => setDelayHours(Number(e.target.value))} style={{ width: 40 }} /> hours
        <input type="number" value={delayMinutes} onChange={e => setDelayMinutes(Number(e.target.value))} style={{ width: 40, marginLeft: 8 }} /> mins
        <input type="number" value={delaySeconds} onChange={e => setDelaySeconds(Number(e.target.value))} style={{ width: 40, marginLeft: 8 }} /> secs
        <input type="number" value={delayMilliseconds} onChange={e => setDelayMilliseconds(Number(e.target.value))} style={{ width: 60, marginLeft: 8 }} /> ms
      </fieldset>

      <fieldset style={{ marginBottom: 10 }}>
        <legend>Click options</legend>
        <label>Mouse button:
          <select value={button} onChange={e => setButton(e.target.value)} style={{ marginLeft: 8 }}>
            <option value="left">Left</option>
            <option value="right">Right</option>
            <option value="middle">Middle</option>
          </select>
        </label>
        <br />
        <label>Click type:
          <select value={clickType} onChange={e => setClickType(e.target.value)} style={{ marginLeft: 8 }}>
            <option value="single">Single</option>
            <option value="double">Double</option>
          </select>
        </label>
      </fieldset>

      <fieldset style={{ marginBottom: 10 }}>
        <legend>Click repeat</legend>
        <label>
          <input type="radio" checked={repeatEnabled} onChange={() => setRepeatEnabled(true)} /> Repeat
          <input type="number" value={repeatCount} onChange={e => setRepeatCount(Number(e.target.value))} disabled={!repeatEnabled} style={{ width: 60, marginLeft: 8 }} /> times
        </label>
        <br />
        <label>
          <input type="radio" checked={!repeatEnabled} onChange={() => setRepeatEnabled(false)} /> Repeat until stopped
        </label>
      </fieldset>

      <fieldset style={{ marginBottom: 10 }}>
        <legend>Cursor position</legend>
        <label>
          <input type="radio" checked={useCurrentLocation} onChange={() => setUseCurrentLocation(true)} /> Current location
        </label>
        <br />
        <label>
          <input type="radio" checked={!useCurrentLocation} onChange={() => setUseCurrentLocation(false)} /> Pick location
          <input type="number" value={positionX} onChange={e => setPositionX(Number(e.target.value))} style={{ width: 60, marginLeft: 8 }} disabled={useCurrentLocation} />
          <input type="number" value={positionY} onChange={e => setPositionY(Number(e.target.value))} style={{ width: 60, marginLeft: 4 }} disabled={useCurrentLocation} />
        </label>
      </fieldset>

      <button onClick={startClicking} disabled={isClicking} style={{ marginRight: 10 }}>Start (F6)</button>
      <button onClick={stopClicking} disabled={!isClicking}>Stop (F6)</button>
    </main>
  );
}
