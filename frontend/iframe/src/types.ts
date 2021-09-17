import { WalletStatus } from '@terra-money/wallet-provider';

//Top-level messages
type _IframeMsg = 
  WalletBridgeStatusMsg 
  | WalletBridgeWindowMsg
  | WalletBridgeRequestMsg
  | WalletBridgeResponseMsg
  | ContractInstantiate
  | ContractExecute

export type IframeMsg = [number | undefined, string, _IframeMsg];

export enum IframeMessageKind {
  WalletBridgeStatus = "wallet_bridge_status",
  WalletBridgeWindowEvent = "wallet_bridge_window_event",
  WalletBridgeRequest = "wallet_bridge_request",
  WalletBridgeResponse = "wallet_bridge_response",
  ContractInstantiate = "contract_instantiate",
  ContractExecute = "contract_execute",
  ContractQuery = "contract_query",
}

export type WalletBridgeStatusMsg = {
  kind: IframeMessageKind.WalletBridgeStatus, 
  data: WalletStatus
};

export type WalletBridgeWindowMsg = {
  kind: IframeMessageKind.WalletBridgeWindowEvent, 
  data: WalletBridgeWindowEvent
};

export type WalletBridgeRequestMsg = {
  kind: IframeMessageKind.WalletBridgeRequest, 
  data: WalletBridgeRequest 
};

export type WalletBridgeResponseMsg = {
  kind: IframeMessageKind.WalletBridgeResponse, 
  data: WalletBridgeResponse
};

export enum WalletBridgeWindowEvent {
  Click = "click",
}

/// WalletBridge Requests
export type WalletBridgeRequest = 
  WalletBridgeRequestSetup
  | WalletBridgeRequestWalletInfo
  | WalletBridgeRequestContractUpload

export type WalletBridgeRequestSetup = 
  { kind: WalletBridgeRequestKind.Setup, data: {kind: WalletBridgeSetupKind.ConnectExtension }}
  | { kind: WalletBridgeRequestKind.Setup, data: {kind: WalletBridgeSetupKind.ConnectMobile }}
  | { kind: WalletBridgeRequestKind.Setup, data: {kind: WalletBridgeSetupKind.ConnectManual, data: [string, string, string]}}
  | { kind: WalletBridgeRequestKind.Setup, data: {kind: WalletBridgeSetupKind.Install }}
  | { kind: WalletBridgeRequestKind.Setup, data: {kind: WalletBridgeSetupKind.Disconnect }};

export enum WalletBridgeSetupKind {
  ConnectExtension = "connect_extension",
  ConnectMobile = "connect_mobile",
  ConnectManual = "connect_manually",
  Install = "install",
  Disconnect = "disconnect",
}
export enum WalletBridgeRequestKind {
  Setup = "setup",
  WalletInfo = "wallet_info",
  ContractUpload = "contract_upload",
}

export type WalletBridgeRequestWalletInfo = {
  kind: WalletBridgeRequestKind.WalletInfo,
  data?: any
}

export type WalletBridgeRequestContractUpload = {
  kind: WalletBridgeRequestKind.ContractUpload,
  data: string 
}

/// WalletBridge Responses 
export type WalletBridgeResponse = 
  WalletBridgeResponseWalletInfo
  | WalletBridgeResponseContractUpload

export enum WalletBridgeResponseKind {
  WalletInfo = "wallet_info",
  ContractUpload = "contract_upload",
}

export type WalletBridgeResponseWalletInfo = {
  kind: WalletBridgeResponseKind.WalletInfo,
  data?: { addr: string, network_name: string, chain_id: string}
}

export type WalletBridgeResponseContractUpload = {
  kind: WalletBridgeResponseKind.ContractUpload,
  data?: number 
}

export type ContractInstantiate = {
  kind: IframeMessageKind.ContractInstantiate, 
  data: {
    id: number,
    msg?: any
  } 
};

export type ContractExecute = {
  kind: IframeMessageKind.ContractExecute, 
  data: {
    addr: string,
    msg: any,
  } 
};