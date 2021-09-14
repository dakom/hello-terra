import { NetworkInfo, WalletProvider, useWallet, ConnectType} from '@terra-money/wallet-provider';
import { LCDClient} from '@terra-money/terra.js';
import React, {useEffect} from 'react';
import {contractUpload} from "./utils/contract";
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

      const lcd = wallets.length > 0 
        ? new LCDClient({
              URL: wallet.network.lcd,
              chainID: wallet.network.chainID,
          })
        : null;



      //Guards to make sure we're always working with a valid wallet
      const withWallet = (f: ((walletState:WalletState) => any)) => {
        if(lcd != null) {
           return f({lcd, wallet, addr: wallet.wallets[0].terraAddress});
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
            const {data, kind} = msg.data;
            switch(kind) {
              case WalletRequestKind.Addr: {
                withWallet(({addr}) => {
                  postWalletReponse({kind: WalletResponseKind.Addr, data: addr});
                });
              }
              break;

              case WalletRequestKind.ContractUpload: {
                withWallet((walletState) => {
                    contractUpload(walletState, data)
                        .then((codeId:string) => {
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

