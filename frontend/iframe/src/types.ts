export type WalletBridgeMsgWrapper = [number | undefined, string, _WalletBridgeMsgWrapper];

//Top-level messages
type _WalletBridgeMsgWrapper =
  | SetupRequestMsg 
  | ContractInstantiateMsg
  | ContractExecuteMsg
  | ContractUploadMsg
  | ContractQueryMsg;


export enum MessageKind {
  SetupRequest = "setup_request",
  ContractInstantiate = "contract_instantiate",
  ContractExecute = "contract_execute",
  ContractQuery = "contract_query",
  ContractUpload = "contract_upload",
}


export enum WindowEvent {
  Click = "click",
}

export type SetupRequestMsg = {
  kind: MessageKind.SetupRequest 
  data: SetupRequest 
};

export type SetupRequest = 
  { kind: SetupRequestKind.ConnectExtension }
  | { kind: SetupRequestKind.ConnectMobile }
  | { kind: SetupRequestKind.ConnectManual, data: [string, string, string]}
  | { kind: SetupRequestKind.Install }
  | { kind: SetupRequestKind.Disconnect }
  | { kind: SetupRequestKind.WalletInfo };

export enum SetupRequestKind {
  ConnectExtension = "connect_extension",
  ConnectMobile = "connect_mobile",
  Install = "install",
  Disconnect = "disconnect",
  ConnectManual = "connect_manually",
  WalletInfo = "wallet_info"
}

export type WalletInfoResponse = { addr: string, network_name: string, chain_id: string};

export type ContractUploadMsg = {
  kind: MessageKind.ContractUpload
  data: string 
}

export type ContractInstantiateMsg = {
  kind: MessageKind.ContractInstantiate
  data: {
    id: number,
    msg?: any
  } 
}

export type ContractExecuteMsg = {
  kind: MessageKind.ContractExecute
  data: {
    addr: string,
    msg: any,
    coins?: any
  } 
}

export type ContractQueryMsg = {
  kind: MessageKind.ContractQuery
  data: {
    addr: string,
    msg: any,
  } 
}