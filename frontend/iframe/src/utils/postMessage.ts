import {WalletStatus} from "@terra-money/wallet-provider";
import {IframeMessageKind, WalletBridgeWindowEvent, WalletBridgeResponse} from "../types";

///// IFRAME MESSAGE HANDLING //////
//// IT IS ALL SETUP TO MATCH SERDE ON THE RUST SIDE /////

export function postWalletBridgeStatus(status: WalletStatus) {
  postIframeMsg(0, {kind: IframeMessageKind.WalletBridgeStatus, data: status });
}
export function postWalletBridgeWindowEvent(event: WalletBridgeWindowEvent) {
  postIframeMsg(0, {kind: IframeMessageKind.WalletBridgeWindowEvent, data: event});
}

export function postWalletBridgeResponse(bridge_id: number | undefined, resp: WalletBridgeResponse) {
  postIframeMsg(bridge_id, {kind: IframeMessageKind.WalletBridgeResponse, data: resp});
}

function postIframeMsg(bridge_id:number | undefined, msg: any) {

  const payload = [bridge_id ? bridge_id : 0, msg];

  console.log("FROM IFRAME:", payload);

  window.parent.postMessage(payload, "*");
}
