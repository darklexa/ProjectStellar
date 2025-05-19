import React, { useState } from "react";
import { invoke } from "./soroban";

const CONTRACT = process.env.REACT_APP_CONTRACT_ID;

export default function App() {
  const [addr, setAddr] = useState("");
  const [amt, setAmt] = useState(0);
  const [out, setOut] = useState("");

  const call = async (fn) => {
    try {
      const res = await invoke(CONTRACT, fn, fn === "balance" ? { who: addr } : fn==="transfer" ? { from: addr, to: addr, amount: amt } : { to: addr, amount: amt });
      setOut(JSON.stringify(res, null, 2));
    } catch (e) {
      setOut(e.toString());
    }
  };

  return (
    <div style={{ padding: 20 }}>
      <h1>MicroLend UI</h1>
      <input placeholder="Address" onChange={e => setAddr(e.target.value)} /><br/>
      <input type="number" placeholder="Amount" onChange={e => setAmt(Number(e.target.value))} /><br/>
      <button onClick={()=>call("mint")}>Mint</button>
      <button onClick={()=>call("transfer")}>Transfer</button>
      <button onClick={()=>call("freeze_account")}>Freeze</button>
      <button onClick={()=>call("unfreeze_account")}>Unfreeze</button>
      <button onClick={()=>call("balance")}>Balance</button>
      <pre>{out}</pre>
    </div>
  );
}
