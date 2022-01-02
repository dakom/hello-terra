import {
  WalletProvider,
  useWallet,
  ConnectType,
  WalletStatus,
  Wallet as AutoWallet,
} from "@terra-money/wallet-provider";
import {
  MsgStoreCode,
  MsgInstantiateContract,
  MsgExecuteContract,
  MnemonicKey,
  Wallet as ManualWallet,
  LCDClient,
} from "@terra-money/terra.js";
import React, { useEffect, useState } from "react";
import {
  contractUpload,
  contractInstantiate,
  contractExecute,
  contractQuery,
} from "./utils/contract";
import {
  postContractInstantiate,
  postContractExecute,
  postStatus,
  postWalletInfo,
  postWindowEvent,
  postContractQuery,
  postContractUpload,
  postError,
} from "./utils/postMessage";
import { mainnet, TAG, walletConnectChainIds } from "./config";
import { WindowEvent, ContractExecuteMsg, MessageKind, SetupRequestKind, SetupRequestMsg, WalletBridgeMsgWrapper, ContractQueryMsg } from "./types";

import { WalletState, WalletKind } from "./wallet";
import { convertCoin } from "./utils/coin";

export function App() {
  return (
    <WalletProvider
      defaultNetwork={mainnet}
      walletConnectChainIds={walletConnectChainIds}
    >
      <WalletManager />
    </WalletProvider>
  );
}

function WalletManager() {
  const [manualWallet, setManualWallet] = useState<ManualWallet>();

  const autoWallet: AutoWallet = useWallet();
  const autoWallets = autoWallet.wallets;

  const walletStatus =
    manualWallet != null ? WalletStatus.WALLET_CONNECTED : autoWallet.status;


  useEffect(() => {
    const wallet =
      manualWallet != null
        ? new WalletState(manualWallet, manualWallet.lcd, WalletKind.Manual)
        : autoWallets.length > 0
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
    const withWallet = (f: (wallet: WalletState) => any) => {
      if (wallet != null) {
        return f(wallet);
      } else {
        postStatus(WalletStatus.WALLET_NOT_CONNECTED);
      }
    };

    const onMessage = (evt: any) => {
      //console.log("TO IFRAME:", evt.data);

      if (!Array.isArray(evt.data) || evt.data.length < 3) {
        return;
      }

      const [bridge_id, tag, msg]: WalletBridgeMsgWrapper = evt.data;

      if (tag !== TAG) {
        console.log("not meant for this iframe");
        return;
      }

      console.log("TO IFRAME:", msg);

      switch (msg.kind) {
        case MessageKind.SetupRequest:
          try {
            const setup_data = (msg as SetupRequestMsg).data;

            switch (setup_data.kind) {
              case SetupRequestKind.ConnectExtension:
                autoWallet.connect(ConnectType.EXTENSION);
                break;

              case SetupRequestKind.ConnectMobile:
                autoWallet.connect(ConnectType.EXTENSION);
                break;

              case SetupRequestKind.Install:
                autoWallet.install(ConnectType.EXTENSION);
                break;

              case SetupRequestKind.Disconnect:
                setManualWallet(undefined);
                autoWallet.disconnect();
                break;

              case SetupRequestKind.ConnectManual:
                const [key, host, chainId] = setup_data.data;

                const mk = new MnemonicKey({ mnemonic: key });

                const lcd = new LCDClient({
                  URL: host,
                  chainID: chainId,
                });

                setManualWallet(new ManualWallet(lcd, mk));
                break;

              case SetupRequestKind.WalletInfo:
                withWallet((wallet) => {
                  postWalletInfo(bridge_id, {
                    addr: wallet.addr,
                    network_name: wallet.lcd.config.URL,
                    chain_id: wallet.lcd.config.chainID,
                  })
                });
                break;

              default:
                console.log("other setup message:");
                console.log(msg);
                break;
            }
          } catch (e) {
            alert("not supported (do you have the extension installed?)");
            console.error(e);
          }
        break;

        case MessageKind.ContractUpload:
          withWallet((wallet) => {
            const outMsg = new MsgStoreCode(wallet.addr, msg.data);

            contractUpload(wallet, outMsg)
              .then((codeId: number) => {
                return postContractUpload(bridge_id, codeId)
              })
              .catch((error: unknown) => {
                postError(bridge_id, error);
              });
          });
        break;

        case MessageKind.ContractInstantiate:
          const { id } = msg.data; 

          console.log("ID:", id);

          withWallet((wallet) => {
            //TODO - add coin params...
            const outMsg = new MsgInstantiateContract(
              wallet.addr,
              "",
              id,
              msg.data.msg || {},
              {}
            );

            contractInstantiate(wallet, outMsg)
              .then((addr: string) => {
                postContractInstantiate(bridge_id, addr);
              })
              .catch((error: unknown) => {
                postError(bridge_id, error);
              });
          });
        break;

        case MessageKind.ContractExecute:
          withWallet((wallet) => {
            const coins = (msg as ContractExecuteMsg).data.coins?.map(convertCoin);
            //TODO - add coin params...
            const outMsg = new MsgExecuteContract(
              wallet.addr,
              (msg as ContractExecuteMsg).data.addr,
              (msg as ContractExecuteMsg).data.msg,
              coins
            );

            contractExecute(wallet, outMsg)
              .then((resp: any) => {
                postContractExecute(bridge_id, resp);
              })
              .catch((error: unknown) => {
                postError(bridge_id, error);
              });
          });
          break;

        case MessageKind.ContractQuery:
          withWallet((wallet) => {
            contractQuery(
              wallet, 
              (msg as ContractQueryMsg).data.addr, 
              (msg as ContractQueryMsg).data.msg, 
            )
              .then((resp:any) => {
                return postContractQuery(bridge_id, resp);
              })
              .catch((error: unknown) => {
                postError(bridge_id, error);
              });
          });
          break;

        default:
          break;
      }
    };

    const onClick = () => {
      postWindowEvent(WindowEvent.Click);
    };

    window.addEventListener("message", onMessage);
    window.addEventListener("click", onClick);

    return () => {
      window.removeEventListener("message", onMessage);
      window.removeEventListener("click", onClick);
    };
  }, [autoWallet, autoWallets, manualWallet]);

  useEffect(() => {
    postStatus(walletStatus);
  }, [walletStatus]);

  return <React.Fragment />;
}
