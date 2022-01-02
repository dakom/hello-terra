import {WalletStatus} from "@terra-money/wallet-provider";
import { WindowEvent, WalletInfoResponse } from "../types";
import {TAG} from "../config";

///// IFRAME MESSAGE HANDLING //////
//// IT IS ALL SETUP TO MATCH SERDE ON THE RUST SIDE /////

export function postStatus(status: WalletStatus) {
  postIframeMsg(0, {kind: "status", data: status });
}
export function postWindowEvent(event: WindowEvent) {
  postIframeMsg(0, {kind: "window_event", data: event});
}

export function postWalletInfo(bridge_id: number | undefined, resp?: WalletInfoResponse) {
  postIframeMsg(bridge_id, resp);
}
export function postContractUpload(bridge_id: number | undefined, id?: number) {
  postIframeMsg(bridge_id, id);
}

export function postContractInstantiate(bridge_id: number | undefined, addr?: string) {
  postIframeMsg(bridge_id, addr);
}

export function postContractExecute(bridge_id: number | undefined, data?: any) {
  postIframeMsg(bridge_id, data);
}

export function postContractQuery(bridge_id: number | undefined, data?: any) {
  postIframeMsg(bridge_id, data);
}


function postIframeMsg(bridge_id:number | undefined, msg: any) {

  const payload = [bridge_id ? bridge_id : 0, TAG, {Ok: msg}]

  //console.log("FROM IFRAME:", payload);

  window.parent.postMessage(payload, "*");
}

export function postError(bridge_id:number | undefined, msg: any) {

  const payload = [bridge_id ? bridge_id : 0, TAG, {Err: msg ? JSON.stringify(msg) : ""}];

  //console.log("FROM IFRAME:", payload);

  window.parent.postMessage(payload, "*");
}
