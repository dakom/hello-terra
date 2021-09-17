import { WalletStatus } from '@terra-money/wallet-provider';

//Top-level messages
type _IframeMsg = 
  WalletBridgeStatusMsg 
  | WalletBridgeWindowMsg
  | WalletBridgeRequestMsg
  | WalletBridgeResponseMsg;

export type IframeMsg = [number | undefined, string, _IframeMsg];

export enum IframeMessageKind {
  WalletBridgeStatus = "wallet_bridge_status",
  WalletBridgeWindowEvent = "wallet_bridge_window_event",
  WalletBridgeRequest = "wallet_bridge_request",
  WalletBridgeResponse = "wallet_bridge_response",
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
  | WalletBridgeRequestContractInstantiate
  | WalletBridgeRequestContractExecute;

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
  ContractInstantiate = "contract_instantiate",
  ContractExecute = "contract_execute",
}

export type WalletBridgeRequestWalletInfo = {
  kind: WalletBridgeRequestKind.WalletInfo,
  data?: any
}

export type WalletBridgeRequestContractUpload = {
  kind: WalletBridgeRequestKind.ContractUpload,
  data: string 
}

export type WalletBridgeRequestContractInstantiate = {
  kind: WalletBridgeRequestKind.ContractInstantiate,
  data: {
    id: number
  } 
}


export type WalletBridgeRequestContractExecute = {
  kind: WalletBridgeRequestKind.ContractExecute,
  data: {
    addr: string,
    //coins:? Coins, 
    msg: any 
  } 
}
/// WalletBridge Responses 
export type WalletBridgeResponse = 
  WalletBridgeResponseWalletInfo
  | WalletBridgeResponseContractUpload
  | WalletBridgeResponseContractInstantiate
  | WalletBridgeResponseContractExecute;

export enum WalletBridgeResponseKind {
  WalletInfo = "wallet_info",
  ContractUpload = "contract_upload",
  ContractInstantiate = "contract_instantiate",
  ContractExecute = "contract_execute",
}

export type WalletBridgeResponseWalletInfo = {
  kind: WalletBridgeResponseKind.WalletInfo,
  data?: { addr: string, network_name: string, chain_id: string}
}

export type WalletBridgeResponseContractUpload = {
  kind: WalletBridgeResponseKind.ContractUpload,
  data?: number 
}

export type WalletBridgeResponseContractInstantiate = {
  kind: WalletBridgeResponseKind.ContractInstantiate,
  data?: string
}
export type WalletBridgeResponseContractExecute = {
  kind: WalletBridgeResponseKind.ContractExecute,
  data?: string
}