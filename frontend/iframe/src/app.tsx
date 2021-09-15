import { NetworkInfo, WalletProvider, useWallet, ConnectType} from '@terra-money/wallet-provider';
import { LCDClient, MsgStoreCode, MsgInstantiateContract, MsgExecuteContract} from '@terra-money/terra.js';
import React, {useEffect} from 'react';
import {contractUpload, contractInstantiate, contractExecute} from "./utils/contract";
import {postWalletReponse, postWalletStatus, postWalletWindowEvent} from "./utils/postMessage";
import {mainnet, walletConnectChainIds} from "./config";
import {
  WalletState,
  IframeMsg,
  IframeMessageKind,
  WalletRequestKind,
  WalletResponseKind,
  WalletSetup,
  WalletWindowEvent,
  WalletRequestContractInstantiate,
  WalletRequestContractExecute,
} from "./types";


export function App() {
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

      const _lcd = wallets.length > 0 
        ? new LCDClient({
              URL: wallet.network.lcd,
              chainID: wallet.network.chainID,
          })
        : null;



      //Guards to make sure we're always working with a valid wallet
      const withWallet = (f: ((walletState:WalletState) => any)) => {
        if(_lcd != null) {
           return f({lcd: _lcd, wallet, addr: wallet.wallets[0].terraAddress});
        } else {
          //Will always effectively logout (i.e. response to "get id" without an id
          postWalletReponse({kind: WalletResponseKind.Addr});
        }

      }

      const onMessage = (evt:any) => {
        const msg:IframeMsg = evt.data;

        switch(msg.kind) {
          case IframeMessageKind.WalletSetup: {
              try {
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
            } catch(e) {
                alert("not supported (do you have the extension installed?)");
                console.error(e);
            }
          }
          break;

          case IframeMessageKind.WalletRequest: {
            switch(msg.data.kind) {
              case WalletRequestKind.Addr: {
                withWallet(({wallet, addr}) => {
                  postWalletReponse({kind: WalletResponseKind.Addr, data: {addr, network_name: wallet.network.name, chain_id: wallet.network.chainID}});
                });
              }
              break;

              case WalletRequestKind.ContractUpload: {
                withWallet((walletState) => {
                    const outMsg = new MsgStoreCode(walletState.addr, msg.data.data);
                    contractUpload(walletState, outMsg)
                        .then((codeId:number) => {
                                postWalletReponse({kind: WalletResponseKind.ContractUpload, data: codeId});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletReponse({kind: WalletResponseKind.ContractUpload});
                        });
                });
              }
              break;
              case WalletRequestKind.ContractInstantiate: {
                const {data: {id}}:WalletRequestContractInstantiate = msg.data;

                withWallet((walletState) => {
                  //TODO - add coin params...
                  const outMsg = new MsgInstantiateContract(
                    walletState.addr,
                    "",
                    id,
                    {},
                    {}
                  ); 

                  contractInstantiate(walletState, outMsg)
                        .then((addr:string) => {
                                postWalletReponse({kind: WalletResponseKind.ContractInstantiate, data: addr});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletReponse({kind: WalletResponseKind.ContractInstantiate});
                        });
                });
              }
              break;

              case WalletRequestKind.ContractExecute: {
                withWallet((walletState) => {
                  //TODO - add coin params...
                  const outMsg = new MsgExecuteContract(
                    walletState.addr,
                    msg.data.data.addr,
                    msg.data.data.msg
                  );

                  contractExecute(walletState, outMsg)
                        .then((resp:any) => {
                                postWalletReponse({kind: WalletResponseKind.ContractExecute, data: resp});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletReponse({kind: WalletResponseKind.ContractExecute});
                        });
                });

              }
              break;

              default: {
                console.log("unhandled request:");
                console.log(msg);
              }break;
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
    }, [wallet, wallets]);

    useEffect(() => {
      postWalletStatus(status);
    }, [status]);

    return <React.Fragment />;
}

