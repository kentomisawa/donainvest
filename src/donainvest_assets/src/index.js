import { donainvest } from "../../declarations/donainvest";

const getIdentity = async () => {
  const whitelist = [
    require("../../../.dfx/local/canister_ids.json").generalized_dex.local
  ];
  const host = "http://127.0.0.1:8000";
  const connected = await window.ic.plug.isConnected();
  if (!connected) window.ic.plug.requestConnect({ whitelist, host });
  if (connected && !window.ic.plug.agent) {
    window.ic.plug.createAgent({ whitelist, host })
  }  
}

getIdentity()

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);

  // Interact with foo actor, calling the greet method
  const greeting = await donainvest.greet(name);

  button.removeAttribute("disabled");

  document.getElementById("greeting").innerText = greeting;

  return false;
});
