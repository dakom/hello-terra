import { WalletProvider, useWallet, ConnectType, WalletStatus, Wallet as AutoWallet} from '@terra-money/wallet-provider';
import { MsgStoreCode, MsgInstantiateContract, MsgExecuteContract, MnemonicKey, Wallet as ManualWallet, LCDClient} from '@terra-money/terra.js';
import React, {useEffect, useState} from 'react';
import {contractUpload, contractInstantiate, contractExecute} from "./utils/contract";
import {postWalletBridgeResponse, postWalletBridgeStatus, postWalletBridgeWindowEvent} from "./utils/postMessage";
import {mainnet, TAG, walletConnectChainIds} from "./config";
import {
  IframeMsg,
  IframeMessageKind,
  WalletBridgeRequestKind,
  WalletBridgeResponseKind,
  WalletBridgeSetupKind,
  WalletBridgeWindowEvent,
  WalletBridgeRequestContractInstantiate,
  WalletBridgeRequestContractUpload,
  WalletBridgeRequestContractExecute,
  WalletBridgeRequestSetup,
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
          postWalletBridgeStatus(WalletStatus.WALLET_NOT_CONNECTED);
        }
      }

      const onMessage = (evt:any) => {
        console.log("TO IFRAME:", evt.data);

        if(!Array.isArray(evt.data) || evt.data.length < 3) {
          return;
        }

        const [bridge_id, tag, msg]:IframeMsg = evt.data;

        if(tag !== TAG) {
          console.log("not meant for this iframe");
          return;
        }

        switch(msg.kind) {

          case IframeMessageKind.WalletBridgeRequest:
            switch(msg.data.kind) {

              case WalletBridgeRequestKind.Setup:
                  try {
                    const setup_data = (msg.data as WalletBridgeRequestSetup).data;

                    switch(setup_data.kind) {
                        case WalletBridgeSetupKind.ConnectExtension:
                            autoWallet.connect(ConnectType.CHROME_EXTENSION); 
                            break;

                        case WalletBridgeSetupKind.ConnectMobile:
                            autoWallet.connect(ConnectType.WALLETCONNECT); 
                            break;

                        case WalletBridgeSetupKind.Install:
                            autoWallet.install(ConnectType.CHROME_EXTENSION); 
                            break;

                        case WalletBridgeSetupKind.Disconnect:
                            setManualWallet(undefined);
                            autoWallet.disconnect();
                            break;

                        case WalletBridgeSetupKind.ConnectManual:
                            const [key, host, chainId] = setup_data.data; 

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

              case WalletBridgeRequestKind.WalletInfo:
                withWallet((wallet) => {
                  postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.WalletInfo, data: {addr: wallet.addr, network_name: wallet.lcd.config.URL, chain_id: wallet.lcd.config.chainID}});
                });
                break;

              case WalletBridgeRequestKind.ContractUpload:
                withWallet((wallet) => {
                    console.log(wallet.addr);
                    const outMsg = new MsgStoreCode(wallet.addr, (msg.data as WalletBridgeRequestContractUpload).data);
                    console.log(outMsg);

                    contractUpload(wallet, outMsg)
                        .then((codeId:number) => {
                                postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.ContractUpload, data: codeId});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.ContractUpload});
                        });
                });
                break;

              case WalletBridgeRequestKind.ContractInstantiate:
                const {data: {id}}:WalletBridgeRequestContractInstantiate = msg.data;

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
                                postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.ContractInstantiate, data: addr});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.ContractInstantiate});
                        });
                });
                break;

              case WalletBridgeRequestKind.ContractExecute:
                withWallet((wallet) => {
                  //TODO - add coin params...
                  const outMsg = new MsgExecuteContract(
                    wallet.addr,
                    (msg.data as WalletBridgeRequestContractExecute).data.addr,
                    (msg.data as WalletBridgeRequestContractExecute).data.msg
                  );

                  contractExecute(wallet, outMsg)
                        .then((resp:any) => {
                                postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.ContractExecute, data: resp});
                        })
                        .catch((error: unknown) => {
                            console.error("GOT ERROR:");
                            console.error(error);
                            postWalletBridgeResponse(bridge_id, {kind: WalletBridgeResponseKind.ContractExecute});
                        });
                });
                break;

              default:
                console.log("unhandled request:");
                console.log(msg);
                break;
            }
          break;

          case IframeMessageKind.WalletBridgeStatus:
            console.warn("weird! child received status message...");
            break;

          case IframeMessageKind.WalletBridgeResponse:
            console.warn("weird! child received response message...");
            break;

          default: break;
        }
      };
      
      const onClick = () => {
        postWalletBridgeWindowEvent(WalletBridgeWindowEvent.Click);
      };

      window.addEventListener("message", onMessage);
      window.addEventListener("click", onClick);

      return () => {
        window.removeEventListener("message", onMessage);
        window.removeEventListener("click", onClick);
      }
    }, [autoWallet, autoWallets, manualWallet]);

    useEffect(() => {

      postWalletBridgeStatus(walletStatus);
    }, [walletStatus]);

    return <React.Fragment />;
}

