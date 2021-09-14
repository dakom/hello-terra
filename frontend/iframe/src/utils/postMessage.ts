import {WalletStatus} from "@terra-money/wallet-provider";
import {IframeMessageKind, WalletWindowEvent, WalletResponse, IframeMsg} from "../types";

///// IFRAME MESSAGE HANDLING //////
//// IT IS ALL SETUP TO MATCH SERDE ON THE RUST SIDE /////

export function postWalletStatus(status: WalletStatus) {
  postIframeMsg({kind: IframeMessageKind.WalletStatus, data: status });
}
export function postWalletWindowEvent(event: WalletWindowEvent) {
  postIframeMsg({kind: IframeMessageKind.WalletWindow, data: event});
}

export function postWalletReponse(resp: WalletResponse) {
  postIframeMsg({kind: IframeMessageKind.WalletResponse, data: resp});
}

function postIframeMsg(msg: IframeMsg) {
  console.log(msg);
  window.parent.postMessage(msg, "*");
}
