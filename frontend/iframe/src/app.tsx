import { WalletProvider, useWallet, ConnectType, WalletStatus, Wallet as AutoWallet} from '@terra-money/wallet-provider';
import { MsgStoreCode, MsgInstantiateContract, MsgExecuteContract, MnemonicKey, Wallet as ManualWallet, LCDClient} from '@terra-money/terra.js';
import React, {useEffect, useState} from 'react';
import {contractUpload, contractInstantiate, contractExecute} from "./utils/contract";
import {postWalletReponse, postWalletStatus, postWalletWindowEvent} from "./utils/postMessage";
import {mainnet, walletConnectChainIds} from "./config";
import {
  IframeMsg,
  IframeMessageKind,
  WalletRequestKind,
  WalletResponseKind,
  WalletSetupKind,
  WalletWindowEvent,
  WalletRequestContractInstantiate,
} from "./types";

import {
  WalletState,
  WalletKind,
} from "./wallet";

export function App() {
  return (
    <WalletProvider defaultNetwork={mainnet} walletConnectChainIds={walletConnectChainIds}>
      <WalletManager />
    </WalletProvider>
  );
}

function WalletManager() {
    const [manualWallet, setManualWallet] = useState<ManualWallet>();

    const autoWallet:AutoWallet = useWallet();
    const autoWallets = autoWallet.wallets;

    const walletStatus = (manualWallet != null)
      ? WalletStatus.WALLET_CONNECTED
      : autoWallet.status;

    useEffect(() => {

      const wallet = (manualWallet != null)
        ? new WalletState(
            manualWallet,
            manualWallet.lcd,
            WalletKind.Manual,
        )
        :  autoWallets.length > 0 
            ? new WalletState(
                  autoWallet,
                  new LCDClient({
                    URL: autoWallet.network.lcd,
                    chainID: autoWallet.network.chainID,
                  }),
                  WalletKind.Auto
              )
            : null;



      //Guards to make sure we're always working with a valid wallet
      const withWallet = (f: ((wallet:WalletState) => any)) => {
        if(wallet != null) {
           return f(wallet);
        } else {
          //Will always effectively logout (i.e. response to "get id" without an id
          postWalletReponse({kind: WalletResponseKind.Addr});
        }

      }

      const onMessage = (evt:any) => {
        const msg:IframeMsg = evt.data;

        switch(msg.kind) {
          case IframeMessageKind.WalletSetup:
              try {
                switch(msg.data.kind) {
                    case WalletSetupKind.ConnectExtension:
                        autoWallet.connect(ConnectType.CHROME_EXTENSION); 
                        break;

                    case WalletSetupKind.ConnectMobile:
                        autoWallet.connect(ConnectType.WALLETCONNECT); 
                        break;

                    case WalletSetupKind.Install:
                        autoWallet.install(ConnectType.CHROME_EXTENSION); 
                        break;

                    case WalletSetupKind.Disconnect:
                        autoWallet.disconnect();
                        setManualWallet(undefined);
                        break;

                    case WalletSetupKind.ConnectManual:
                        const [key, host, chainId] = msg.data.data;

                        const mk = new MnemonicKey({ mnemonic: key});
                        
                        const lcd = new LCDClient({
                          URL: host,
                          chainID: chainId 
                        });

                        setManualWallet(new ManualWallet(lcd, mk));
                        break;

                    default: 
                        console.log("other setup message:");
                        console.log(msg);
                      break;
                }
            } catch(e) {
                alert("not supported (do you have the extension installed?)");
                console.error(e);
            }
            break;

          case IframeMessageKind.WalletRequest:
            switch(msg.data.kind) {
              case WalletRequestKind.Addr:
                withWallet((wallet) => {
                  postWalletReponse({kind: WalletResponseKind.Addr, data: {addr: wallet.addr, network_name: wallet.lcd.config.URL, chain_id: wallet.lcd.config.chainID}});
                });
                break;

              case WalletRequestKind.ContractUpload:
                withWallet((wallet) => {
                    const outMsg = new MsgStoreCode(wallet.addr, msg.data.data);
                    contractUpload(wallet, outMsg)
                        .then((codeId:number) => {
                                postWalletReponse({kind: WalletResponseKind.ContractUpload, data: codeId});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletReponse({kind: WalletResponseKind.ContractUpload});
                        });
                });
                break;

              case WalletRequestKind.ContractInstantiate:
                const {data: {id}}:WalletRequestContractInstantiate = msg.data;

                withWallet((wallet) => {
                  //TODO - add coin params...
                  const outMsg = new MsgInstantiateContract(
                    wallet.addr,
                    "",
                    id,
                    {},
                    {}
                  ); 

                  contractInstantiate(wallet, outMsg)
                        .then((addr:string) => {
                                postWalletReponse({kind: WalletResponseKind.ContractInstantiate, data: addr});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletReponse({kind: WalletResponseKind.ContractInstantiate});
                        });
                });
                break;

              case WalletRequestKind.ContractExecute:
                withWallet((wallet) => {
                  //TODO - add coin params...
                  const outMsg = new MsgExecuteContract(
                    wallet.addr,
                    msg.data.data.addr,
                    msg.data.data.msg
                  );

                  contractExecute(wallet, outMsg)
                        .then((resp:any) => {
                                postWalletReponse({kind: WalletResponseKind.ContractExecute, data: resp});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletReponse({kind: WalletResponseKind.ContractExecute});
                        });
                });
                break;

              default:
                console.log("unhandled request:");
                console.log(msg);
                break;
            }
          break;

          case IframeMessageKind.WalletStatus:
            console.warn("weird! child received status message...");
            break;

          case IframeMessageKind.WalletResponse:
            console.warn("weird! child received response message...");
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
    }, [autoWallet, autoWallets, manualWallet]);

    useEffect(() => {

      postWalletStatus(walletStatus);
    }, [walletStatus]);

    return <React.Fragment />;
}

