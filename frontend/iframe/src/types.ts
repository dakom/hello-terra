import { WalletStatus } from '@terra-money/wallet-provider';

//Top-level messages
export type IframeMsg = 
  WalletStatusMsg 
  | WalletSetupMsg
  | WalletWindowMsg
  | WalletRequestMsg
  | WalletResponseMsg;

export enum IframeMessageKind {
  WalletStatus = "wallet_status",
  WalletSetup = "wallet_setup",
  WalletWindow = "wallet_window",
  WalletRequest = "wallet_request",
  WalletResponse = "wallet_response",
}

export type WalletStatusMsg = {
  kind: IframeMessageKind.WalletStatus, 
  data: WalletStatus
};

export type WalletSetupMsg = {
  kind: IframeMessageKind.WalletSetup, 
  data: WalletSetup
};

export type WalletWindowMsg = {
  kind: IframeMessageKind.WalletWindow, 
  data: WalletWindowEvent
};

export type WalletRequestMsg = {
  kind: IframeMessageKind.WalletRequest, 
  data: WalletRequest 
};

export type WalletResponseMsg = {
  kind: IframeMessageKind.WalletResponse, 
  data: WalletResponse
};
export type WalletSetup = 
  { kind: WalletSetupKind.ConnectExtension }
  | { kind: WalletSetupKind.ConnectMobile }  
  | { kind: WalletSetupKind.ConnectManual, data: [string, string, string]}
  | { kind: WalletSetupKind.Install }  
  | { kind: WalletSetupKind.Disconnect };

export enum WalletSetupKind {
  ConnectExtension = "connect_extension",
  ConnectMobile = "connect_mobile",
  ConnectManual = "connect_manually",
  Install = "install",
  Disconnect = "disconnect",
}

export enum WalletWindowEvent {
  Click = "click",
}

/// Wallet Requests
export type WalletRequest = 
  WalletRequestAddr
  | WalletRequestContractUpload
  | WalletRequestContractInstantiate
  | WalletRequestContractExecute;

export enum WalletRequestKind {
  Addr = "addr",
  ContractUpload = "contract_upload",
  ContractInstantiate = "contract_instantiate",
  ContractExecute = "contract_execute",
}

export type WalletRequestAddr = {
  kind: WalletRequestKind.Addr,
  data?: any
}

export type WalletRequestContractUpload = {
  kind: WalletRequestKind.ContractUpload,
  data: string 
}

export type WalletRequestContractInstantiate = {
  kind: WalletRequestKind.ContractInstantiate,
  data: {
    id: number
  } 
}


export type WalletRequestContractExecute = {
  kind: WalletRequestKind.ContractExecute,
  data: {
    addr: string,
    //coins:? Coins, 
    msg: any 
  } 
}
/// Wallet Responses 
export type WalletResponse = 
  WalletResponseAddr
  | WalletResponseContractUpload
  | WalletResponseContractInstantiate
  | WalletResponseContractExecute;

export enum WalletResponseKind {
  Addr = "addr",
  ContractUpload = "contract_upload",
  ContractInstantiate = "contract_instantiate",
  ContractExecute = "contract_execute",
}

export type WalletResponseAddr = {
  kind: WalletResponseKind.Addr,
  data?: { addr: string, network_name: string, chain_id: string}
}

export type WalletResponseContractUpload = {
  kind: WalletResponseKind.ContractUpload,
  data?: number 
}

export type WalletResponseContractInstantiate = {
  kind: WalletResponseKind.ContractInstantiate,
  data?: string
}
export type WalletResponseContractExecute = {
  kind: WalletResponseKind.ContractExecute,
  data?: string
}