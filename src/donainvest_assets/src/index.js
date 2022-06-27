import { donainvest } from "../../declarations/donainvest";

const getIdentity = async () => {
  const whitelist = [
    require("../../../.dfx/local/canister_ids.json").donainvest.local
  ];
  const host = "http://127.0.0.1:8000";
  const connected = await window.ic.plug.isConnected();
  if (!connected) window.ic.plug.requestConnect({ whitelist, host });
  if (connected && !window.ic.plug.agent) {
    window.ic.plug.createAgent({ whitelist, host })
  }  
}

window.show = (id) => {
  document.getElementById(id).style.display = "block"
}

window.hide = (id) => {
  document.getElementById(id).style.display = "none"
}

window.buy = (id, key, usd, donation) => {
  const donationRate = Number(document.getElementById(donation).value) / 100
  const value = Number(document.getElementById(id).value) * (1 - donationRate)
  if (value <= window.credit) {
    window[key] += value
    window[usd] += value
    window.credit -= value
    window.update()
  } else {
    window.alert("Credit not enough")
  }
}

window.sell = (id, key, usd, donation) => {
  const donationRate = Number(document.getElementById(donation).value) / 100
  const amount = Number(document.getElementById(id).value) * (1 - donationRate)
  const price = window[usd] / window[key]
  if (amount <= window[key]) {
    window[key] -= amount
    window[usd] -= price * amount
    window.credit += price * amount
    window.update()
  } else {
    window.alert("Balance not enough")
  }
}

window.update = () => {
  document.getElementById("credit").innerText = credit
  document.getElementById("balance1").innerText = balance1
  document.getElementById("balance2").innerText = balance2
  document.getElementById("usd1").innerText = usd1
  document.getElementById("usd2").innerText = usd2
  document.getElementById("profit1").innerText = profit1
  document.getElementById("profit2").innerText = profit2
}

setInterval(() => {
  if (balance1 > 0) {
    window.usd1 += usd1 * 0.1
    window.profit1 = 100 * (usd1 - balance1) / usd1
  }
  if (balance2 > 0) {
    window.usd2 += usd2 * 0.1
    window.profit2 = 100 * (usd2 - balance2) / usd2
  }
  update()
},1000)

window.credit = 1000
window.balance1 = 0
window.balance2 = 0
window.usd1 = 0
window.usd2 = 0
window.profit1 = 0
window.profit2 = 0
window.update()
getIdentity()

// document.querySelector("form").addEventListener("submit", async (e) => {
//   e.preventDefault();
//   const button = e.target.querySelector("button");
//
//   const name = document.getElementById("name").value.toString();
//
//   button.setAttribute("disabled", true);
//
//   // Interact with foo actor, calling the greet method
//   const greeting = await donainvest.greet(name);
//
//   button.removeAttribute("disabled");
//
//   document.getElementById("greeting").innerText = greeting;
//
//   return false;
// });
