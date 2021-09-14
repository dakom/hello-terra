import { NetworkInfo, WalletProvider, useWallet, WalletStatus, ConnectType, Wallet } from '@terra-money/wallet-provider';
import { MsgStoreCode, LCDClient} from '@terra-money/terra.js';
import { ConnectSample } from 'components/ConnectSample';
import { QuerySample } from 'components/QuerySample';
import { TxSample } from 'components/TxSample';
import React, {useEffect, useCallback} from 'react';
import ReactDOM from 'react-dom';
import './style.css';

const mainnet = {
  name: 'mainnet',
  chainID: 'columbus-4',
  lcd: 'https://lcd.terra.dev',
};

const testnet = {
  name: 'testnet',
  chainID: 'tequila-0004',
  lcd: 'https://tequila-lcd.terra.dev',
};

const walletConnectChainIds: Record<number, NetworkInfo> = {
  0: testnet,
  1: mainnet,
};

function App() {
  return (
    <WalletProvider defaultNetwork={mainnet} walletConnectChainIds={walletConnectChainIds}>
      <WalletManager />
    </WalletProvider>
  );
}

function WalletManager() {
    const wallet = useWallet();
    const {
      status,
      network,
      wallets,
      availableConnectTypes,
      availableInstallTypes,
      connect,
      install,
      disconnect,
    } = wallet;

    useEffect(() => {

      const lcd = wallets.length > 0 
        ? new LCDClient({
              URL: wallet.network.lcd,
              chainID: wallet.network.chainID,
          })
        : null;



      //Guards to make sure we're always working with a valid wallet
      const withWallet = (f: ((lcd: LCDClient, wallet:Wallet, id: string) => any)) => {
        if(lcd != null) {
           return f(lcd, wallet, wallet.wallets[0].terraAddress);
        } else {
          //Will always effectively logout (i.e. response to "get id" without an id
          postWalletReponse({kind: WalletResponseKind.Id});
        }

      }

      const onMessage = (evt:any) => {
        const msg:IframeMsg = evt.data;

        switch(msg.kind) {
          case IframeMessageKind.WalletSetup: {
            switch(msg.data) {
              case WalletSetup.ConnectExtension: {
                connect(ConnectType.CHROME_EXTENSION); 
              }
              break;

              case WalletSetup.ConnectMobile: {
                connect(ConnectType.WALLETCONNECT); 
              }
              break;

              case WalletSetup.Install: {
                install(ConnectType.CHROME_EXTENSION); 
              }
              break;

              case WalletSetup.Disconnect: {
                disconnect();
              }
              break;

              default: break;
            }
          }
          break;

          case IframeMessageKind.WalletRequest: {
            const {data, kind} = msg.data;
            switch(kind) {
              case WalletRequestKind.Id: {
                withWallet((_lcd, _wallet, id) => {
                  postWalletReponse({kind: WalletResponseKind.Id, data: id});
                });
              }
              break;

              case WalletRequestKind.ContractUpload: {
                withWallet((lcd, wallet, id) => {
                  const bytes:string = data;

                  const storeCode = new MsgStoreCode(id, bytes);
                  wallet.post({ msgs: [storeCode] })
                    .then(res => {
                      if(!res.success) {
                        return Promise.reject(res);
                      } else {
                        //TODO - remove delay: https://github.com/terra-money/wallet-provider/issues/23#issuecomment-918725271
                        return new Promise(resolve => {
                          setTimeout(() => {
                            lcd.tx.txInfo(res.result.txhash).then(resolve);
                          }, 7000);
                        });
                      }
                    })
                    .then((res:any) => {
                      const {
                        store_code: { code_id },
                      } = res.logs[0].eventsByType;

                      postWalletReponse({kind: WalletResponseKind.ContractUpload, data: code_id[0]});
                    })
                    .catch((error: unknown) => {
                      console.error("GOT ERROR:");
                      console.error(error);
                      postWalletReponse({kind: WalletResponseKind.ContractUpload});
                    });
                });
              }
              break;

              default: break;
            }
          }
          break;

          case IframeMessageKind.WalletStatus: {
            console.warn("weird! child received status message...");
          }
          break;

          case IframeMessageKind.WalletResponse: {
            console.warn("weird! child received response message...");
          }
          break;

          default: break;
        }
      };
      
      const onClick = () => {
        postWalletWindowEvent(WalletWindowEvent.Click);
      };

      window.addEventListener("message", onMessage);
      window.addEventListener("click", onClick);

      return () => {
        window.removeEventListener("message", onMessage);
        window.removeEventListener("click", onClick);
      }
    }, [wallets]);

    useEffect(() => {
      postWalletStatus(status);
    }, [status]);

    return <React.Fragment />;
}

ReactDOM.render(<App />, document.getElementById('root'));

///// IFRAME MESSAGE HANDLING //////
//// IT IS ALL SETUP TO MATCH SERDE ON THE RUST SIDE /////

function postWalletStatus(status: WalletStatus) {
  postIframeMsg({kind: IframeMessageKind.WalletStatus, data: status });
}
function postWalletWindowEvent(event: WalletWindowEvent) {
  postIframeMsg({kind: IframeMessageKind.WalletWindow, data: event});
}

function postWalletReponse(resp: WalletResponse) {
  postIframeMsg({kind: IframeMessageKind.WalletResponse, data: resp});
}

function postIframeMsg(msg: IframeMsg) {
  console.log(msg);
  window.parent.postMessage(msg, "*");
}

//Top-level messages
type IframeMsg = 
  WalletStatusMsg 
  | WalletSetupMsg
  | WalletWindowMsg
  | WalletRequestMsg
  | WalletResponseMsg;

enum IframeMessageKind {
  WalletStatus = "wallet_status",
  WalletSetup = "wallet_setup",
  WalletWindow = "wallet_window",
  WalletRequest = "wallet_request",
  WalletResponse = "wallet_response",
}

type WalletStatusMsg = {
  kind: IframeMessageKind.WalletStatus, 
  data: WalletStatus
};

type WalletSetupMsg = {
  kind: IframeMessageKind.WalletSetup, 
  data: WalletSetup
};

type WalletWindowMsg = {
  kind: IframeMessageKind.WalletWindow, 
  data: WalletWindowEvent
};

type WalletRequestMsg = {
  kind: IframeMessageKind.WalletRequest, 
  data: WalletRequest 
};

type WalletResponseMsg = {
  kind: IframeMessageKind.WalletResponse, 
  data: WalletResponse
};
enum WalletSetup {
  ConnectExtension = "connect_extension",
  ConnectMobile = "connect_mobile",
  Install = "install",
  Disconnect = "disconnect",
}

enum WalletWindowEvent {
  Click = "click",
}

/// Wallet Requests
type WalletRequest = 
  WalletRequestId
  | WalletRequestContractUpload;

enum WalletRequestKind {
  Id = "id",
  ContractUpload = "contract_upload",
}

type WalletRequestId = {
  kind: WalletRequestKind.Id,
  data?: any
}

type WalletRequestContractUpload = {
  kind: WalletRequestKind.ContractUpload,
  data: string 
}
/// Wallet Responses 
type WalletResponse = 
  WalletResponseId
  | WalletResponseContractUpload;

enum WalletResponseKind {
  Id = "id",
  ContractUpload = "contract_upload"
}

type WalletResponseId = {
  kind: WalletResponseKind.Id,
  data?: string
}

type WalletResponseContractUpload = {
  kind: WalletResponseKind.ContractUpload,
  data?: string
}